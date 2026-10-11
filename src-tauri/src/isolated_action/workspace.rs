//! Host file access is restricted to an owner-selected directory descriptor.
use super::{hash, text, ActionError, Evidence, Result};
use rustix::fs::{openat, unlinkat, AtFlags, Mode, OFlags};
use std::os::unix::fs::MetadataExt;
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

// Only successful native decision resolution can construct this effect permit.
// Consuming it does not skip final target, evidence, expiry or journal checks.
pub(crate) struct ApplyGrant {
    review: String,
    baseline: String,
    deadline: std::time::Instant,
}
pub(crate) struct Workspace {
    root: PathBuf,
    directory: File,
    pub(crate) area: PathBuf,
    recovery_directory: File,
    pub(crate) file: String,
    pub(crate) test_file: String,
    original: Option<Vec<u8>>,
    test: Vec<u8>,
    baseline: String,
}
fn read_at(root: &File, name: &str) -> Result<Option<Vec<u8>>> {
    read_mode(root, name, 0o644)
}
fn read_mode(root: &File, name: &str, mode: u32) -> Result<Option<Vec<u8>>> {
    let fd = match openat(
        root,
        name,
        OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::NONBLOCK,
        Mode::empty(),
    ) {
        Ok(fd) => fd,
        Err(rustix::io::Errno::NOENT) => return Ok(None),
        Err(_) => return Err(ActionError::Scope),
    };
    let mut f = File::from(fd);
    let m = f.metadata().map_err(|_| ActionError::Storage)?;
    if !m.is_file() || m.nlink() != 1 || m.len() > 8192 || m.mode() & 0o777 != mode {
        return Err(ActionError::Scope);
    }
    let mut bytes = Vec::new();
    Read::by_ref(&mut f)
        .take(8193)
        .read_to_end(&mut bytes)
        .map_err(|_| ActionError::Storage)?;
    text(&bytes)?;
    Ok(Some(bytes))
}
fn git(root: &Path, args: &[&str]) -> Result<String> {
    let mut c = Command::new("/usr/bin/git");
    c.env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_TERMINAL_PROMPT", "0")
        .args([
            "--no-optional-locks",
            "-c",
            "core.fsmonitor=false",
            "-c",
            "core.hooksPath=/dev/null",
            "-c",
            "core.untrackedCache=false",
            "-c",
            "core.pager=cat",
        ])
        .current_dir(root)
        .args(args);
    let result = super::sandbox::host_command(c, std::time::Duration::from_secs(5), 32768)?;
    if result.0 != Some(0) {
        return Err(ActionError::Target);
    }
    String::from_utf8(result.1).map_err(|_| ActionError::Target)
}
fn fingerprint(root: &Path, directory: &File, owned_temp: Option<&str>) -> Result<String> {
    // Avoid status/diff on untrusted working files: those can invoke configured
    // Git clean filters. Compare index/tree/raw file bytes using read-only commands.
    let head = git(root, &["rev-parse", "--verify", "HEAD"])?;
    let staged = git(root, &["ls-files", "--stage", "-z"])?;
    let tree = git(root, &["ls-tree", "-z", "HEAD"])?;
    let mut expected = BTreeMap::new();
    for row in tree.split('\0').filter(|s| !s.is_empty()) {
        let (meta, name) = row.split_once('\t').ok_or(ActionError::Target)?;
        let bits = meta.split_whitespace().collect::<Vec<_>>();
        if bits.len() != 3 || bits[0] != "100644" || bits[1] != "blob" {
            return Err(ActionError::Scope);
        }
        expected.insert(name.to_owned(), bits[2].to_owned());
    }
    let mut contents = BTreeMap::new();
    for row in staged.split('\0').filter(|s| !s.is_empty()) {
        let (meta, name) = row.split_once('\t').ok_or(ActionError::Target)?;
        let bits = meta.split_whitespace().collect::<Vec<_>>();
        if bits.len() != 3
            || bits[0] != "100644"
            || bits[2] != "0"
            || expected.get(name).map(String::as_str) != Some(bits[1])
        {
            return Err(ActionError::Drift);
        }
        if contents.len() >= 32
            || name.contains('/')
            || name.starts_with('.')
            || name.contains("..")
            || !name
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b))
        {
            return Err(ActionError::Scope);
        }
        let bytes = read_at(directory, name)?.ok_or(ActionError::Drift)?;
        let original = git(root, &["cat-file", "blob", bits[1]])?;
        if original.as_bytes() != bytes {
            return Err(ActionError::Drift);
        }
        contents.insert(name.to_owned(), hash(&bytes));
    }
    if contents.len() != expected.len() {
        return Err(ActionError::Drift);
    }
    for entry in fs::read_dir(root).map_err(|_| ActionError::Target)? {
        let entry = entry.map_err(|_| ActionError::Target)?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| ActionError::Scope)?;
        if name != ".git" && Some(name.as_str()) != owned_temp && !contents.contains_key(&name) {
            return Err(ActionError::Drift);
        }
    }
    let branch = git(root, &["symbolic-ref", "-q", "HEAD"])?;
    serde_json::to_vec(&(
        root,
        directory.metadata().map_err(|_| ActionError::Drift)?.dev(),
        directory.metadata().map_err(|_| ActionError::Drift)?.ino(),
        head,
        branch,
        contents,
    ))
    .map(|b| hash(&b))
    .map_err(|_| ActionError::Storage)
}
fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    use std::os::unix::fs::OpenOptionsExt;
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
        .map_err(|_| ActionError::Storage)?;
    f.write_all(bytes)
        .and_then(|()| f.sync_all())
        .map_err(|_| ActionError::Storage)
}
pub(crate) fn verify_snapshot(
    area: &Path,
    file: &str,
    tests: &str,
    candidate: &str,
    test_content: &str,
) -> Result<()> {
    let dir = File::from(
        rustix::fs::open(
            area,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW,
            Mode::empty(),
        )
        .map_err(|_| ActionError::Drift)?,
    );
    if read_mode(&dir, file, 0o444)?.as_deref() != Some(candidate.as_bytes())
        || read_mode(&dir, tests, 0o444)?.as_deref() != Some(test_content.as_bytes())
    {
        return Err(ActionError::Drift);
    }
    let entries = fs::read_dir(area).map_err(|_| ActionError::Drift)?.count();
    if entries != 2 {
        return Err(ActionError::Drift);
    }
    Ok(())
}
impl Workspace {
    pub(crate) fn prepare(
        root: PathBuf,
        file: String,
        test_file: String,
        area: PathBuf,
    ) -> Result<Self> {
        super::file_name(&file)?;
        super::file_name(&test_file)?;
        if file == test_file || file.starts_with("test_") || !test_file.starts_with("test_") {
            return Err(ActionError::Scope);
        }
        if root.canonicalize().map_err(|_| ActionError::Target)? != root {
            return Err(ActionError::Scope);
        }
        let directory = File::from(
            rustix::fs::open(
                &root,
                OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW,
                Mode::empty(),
            )
            .map_err(|_| ActionError::Target)?,
        );
        if Path::new(git(&root, &["rev-parse", "--show-toplevel"])?.trim()) != root {
            return Err(ActionError::Scope);
        }
        // First supported target is an ordinary owner-owned clone, not a linked
        // worktree, alternate metadata path or symlink. No Git metadata is copied.
        let meta = fs::symlink_metadata(root.join(".git")).map_err(|_| ActionError::Scope)?;
        let own = directory.metadata().map_err(|_| ActionError::Scope)?;
        if !meta.is_dir() || meta.uid() != own.uid() || own.mode() & 0o022 != 0 {
            return Err(ActionError::Scope);
        }
        let baseline = fingerprint(&root, &directory, None)?;
        let original = read_at(&directory, &file)?;
        if original.is_some() {
            return Err(ActionError::Scope);
        }
        let test = read_at(&directory, &test_file)?.ok_or(ActionError::Scope)?;
        if area.exists() {
            return Err(ActionError::Storage);
        }
        use std::os::unix::fs::DirBuilderExt;
        fs::DirBuilder::new()
            .mode(0o700)
            .create(&area)
            .map_err(|_| ActionError::Storage)?;
        if let Some(bytes) = &original {
            write_new(&area.join("original.py"), bytes)?;
        }
        write_new(&area.join("original-tests.py"), &test)?;
        write_new(
            &area.join("baseline.json"),
            &serde_json::to_vec(&(&file, &test_file, &baseline, original.is_some()))
                .map_err(|_| ActionError::Storage)?,
        )?;
        let recovery_directory = File::from(
            rustix::fs::open(
                &area,
                OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW,
                Mode::empty(),
            )
            .map_err(|_| ActionError::Storage)?,
        );
        let this = Self {
            root,
            directory,
            area,
            recovery_directory,
            file,
            test_file,
            original,
            test,
            baseline,
        };
        this.recheck()?;
        Ok(this)
    }
    pub(crate) fn record_coding_rejection(&self, rejection: super::CodingRejection) -> Result<()> {
        let path = fs::symlink_metadata(&self.area).map_err(|_| ActionError::Storage)?;
        let bound = self
            .recovery_directory
            .metadata()
            .map_err(|_| ActionError::Storage)?;
        if !path.is_dir()
            || path.dev() != bound.dev()
            || path.ino() != bound.ino()
            || path.uid() != bound.uid()
            || path.mode() & 0o777 != 0o700
        {
            return Err(ActionError::Storage);
        }
        // Only closed enums can reach this record. No input, error Display or path fields.
        let bytes = serde_json::to_vec(&serde_json::json!({
            "phase": rejection.phase(), "category": rejection.category()
        }))
        .map_err(|_| ActionError::Storage)?;
        if bytes.len() > 128 {
            return Err(ActionError::Storage);
        }
        let fd = openat(
            &self.recovery_directory,
            "coding-rejection.json",
            OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW,
            Mode::RUSR | Mode::WUSR,
        )
        .map_err(|_| ActionError::Storage)?;
        let mut file = File::from(fd);
        file.write_all(&bytes)
            .and_then(|()| file.sync_all())
            .map_err(|_| ActionError::Storage)
    }
    pub(crate) fn evidence(&self) -> Result<Evidence> {
        Ok(Evidence {
            file: self.file.clone(),
            test_file: self.test_file.clone(),
            baseline: self.baseline.clone(),
            original: text(self.original.as_deref().unwrap_or_default())?,
            tests: self.context()?,
            attempts: vec![],
            review_hash: None,
            disposition: "prepared".into(),
            recovery: self.area.to_string_lossy().into_owned(),
            requests: 0,
        })
    }
    pub(crate) fn context(&self) -> Result<String> {
        text(&self.test)
    }
    pub(crate) fn recheck(&self) -> Result<()> {
        let m = fs::symlink_metadata(&self.root).map_err(|_| ActionError::Drift)?;
        let d = self.directory.metadata().map_err(|_| ActionError::Drift)?;
        if !m.is_dir()
            || m.dev() != d.dev()
            || m.ino() != d.ino()
            || fingerprint(&self.root, &self.directory, None)? != self.baseline
            || read_at(&self.directory, &self.file)? != self.original
            || read_at(&self.directory, &self.test_file)?.as_deref() != Some(self.test.as_slice())
        {
            return Err(ActionError::Drift);
        }
        Ok(())
    }
    pub(crate) fn stage(&self, candidate: &str, attempt: usize) -> Result<PathBuf> {
        if attempt >= super::MAX_ATTEMPTS {
            return Err(ActionError::Limit);
        }
        self.recheck()?;
        text(candidate.as_bytes())?;
        let p = self.area.join(format!("attempt-{attempt}"));
        fs::create_dir(&p).map_err(|_| ActionError::Storage)?;
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&p, fs::Permissions::from_mode(0o755))
            .map_err(|_| ActionError::Storage)?;
        for (name, bytes) in [
            (&self.file, candidate.as_bytes()),
            (&self.test_file, self.test.as_slice()),
        ] {
            write_new(&p.join(name), bytes)?;
            fs::set_permissions(p.join(name), fs::Permissions::from_mode(0o444))
                .map_err(|_| ActionError::Storage)?;
        }
        Ok(p)
    }
    #[cfg(target_os = "macos")]
    pub(crate) fn native_approval(
        &self,
        run: &str,
        e: &Evidence,
        review: &str,
        source: &crate::approvals::decision_source::MacOsNativeApprovalDecisionSource<'_>,
    ) -> Result<Option<ApplyGrant>> {
        use crate::approvals::{
            manager::{ApprovalManager, InMemoryApprovalManager},
            types::ApprovalDisposition,
        };
        if e.review_hash()? != review {
            return Err(ActionError::Drift);
        }
        if self.area.join("approval.json").exists() {
            return Err(ActionError::Approval);
        }
        let summary = format!(
            "{} / {} | baseline {} | reviewed change {}",
            self.root.display(),
            self.file,
            self.baseline,
            review
        );
        let mut manager = InMemoryApprovalManager::new();
        let id = manager
            .create_isolated_change(run, review, &summary)
            .map_err(|_| ActionError::Approval)?;
        let presentation = manager
            .issue_presentation(id)
            .map_err(|_| ActionError::Approval)?;
        let outcome = source.request_decision(presentation);
        let resolution = manager
            .resolve_source_outcome(outcome)
            .map_err(|_| ActionError::Approval)?;
        let mut audit = crate::audit::approval::InMemoryApprovalAuditAdapter::new();
        audit
            .record(&resolution)
            .map_err(|_| ActionError::Approval)?;
        let approved = resolution.disposition() == ApprovalDisposition::Approved;
        // Immutable create-new receipt also prevents repeated native review/apply.
        let receipt = serde_json::json!({"run":run,"reviewHash":review,"approved":approved,"disposition":format!("{:?}",resolution.disposition()),"source":"native_dialog","authentication":"local_interaction_only","at":crate::collaboration::now()});
        write_new(
            &self.area.join("approval.json"),
            &serde_json::to_vec(&receipt).map_err(|_| ActionError::Storage)?,
        )?;
        Ok(approved.then(|| ApplyGrant {
            review: review.into(),
            baseline: self.baseline.clone(),
            deadline: std::time::Instant::now() + std::time::Duration::from_secs(30),
        }))
    }
    #[cfg(not(target_os = "macos"))]
    pub(crate) fn native_approval(
        &self,
        _run: &str,
        _e: &Evidence,
        _review: &str,
    ) -> Result<Option<ApplyGrant>> {
        Err(ActionError::Approval)
    }
    #[cfg(test)]
    pub(crate) fn test_grant(&self, review: &str) -> ApplyGrant {
        ApplyGrant {
            review: review.into(),
            baseline: self.baseline.clone(),
            deadline: std::time::Instant::now() + std::time::Duration::from_secs(30),
        }
    }
    pub(crate) fn apply(&self, evidence: &Evidence, grant: ApplyGrant) -> Result<()> {
        if grant.baseline != self.baseline || std::time::Instant::now() >= grant.deadline {
            return Err(ActionError::Approval);
        }
        let approved_hash = grant.review.as_str();
        if approved_hash.len() != 64
            || !approved_hash.bytes().all(|b| b.is_ascii_hexdigit())
            || evidence.review_hash()?.as_str() != approved_hash
            || evidence.baseline != self.baseline
            || evidence.file != self.file
            || evidence.test_file != self.test_file
        {
            return Err(ActionError::Drift);
        }
        self.recheck()?;
        // Original evidence is independently checked before any application.
        if fs::read(self.area.join("original-tests.py")).map_err(|_| ActionError::Recovery)?
            != self.test
        {
            return Err(ActionError::Drift);
        }
        if let Some(original) = &self.original {
            if fs::read(self.area.join("original.py")).map_err(|_| ActionError::Recovery)?
                != *original
            {
                return Err(ActionError::Drift);
            }
        }
        let last = evidence.attempts.last().ok_or(ActionError::Validation)?;
        verify_snapshot(
            &self
                .area
                .join(format!("attempt-{}", evidence.attempts.len() - 1)),
            &self.file,
            &self.test_file,
            &last.candidate,
            &self.context()?,
        )?;
        if last.check.test_hash != hash(&self.test) {
            return Err(ActionError::Drift);
        }
        let journal = self.area.join("apply-started.json");
        write_new(
            &journal,
            &serde_json::to_vec(&(
                approved_hash,
                &self.baseline,
                &last.candidate_hash,
                self.original.is_some(),
            ))
            .map_err(|_| ActionError::Storage)?,
        )?;
        File::open(&self.area)
            .and_then(|f| f.sync_all())
            .map_err(|_| ActionError::Recovery)?;
        self.recheck()?;
        let tmp = format!(".cortexa-apply-{}", &approved_hash[..24]);
        let fd = openat(
            &self.directory,
            tmp.as_str(),
            OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW,
            Mode::RUSR | Mode::WUSR | Mode::RGRP | Mode::ROTH,
        )
        .map_err(|_| ActionError::Recovery)?;
        let mut file = File::from(fd);
        file.write_all(last.candidate.as_bytes())
            .and_then(|()| file.sync_all())
            .map_err(|_| ActionError::Recovery)?;
        // Recheck the exact preimage through the held directory immediately before
        // atomic addition. No model or WebView path is used by renameat.
        let m = fs::symlink_metadata(&self.root).map_err(|_| ActionError::Drift)?;
        let d = self.directory.metadata().map_err(|_| ActionError::Drift)?;
        if !m.is_dir()
            || m.dev() != d.dev()
            || m.ino() != d.ino()
            || fingerprint(&self.root, &self.directory, Some(&tmp))? != self.baseline
            || read_at(&self.directory, &self.file)? != self.original
            || read_at(&self.directory, &self.test_file)?.as_deref() != Some(self.test.as_slice())
        {
            let _ = unlinkat(&self.directory, tmp.as_str(), AtFlags::empty());
            return Err(ActionError::Drift);
        }
        if std::time::Instant::now() >= grant.deadline {
            return Err(ActionError::Approval);
        }
        // The first supported target operation is addition only. Atomic NOREPLACE
        // rejects even a file created between the final check and this syscall.
        add_new_file(&self.directory, &tmp, &self.file)?;
        self.directory
            .sync_all()
            .map_err(|_| ActionError::Recovery)?;
        if read_at(&self.directory, &self.file)?.as_deref() != Some(last.candidate.as_bytes()) {
            return Err(ActionError::Recovery);
        }
        write_new(
            &self.area.join("apply-complete.json"),
            approved_hash.as_bytes(),
        )?;
        File::open(&self.area)
            .and_then(|f| f.sync_all())
            .map_err(|_| ActionError::Recovery)?;
        Ok(())
    }
}

// NOREPLACE is the effect-time guard against a creator racing the preflight.
pub(super) fn add_new_file(directory: &File, staged: &str, target: &str) -> Result<()> {
    rustix::fs::renameat_with(
        directory,
        staged,
        directory,
        target,
        rustix::fs::RenameFlags::NOREPLACE,
    )
    .map_err(|_| ActionError::Recovery)
}

#[cfg(test)]
mod rejection_receipt_tests {
    use super::*;
    use std::os::unix::fs::{symlink, PermissionsExt};
    fn workspace(root: &Path) -> std::result::Result<Workspace, Box<dyn std::error::Error>> {
        let area = root.join("recovery");
        fs::create_dir(&area)?;
        fs::set_permissions(&area, fs::Permissions::from_mode(0o700))?;
        Ok(Workspace {
            root: root.to_owned(),
            directory: File::open(root)?,
            recovery_directory: File::open(&area)?,
            area,
            file: "solution.py".into(),
            test_file: "test_solution.py".into(),
            original: None,
            test: vec![],
            baseline: String::new(),
        })
    }
    #[test]
    fn rejection_receipt_is_closed_private_and_create_new(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let temp = tempfile::tempdir()?;
        let w = workspace(temp.path())?;
        w.record_coding_rejection(super::super::CodingRejection::Edit(
            super::super::EditRejection::SchemaData,
        ))?;
        let p = w.area.join("coding-rejection.json");
        let before = fs::read(&p)?;
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&before)?,
            serde_json::json!({"phase":"edit_contract","category":"schema_data"})
        );
        assert!(before.len() <= 128);
        assert_eq!(fs::metadata(&p)?.mode() & 0o777, 0o600);
        assert_eq!(
            w.record_coding_rejection(super::super::CodingRejection::Workspace(ActionError::Drift)),
            Err(ActionError::Storage)
        );
        assert_eq!(fs::read(&p)?, before);
        assert_eq!(fs::read_dir(&w.area)?.count(), 1);
        Ok(())
    }
    #[test]
    fn schema_shape_receipts_keep_privacy_and_replay_boundary(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        use super::super::{CodingRejection, EditRejection};
        for category in [
            EditRejection::SchemaRoot,
            EditRejection::SchemaMissing,
            EditRejection::SchemaUnexpected,
            EditRejection::SchemaDuplicate,
            EditRejection::SchemaType,
            EditRejection::SchemaVersionRange,
            EditRejection::SchemaData,
        ] {
            let temp = tempfile::tempdir()?;
            let w = workspace(temp.path())?;
            w.record_coding_rejection(CodingRejection::Edit(category))?;
            let p = w.area.join("coding-rejection.json");
            let bytes = fs::read(&p)?;
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(&bytes)?,
                serde_json::json!({"phase":"edit_contract","category":category.label()})
            );
            assert!(bytes.len() <= 128);
            assert_eq!(fs::metadata(&p)?.mode() & 0o777, 0o600);
            assert_eq!(
                w.record_coding_rejection(CodingRejection::Edit(category)),
                Err(ActionError::Storage)
            );
            assert_eq!(fs::read(&p)?, bytes);
            assert_eq!(fs::read_dir(&w.area)?.count(), 1);
            assert_eq!(CodingRejection::Edit(category).error(), ActionError::Scope);
        }
        Ok(())
    }
    #[test]
    fn rejection_receipt_rejects_links_replacement_and_permissions(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let rejection = super::super::CodingRejection::Workspace(ActionError::Scope);
        for mode in 0..3 {
            let temp = tempfile::tempdir()?;
            let w = workspace(temp.path())?;
            let outside = temp.path().join("outside");
            fs::write(&outside, "private-canary")?;
            if mode == 0 {
                symlink(&outside, w.area.join("coding-rejection.json"))?;
            }
            if mode == 1 {
                fs::rename(&w.area, temp.path().join("old"))?;
                fs::create_dir(&w.area)?;
            }
            if mode == 2 {
                fs::set_permissions(&w.area, fs::Permissions::from_mode(0o755))?;
            }
            assert_eq!(
                w.record_coding_rejection(rejection),
                Err(ActionError::Storage)
            );
            assert_eq!(fs::read_to_string(&outside)?, "private-canary");
        }
        Ok(())
    }
}
