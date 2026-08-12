//! Bounded, application-owned access to explicitly approved documents.
//!
//! Paths remain private to this module. A runtime or agent can neither register
//! paths nor enumerate the registry; the orchestrator supplies unforgeable
//! application-control and live-access proofs for every operation.

use std::{
    collections::BTreeMap,
    fmt,
    fs::{self, File, Metadata},
    io::Read,
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::agent::{
    definition::AgentId,
    governance::AgentAttribution,
    orchestrator::{ApplicationDocumentControlProof, LiveDocumentAccessProof},
    task::{AgentTaskId, AgentTaskOutput, RootTaskId},
};

pub const MAX_APPROVED_ROOTS: usize = 8;
pub const MAX_APPROVED_DOCUMENTS: usize = 16;
pub const MAX_DOCUMENT_BYTES: usize = 16_384;
pub const MAX_DOCUMENT_MEMORY_CONTEXT_BYTES: usize = 8_192;
pub const MAX_DOCUMENT_REQUEST_FRAMING_BYTES: usize = 2_048;
pub const MAX_DOCUMENT_RAW_REQUEST_BYTES: usize =
    MAX_DOCUMENT_BYTES + MAX_DOCUMENT_MEMORY_CONTEXT_BYTES + MAX_DOCUMENT_REQUEST_FRAMING_BYTES;
pub const MAX_DOCUMENT_PATH_BYTES: usize = 4_096;
pub const MAX_RELATIVE_PATH_COMPONENTS: usize = 64;
pub const MAX_RELATIVE_PATH_COMPONENT_BYTES: usize = 255;

type ApprovedDocumentResult<T> = Result<T, ApprovedDocumentError>;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ApprovedDocumentId(String);

impl ApprovedDocumentId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ApprovedDocumentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ApprovedDocumentId")
            .field(&"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ApprovedRootId(String);

impl ApprovedRootId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ApprovedRootId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ApprovedRootId")
            .field(&"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovedDocumentSource {
    UserSelectedFile,
    TaskAttachment,
    ApprovedRootMember,
    GeneratedArtifact,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovedDocumentFormat {
    Utf8Text,
    Markdown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DocumentOperation {
    Read,
    Summarize,
    ExtractFacts,
    IdentifySections,
    Classify,
    ProposeOutline,
}

impl DocumentOperation {
    pub const ALL: [Self; 6] = [
        Self::Read,
        Self::Summarize,
        Self::ExtractFacts,
        Self::IdentifySections,
        Self::Classify,
        Self::ProposeOutline,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Summarize => "summarize",
            Self::ExtractFacts => "extract-facts",
            Self::IdentifySections => "identify-sections",
            Self::Classify => "classify",
            Self::ProposeOutline => "propose-outline",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DocumentCapabilities {
    available: bool,
}

impl DocumentCapabilities {
    #[must_use]
    pub const fn available(self) -> bool {
        self.available
    }

    #[must_use]
    pub const fn formats(self) -> &'static [ApprovedDocumentFormat] {
        if self.available {
            &[
                ApprovedDocumentFormat::Utf8Text,
                ApprovedDocumentFormat::Markdown,
            ]
        } else {
            &[]
        }
    }

    #[must_use]
    pub const fn operations(self) -> &'static [DocumentOperation] {
        if self.available {
            &DocumentOperation::ALL
        } else {
            &[]
        }
    }

    #[must_use]
    pub const fn maximum_document_bytes(self) -> usize {
        if self.available {
            MAX_DOCUMENT_BYTES
        } else {
            0
        }
    }

    #[must_use]
    pub const fn maximum_raw_request_bytes(self) -> usize {
        if self.available {
            MAX_DOCUMENT_RAW_REQUEST_BYTES
        } else {
            0
        }
    }

    #[must_use]
    pub const fn read_only(self) -> bool {
        self.available
    }

    #[must_use]
    pub const fn supports_enumeration(self) -> bool {
        false
    }

    #[must_use]
    pub const fn supports_artifact_write(self) -> bool {
        false
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ApprovedRelativePath(String);

impl ApprovedRelativePath {
    pub fn new(value: impl Into<String>) -> ApprovedDocumentResult<Self> {
        let value = value.into();
        validate_relative_path(&value)?;
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn components(&self) -> impl Iterator<Item = &str> {
        self.0.split('/')
    }
}

impl fmt::Debug for ApprovedRelativePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ApprovedRelativePath")
            .field("value", &"[REDACTED]")
            .field("bytes", &self.0.len())
            .finish()
    }
}

/// Exact live authority for a single document operation. Construction requires
/// the orchestrator's unforgeable proof and is unavailable to runtimes/models.
pub struct DocumentAccessGrant {
    attribution: AgentAttribution,
    document_id: ApprovedDocumentId,
    target_agent_id: AgentId,
    operation: DocumentOperation,
}

impl DocumentAccessGrant {
    pub(crate) fn from_live_attribution(
        attribution: AgentAttribution,
        document_id: ApprovedDocumentId,
        target_agent_id: AgentId,
        operation: DocumentOperation,
        _proof: LiveDocumentAccessProof,
    ) -> ApprovedDocumentResult<Self> {
        if attribution.agent_id() != AgentId::PersonalAssistant
            || attribution.task_id() != attribution.root_task_id().task_id()
            || attribution.parent_task_id().is_some()
            || attribution.depth() != 0
            || target_agent_id != AgentId::KnowledgeDocument
        {
            return Err(ApprovedDocumentError::AccessDenied);
        }
        Ok(Self {
            attribution,
            document_id,
            target_agent_id,
            operation,
        })
    }
}

impl fmt::Debug for DocumentAccessGrant {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DocumentAccessGrant")
            .field("attribution", &self.attribution)
            .field("document_id", &self.document_id)
            .field("target_agent_id", &self.target_agent_id)
            .field("operation", &self.operation)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct DocumentTaskDescriptor {
    document_id: ApprovedDocumentId,
    format: ApprovedDocumentFormat,
    operation: DocumentOperation,
}

impl DocumentTaskDescriptor {
    #[must_use]
    pub fn document_id(&self) -> &ApprovedDocumentId {
        &self.document_id
    }

    #[must_use]
    pub const fn format(&self) -> ApprovedDocumentFormat {
        self.format
    }

    #[must_use]
    pub const fn operation(&self) -> DocumentOperation {
        self.operation
    }
}

impl fmt::Debug for DocumentTaskDescriptor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DocumentTaskDescriptor")
            .field("document_id", &self.document_id)
            .field("format", &self.format)
            .field("operation", &self.operation)
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct DocumentTaskResult {
    task_id: AgentTaskId,
    descriptor: DocumentTaskDescriptor,
    output: AgentTaskOutput,
}

impl DocumentTaskResult {
    pub(crate) fn new(
        task_id: AgentTaskId,
        descriptor: DocumentTaskDescriptor,
        output: AgentTaskOutput,
    ) -> Self {
        Self {
            task_id,
            descriptor,
            output,
        }
    }

    #[must_use]
    pub fn task_id(&self) -> &AgentTaskId {
        &self.task_id
    }

    #[must_use]
    pub fn descriptor(&self) -> &DocumentTaskDescriptor {
        &self.descriptor
    }

    #[must_use]
    pub fn output(&self) -> &AgentTaskOutput {
        &self.output
    }
}

impl fmt::Debug for DocumentTaskResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DocumentTaskResult")
            .field("task_id", &self.task_id)
            .field("descriptor", &self.descriptor)
            .field("output", &"[REDACTED]")
            .finish()
    }
}

pub struct PreparedDocumentRead {
    reservation: u64,
    descriptor: DocumentTaskDescriptor,
    source: ApprovedDocumentSource,
    content: String,
}

impl PreparedDocumentRead {
    #[must_use]
    pub fn descriptor(&self) -> &DocumentTaskDescriptor {
        &self.descriptor
    }

    #[must_use]
    pub const fn source(&self) -> ApprovedDocumentSource {
        self.source
    }

    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }

    #[must_use]
    pub fn byte_count(&self) -> usize {
        self.content.len()
    }
}

impl fmt::Debug for PreparedDocumentRead {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PreparedDocumentRead")
            .field("descriptor", &self.descriptor)
            .field("source", &self.source)
            .field("content", &"[REDACTED]")
            .field("byte_count", &self.content.len())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct DocumentReadCommit {
    descriptor: DocumentTaskDescriptor,
    source: ApprovedDocumentSource,
    byte_count: usize,
}

impl DocumentReadCommit {
    #[must_use]
    pub fn descriptor(&self) -> &DocumentTaskDescriptor {
        &self.descriptor
    }

    #[must_use]
    pub const fn source(&self) -> ApprovedDocumentSource {
        self.source
    }

    #[must_use]
    pub const fn byte_count(&self) -> usize {
        self.byte_count
    }
}

impl fmt::Debug for DocumentReadCommit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DocumentReadCommit")
            .field("descriptor", &self.descriptor)
            .field("source", &self.source)
            .field("byte_count", &self.byte_count)
            .finish()
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ApprovedDocumentError {
    #[error("approved-document access is unsupported on this platform")]
    UnsupportedPlatform,
    #[error("document path is invalid")]
    InvalidPath,
    #[error("approved-root member path is invalid")]
    InvalidRelativePath,
    #[error("document format is unsupported")]
    UnsupportedFormat,
    #[error("document source must use the approved-root registration operation")]
    InvalidSource,
    #[error("approved-root capacity is exhausted")]
    RootLimitExceeded,
    #[error("approved-document capacity is exhausted")]
    DocumentLimitExceeded,
    #[error("approved-document identity space is exhausted")]
    IdentityExhausted,
    #[error("approved root was not found")]
    RootNotFound,
    #[error("approved document was not found")]
    DocumentNotFound,
    #[error("approved document belongs to another root workflow")]
    RootWorkflowMismatch,
    #[error("document access authority does not permit this operation")]
    AccessDenied,
    #[error("approved path is unavailable")]
    PathUnavailable,
    #[error("symlink paths are prohibited")]
    SymlinkRejected,
    #[error("approved document must be one regular file")]
    NotRegularFile,
    #[error("hard-linked document targets are prohibited")]
    HardLinkRejected,
    #[error("approved path escaped its registered root")]
    RootContainmentViolation,
    #[error("approved path identity changed")]
    PathIdentityChanged,
    #[error("approved document is empty")]
    EmptyDocument,
    #[error("approved document exceeds its byte limit")]
    DocumentTooLarge,
    #[error("approved document is not valid UTF-8")]
    InvalidUtf8,
    #[error("approved document contains a prohibited NUL byte")]
    ProhibitedNul,
    #[error("approved document contains a prohibited control character")]
    ProhibitedControlCharacter,
    #[error("approved-document read failed")]
    ReadFailed,
    #[error("approved document is already reserved")]
    ReferenceReserved,
    #[error("approved document was already consumed")]
    ReferenceConsumed,
    #[error("approved document was revoked")]
    ReferenceRevoked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReferenceState {
    Available,
    Reserved(u64),
    Consumed,
    Revoked,
}

struct ApprovedRootEntry {
    root_task_id: RootTaskId,
    path: PathBuf,
    identity: FileIdentity,
}

enum RegisteredTarget {
    Direct {
        path: PathBuf,
        identity: FileIdentity,
    },
    RootMember {
        root_id: ApprovedRootId,
        relative_path: ApprovedRelativePath,
        identity: FileIdentity,
    },
}

struct ApprovedDocumentEntry {
    root_task_id: RootTaskId,
    source: ApprovedDocumentSource,
    format: ApprovedDocumentFormat,
    target: Option<RegisteredTarget>,
    state: ReferenceState,
}

pub struct ApprovedDocumentReader {
    workflow_sequence: u64,
    roots: BTreeMap<ApprovedRootId, ApprovedRootEntry>,
    documents: BTreeMap<ApprovedDocumentId, ApprovedDocumentEntry>,
    next_root_id: u64,
    next_document_id: u64,
    next_reservation: u64,
}

impl ApprovedDocumentReader {
    #[must_use]
    pub fn new() -> Self {
        Self::for_workflow(0)
    }

    pub(crate) fn for_workflow(workflow_sequence: u64) -> Self {
        Self {
            workflow_sequence,
            roots: BTreeMap::new(),
            documents: BTreeMap::new(),
            next_root_id: 1,
            next_document_id: 1,
            next_reservation: 1,
        }
    }

    #[must_use]
    pub const fn capabilities(&self) -> DocumentCapabilities {
        DocumentCapabilities {
            available: cfg!(unix),
        }
    }

    pub(crate) fn register_document(
        &mut self,
        root_task_id: RootTaskId,
        source: ApprovedDocumentSource,
        path: impl AsRef<Path>,
        _proof: ApplicationDocumentControlProof,
    ) -> ApprovedDocumentResult<ApprovedDocumentId> {
        ensure_supported_platform()?;
        if source == ApprovedDocumentSource::ApprovedRootMember {
            return Err(ApprovedDocumentError::InvalidSource);
        }
        self.ensure_document_capacity()?;
        let target = validate_direct_target(path.as_ref())?;
        let format = format_for_path(&target.path)?;
        read_validated_file(&target.path, target.identity)?;
        let id = self.allocate_document_id()?;
        self.documents.insert(
            id.clone(),
            ApprovedDocumentEntry {
                root_task_id,
                source,
                format,
                target: Some(RegisteredTarget::Direct {
                    path: target.path,
                    identity: target.identity,
                }),
                state: ReferenceState::Available,
            },
        );
        Ok(id)
    }

    pub(crate) fn register_root(
        &mut self,
        root_task_id: RootTaskId,
        path: impl AsRef<Path>,
        _proof: ApplicationDocumentControlProof,
    ) -> ApprovedDocumentResult<ApprovedRootId> {
        ensure_supported_platform()?;
        if self.roots.len() >= MAX_APPROVED_ROOTS {
            return Err(ApprovedDocumentError::RootLimitExceeded);
        }
        let (path, identity) = validate_root(path.as_ref())?;
        let id = self.allocate_root_id()?;
        self.roots.insert(
            id.clone(),
            ApprovedRootEntry {
                root_task_id,
                path,
                identity,
            },
        );
        Ok(id)
    }

    pub(crate) fn register_root_member(
        &mut self,
        root_task_id: RootTaskId,
        root_id: &ApprovedRootId,
        relative_path: ApprovedRelativePath,
        _proof: ApplicationDocumentControlProof,
    ) -> ApprovedDocumentResult<ApprovedDocumentId> {
        ensure_supported_platform()?;
        self.ensure_document_capacity()?;
        let root = self
            .roots
            .get(root_id)
            .ok_or(ApprovedDocumentError::RootNotFound)?;
        if root.root_task_id != root_task_id {
            return Err(ApprovedDocumentError::RootWorkflowMismatch);
        }
        let target = resolve_root_member(root, &relative_path)?;
        let format = format_for_path(&target.path)?;
        read_validated_file(&target.path, target.identity)?;
        let id = self.allocate_document_id()?;
        self.documents.insert(
            id.clone(),
            ApprovedDocumentEntry {
                root_task_id,
                source: ApprovedDocumentSource::ApprovedRootMember,
                format,
                target: Some(RegisteredTarget::RootMember {
                    root_id: root_id.clone(),
                    relative_path,
                    identity: target.identity,
                }),
                state: ReferenceState::Available,
            },
        );
        Ok(id)
    }

    pub(crate) fn revoke(
        &mut self,
        document_id: &ApprovedDocumentId,
        _proof: ApplicationDocumentControlProof,
    ) -> ApprovedDocumentResult<()> {
        let entry = self
            .documents
            .get_mut(document_id)
            .ok_or(ApprovedDocumentError::DocumentNotFound)?;
        match entry.state {
            ReferenceState::Available => {
                entry.state = ReferenceState::Revoked;
                entry.target = None;
                Ok(())
            }
            ReferenceState::Reserved(_) => Err(ApprovedDocumentError::ReferenceReserved),
            ReferenceState::Consumed => Err(ApprovedDocumentError::ReferenceConsumed),
            ReferenceState::Revoked => Err(ApprovedDocumentError::ReferenceRevoked),
        }
    }

    pub(crate) fn prepare_read(
        &mut self,
        grant: &DocumentAccessGrant,
    ) -> ApprovedDocumentResult<PreparedDocumentRead> {
        ensure_supported_platform()?;
        if grant.target_agent_id != AgentId::KnowledgeDocument {
            return Err(ApprovedDocumentError::AccessDenied);
        }
        let entry = self
            .documents
            .get(&grant.document_id)
            .ok_or(ApprovedDocumentError::DocumentNotFound)?;
        if &entry.root_task_id != grant.attribution.root_task_id() {
            return Err(ApprovedDocumentError::RootWorkflowMismatch);
        }
        match entry.state {
            ReferenceState::Available => {}
            ReferenceState::Reserved(_) => return Err(ApprovedDocumentError::ReferenceReserved),
            ReferenceState::Consumed => return Err(ApprovedDocumentError::ReferenceConsumed),
            ReferenceState::Revoked => return Err(ApprovedDocumentError::ReferenceRevoked),
        }
        let content = self.read_entry(entry)?;
        let descriptor = DocumentTaskDescriptor {
            document_id: grant.document_id.clone(),
            format: entry.format,
            operation: grant.operation,
        };
        let source = entry.source;
        let reservation = self.next_reservation;
        self.next_reservation = self
            .next_reservation
            .checked_add(1)
            .ok_or(ApprovedDocumentError::IdentityExhausted)?;
        self.documents
            .get_mut(&grant.document_id)
            .ok_or(ApprovedDocumentError::DocumentNotFound)?
            .state = ReferenceState::Reserved(reservation);
        Ok(PreparedDocumentRead {
            reservation,
            descriptor,
            source,
            content,
        })
    }

    pub(crate) fn abort_read(&mut self, prepared: PreparedDocumentRead) {
        if let Some(entry) = self.documents.get_mut(prepared.descriptor.document_id()) {
            if entry.state == ReferenceState::Reserved(prepared.reservation) {
                entry.state = ReferenceState::Available;
            }
        }
    }

    pub(crate) fn commit_read(&mut self, prepared: PreparedDocumentRead) -> DocumentReadCommit {
        if let Some(entry) = self.documents.get_mut(prepared.descriptor.document_id()) {
            if entry.state == ReferenceState::Reserved(prepared.reservation) {
                entry.state = ReferenceState::Consumed;
                entry.target = None;
            }
        }
        DocumentReadCommit {
            descriptor: prepared.descriptor,
            source: prepared.source,
            byte_count: prepared.content.len(),
        }
    }

    pub(crate) fn cleanup_document(&mut self, document_id: &ApprovedDocumentId) {
        self.documents.remove(document_id);
    }

    pub(crate) fn cleanup_root(&mut self, root_task_id: &RootTaskId) {
        self.documents
            .retain(|_, entry| &entry.root_task_id != root_task_id);
        self.roots
            .retain(|_, entry| &entry.root_task_id != root_task_id);
    }

    fn read_entry(&self, entry: &ApprovedDocumentEntry) -> ApprovedDocumentResult<String> {
        let target = match entry
            .target
            .as_ref()
            .ok_or(ApprovedDocumentError::PathIdentityChanged)?
        {
            RegisteredTarget::Direct { path, identity } => {
                let target = validate_direct_target(path)?;
                if target.identity != *identity {
                    return Err(ApprovedDocumentError::PathIdentityChanged);
                }
                target
            }
            RegisteredTarget::RootMember {
                root_id,
                relative_path,
                identity,
            } => {
                let root = self
                    .roots
                    .get(root_id)
                    .ok_or(ApprovedDocumentError::RootNotFound)?;
                if root.root_task_id != entry.root_task_id {
                    return Err(ApprovedDocumentError::RootWorkflowMismatch);
                }
                let target = resolve_root_member(root, relative_path)?;
                if target.identity != *identity {
                    return Err(ApprovedDocumentError::PathIdentityChanged);
                }
                target
            }
        };
        if format_for_path(&target.path)? != entry.format {
            return Err(ApprovedDocumentError::PathIdentityChanged);
        }
        read_validated_file(&target.path, target.identity)
    }

    fn ensure_document_capacity(&self) -> ApprovedDocumentResult<()> {
        if self.documents.len() >= MAX_APPROVED_DOCUMENTS {
            Err(ApprovedDocumentError::DocumentLimitExceeded)
        } else {
            Ok(())
        }
    }

    fn allocate_root_id(&mut self) -> ApprovedDocumentResult<ApprovedRootId> {
        let current = self.next_root_id;
        self.next_root_id = current
            .checked_add(1)
            .ok_or(ApprovedDocumentError::IdentityExhausted)?;
        Ok(ApprovedRootId(format!(
            "approved-root-{}-{current}",
            self.workflow_sequence
        )))
    }

    fn allocate_document_id(&mut self) -> ApprovedDocumentResult<ApprovedDocumentId> {
        let current = self.next_document_id;
        self.next_document_id = current
            .checked_add(1)
            .ok_or(ApprovedDocumentError::IdentityExhausted)?;
        Ok(ApprovedDocumentId(format!(
            "approved-document-{}-{current}",
            self.workflow_sequence
        )))
    }
}

impl Default for ApprovedDocumentReader {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for ApprovedDocumentReader {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ApprovedDocumentReader")
            .field("root_count", &self.roots.len())
            .field("document_count", &self.documents.len())
            .finish()
    }
}

fn validate_relative_path(value: &str) -> ApprovedDocumentResult<()> {
    if value.is_empty()
        || value.len() > MAX_DOCUMENT_PATH_BYTES
        || value.contains('\0')
        || value.contains('\\')
        || value.contains(':')
        || value.starts_with('/')
        || value.ends_with('/')
        || value.contains("://")
    {
        return Err(ApprovedDocumentError::InvalidRelativePath);
    }
    let mut component_count = 0_usize;
    for component in value.split('/') {
        component_count = component_count
            .checked_add(1)
            .ok_or(ApprovedDocumentError::InvalidRelativePath)?;
        if component.is_empty()
            || matches!(component, "." | "..")
            || component.len() > MAX_RELATIVE_PATH_COMPONENT_BYTES
            || component.chars().any(char::is_control)
        {
            return Err(ApprovedDocumentError::InvalidRelativePath);
        }
    }
    if component_count > MAX_RELATIVE_PATH_COMPONENTS {
        return Err(ApprovedDocumentError::InvalidRelativePath);
    }
    Ok(())
}

fn validate_ambient_path(path: &Path) -> ApprovedDocumentResult<()> {
    let value = path.to_str().ok_or(ApprovedDocumentError::InvalidPath)?;
    if value.is_empty() || value.len() > MAX_DOCUMENT_PATH_BYTES || value.contains('\0') {
        return Err(ApprovedDocumentError::InvalidPath);
    }
    Ok(())
}

fn format_for_path(path: &Path) -> ApprovedDocumentResult<ApprovedDocumentFormat> {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("txt") => Ok(ApprovedDocumentFormat::Utf8Text),
        Some("md") => Ok(ApprovedDocumentFormat::Markdown),
        _ => Err(ApprovedDocumentError::UnsupportedFormat),
    }
}

struct ValidatedTarget {
    path: PathBuf,
    identity: FileIdentity,
}

fn validate_direct_target(path: &Path) -> ApprovedDocumentResult<ValidatedTarget> {
    validate_ambient_path(path)?;
    let metadata = symlink_metadata(path)?;
    validate_file_metadata(&metadata)?;
    let before = FileIdentity::from_metadata(&metadata);
    let canonical = fs::canonicalize(path).map_err(|_| ApprovedDocumentError::PathUnavailable)?;
    validate_ambient_path(&canonical)?;
    let canonical_metadata = symlink_metadata(&canonical)?;
    validate_file_metadata(&canonical_metadata)?;
    if before != FileIdentity::from_metadata(&canonical_metadata) {
        return Err(ApprovedDocumentError::PathIdentityChanged);
    }
    Ok(ValidatedTarget {
        path: canonical,
        identity: before,
    })
}

fn validate_root(path: &Path) -> ApprovedDocumentResult<(PathBuf, FileIdentity)> {
    validate_ambient_path(path)?;
    let metadata = symlink_metadata(path)?;
    if metadata.file_type().is_symlink() {
        return Err(ApprovedDocumentError::SymlinkRejected);
    }
    if !metadata.is_dir() {
        return Err(ApprovedDocumentError::NotRegularFile);
    }
    let identity = FileIdentity::from_metadata(&metadata);
    let canonical = fs::canonicalize(path).map_err(|_| ApprovedDocumentError::PathUnavailable)?;
    validate_ambient_path(&canonical)?;
    let canonical_metadata = symlink_metadata(&canonical)?;
    if canonical_metadata.file_type().is_symlink()
        || !canonical_metadata.is_dir()
        || identity != FileIdentity::from_metadata(&canonical_metadata)
    {
        return Err(ApprovedDocumentError::PathIdentityChanged);
    }
    Ok((canonical, identity))
}

fn resolve_root_member(
    root: &ApprovedRootEntry,
    relative_path: &ApprovedRelativePath,
) -> ApprovedDocumentResult<ValidatedTarget> {
    let root_metadata = symlink_metadata(&root.path)?;
    if root_metadata.file_type().is_symlink()
        || !root_metadata.is_dir()
        || FileIdentity::from_metadata(&root_metadata) != root.identity
    {
        return Err(ApprovedDocumentError::PathIdentityChanged);
    }
    let mut candidate = root.path.clone();
    let component_count = relative_path.components().count();
    for (index, component) in relative_path.components().enumerate() {
        candidate.push(component);
        let metadata = symlink_metadata(&candidate)?;
        if metadata.file_type().is_symlink() {
            return Err(ApprovedDocumentError::SymlinkRejected);
        }
        if index + 1 < component_count && !metadata.is_dir() {
            return Err(ApprovedDocumentError::PathUnavailable);
        }
    }
    let target = validate_direct_target(&candidate)?;
    if !target.path.starts_with(&root.path) {
        return Err(ApprovedDocumentError::RootContainmentViolation);
    }
    Ok(target)
}

fn read_validated_file(
    path: &Path,
    registered_identity: FileIdentity,
) -> ApprovedDocumentResult<String> {
    read_validated_file_with_hook(path, registered_identity, || Ok(()))
}

fn read_validated_file_with_hook(
    path: &Path,
    registered_identity: FileIdentity,
    after_open: impl FnOnce() -> ApprovedDocumentResult<()>,
) -> ApprovedDocumentResult<String> {
    let before_metadata = symlink_metadata(path)?;
    validate_file_metadata(&before_metadata)?;
    let before = FileIdentity::from_metadata(&before_metadata);
    if before != registered_identity {
        return Err(ApprovedDocumentError::PathIdentityChanged);
    }
    let mut file = File::open(path).map_err(|_| ApprovedDocumentError::ReadFailed)?;
    let opened = FileIdentity::from_metadata(
        &file
            .metadata()
            .map_err(|_| ApprovedDocumentError::ReadFailed)?,
    );
    if opened != before {
        return Err(ApprovedDocumentError::PathIdentityChanged);
    }
    after_open()?;
    let mut bytes = Vec::with_capacity(MAX_DOCUMENT_BYTES.saturating_add(1));
    file.by_ref()
        .take((MAX_DOCUMENT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| ApprovedDocumentError::ReadFailed)?;
    if bytes.len() > MAX_DOCUMENT_BYTES {
        return Err(ApprovedDocumentError::DocumentTooLarge);
    }
    if bytes.is_empty() {
        return Err(ApprovedDocumentError::EmptyDocument);
    }
    if bytes.contains(&0) {
        return Err(ApprovedDocumentError::ProhibitedNul);
    }
    let after_handle = FileIdentity::from_metadata(
        &file
            .metadata()
            .map_err(|_| ApprovedDocumentError::ReadFailed)?,
    );
    let final_metadata = symlink_metadata(path)?;
    validate_file_metadata(&final_metadata)?;
    let final_path = FileIdentity::from_metadata(&final_metadata);
    if before != after_handle || before != final_path || before.size() != bytes.len() as u64 {
        return Err(ApprovedDocumentError::PathIdentityChanged);
    }
    let text = String::from_utf8(bytes).map_err(|_| ApprovedDocumentError::InvalidUtf8)?;
    if text
        .chars()
        .any(|character| character.is_control() && !matches!(character, '\n' | '\r' | '\t'))
    {
        return Err(ApprovedDocumentError::ProhibitedControlCharacter);
    }
    Ok(text)
}

fn symlink_metadata(path: &Path) -> ApprovedDocumentResult<Metadata> {
    fs::symlink_metadata(path).map_err(|_| ApprovedDocumentError::PathUnavailable)
}

#[cfg(unix)]
fn validate_file_metadata(metadata: &Metadata) -> ApprovedDocumentResult<()> {
    use std::os::unix::fs::MetadataExt;

    if metadata.file_type().is_symlink() {
        return Err(ApprovedDocumentError::SymlinkRejected);
    }
    if !metadata.is_file() {
        return Err(ApprovedDocumentError::NotRegularFile);
    }
    if metadata.nlink() != 1 {
        return Err(ApprovedDocumentError::HardLinkRejected);
    }
    Ok(())
}

#[cfg(not(unix))]
fn validate_file_metadata(_metadata: &Metadata) -> ApprovedDocumentResult<()> {
    Err(ApprovedDocumentError::UnsupportedPlatform)
}

#[cfg(unix)]
#[derive(Clone, Copy, Eq, PartialEq)]
struct FileIdentity {
    device: u64,
    inode: u64,
    mode: u32,
    links: u64,
    size: u64,
    modified_seconds: i64,
    modified_nanoseconds: i64,
    changed_seconds: i64,
    changed_nanoseconds: i64,
}

#[cfg(unix)]
impl FileIdentity {
    fn from_metadata(metadata: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;

        Self {
            device: metadata.dev(),
            inode: metadata.ino(),
            mode: metadata.mode(),
            links: metadata.nlink(),
            size: metadata.size(),
            modified_seconds: metadata.mtime(),
            modified_nanoseconds: metadata.mtime_nsec(),
            changed_seconds: metadata.ctime(),
            changed_nanoseconds: metadata.ctime_nsec(),
        }
    }

    const fn size(self) -> u64 {
        self.size
    }
}

#[cfg(not(unix))]
#[derive(Clone, Copy, Eq, PartialEq)]
struct FileIdentity;

#[cfg(not(unix))]
impl FileIdentity {
    fn from_metadata(_metadata: &Metadata) -> Self {
        Self
    }

    const fn size(self) -> u64 {
        0
    }
}

const fn ensure_supported_platform() -> ApprovedDocumentResult<()> {
    if cfg!(unix) {
        Ok(())
    } else {
        Err(ApprovedDocumentError::UnsupportedPlatform)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        sync::atomic::{AtomicU64, Ordering},
    };

    use super::*;
    use crate::agent::{
        native_runtime::NativeAgentRuntime,
        orchestrator::{
            AgentOrchestrator, ApplicationDocumentControlProof, LiveDocumentAccessProof,
        },
    };

    static NEXT_TEMP_DIRECTORY: AtomicU64 = AtomicU64::new(1);

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new() -> std::io::Result<Self> {
            let sequence = NEXT_TEMP_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "cortexa-approved-documents-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir(&path)?;
            Ok(Self(path))
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn live_personal_attribution() -> Result<AgentAttribution, Box<dyn std::error::Error>> {
        let mut orchestrator = AgentOrchestrator::new(NativeAgentRuntime)?;
        let context = orchestrator.start_root("Review one explicitly approved document")?;
        Ok(orchestrator.live_attribution_for_test(&context)?)
    }

    #[test]
    fn relative_paths_are_closed_bounded_and_redacted() -> Result<(), Box<dyn std::error::Error>> {
        let path = ApprovedRelativePath::new("reports/quarter-one.md")?;
        assert_eq!(path.as_str(), "reports/quarter-one.md");
        assert!(!format!("{path:?}").contains("quarter-one"));

        for invalid in [
            "",
            "/absolute.md",
            "trailing/",
            "double//separator.md",
            "./current.md",
            "../parent.md",
            "nested/../escape.md",
            "windows\\separator.md",
            "file://document.md",
            "file:document.md",
            "nul\0document.md",
        ] {
            assert_eq!(
                ApprovedRelativePath::new(invalid),
                Err(ApprovedDocumentError::InvalidRelativePath),
                "{invalid:?} must fail closed"
            );
        }
        assert_eq!(
            ApprovedRelativePath::new(format!("{}.md", "x".repeat(256))),
            Err(ApprovedDocumentError::InvalidRelativePath)
        );
        assert_eq!(
            ApprovedRelativePath::new(
                std::iter::repeat_n("x", MAX_RELATIVE_PATH_COMPONENTS + 1)
                    .collect::<Vec<_>>()
                    .join("/")
            ),
            Err(ApprovedDocumentError::InvalidRelativePath)
        );
        let exact_total = std::iter::repeat_n("x".repeat(63), 63)
            .chain(std::iter::once("x".repeat(64)))
            .collect::<Vec<_>>()
            .join("/");
        assert_eq!(exact_total.len(), MAX_DOCUMENT_PATH_BYTES);
        assert!(ApprovedRelativePath::new(&exact_total).is_ok());
        assert_eq!(
            ApprovedRelativePath::new(format!("{exact_total}x")),
            Err(ApprovedDocumentError::InvalidRelativePath)
        );
        assert!(validate_ambient_path(Path::new(&"x".repeat(MAX_DOCUMENT_PATH_BYTES))).is_ok());
        assert_eq!(
            validate_ambient_path(Path::new(&"x".repeat(MAX_DOCUMENT_PATH_BYTES + 1))),
            Err(ApprovedDocumentError::InvalidPath)
        );
        Ok(())
    }

    #[test]
    fn capabilities_are_exactly_read_only_bounded_and_non_enumerating() {
        let capabilities = ApprovedDocumentReader::new().capabilities();
        assert_eq!(capabilities.available(), cfg!(unix));
        assert_eq!(capabilities.read_only(), cfg!(unix));
        assert!(!capabilities.supports_enumeration());
        assert!(!capabilities.supports_artifact_write());
        assert_eq!(
            capabilities.maximum_raw_request_bytes(),
            if cfg!(unix) {
                MAX_DOCUMENT_RAW_REQUEST_BYTES
            } else {
                0
            }
        );
    }

    #[test]
    fn exact_formats_and_bounded_utf8_content_are_enforced(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let directory = TestDirectory::new()?;
        let exact = directory.path().join("exact.txt");
        fs::write(&exact, vec![b'x'; MAX_DOCUMENT_BYTES])?;
        let target = validate_direct_target(&exact)?;
        assert_eq!(
            read_validated_file(&target.path, target.identity)?.len(),
            MAX_DOCUMENT_BYTES
        );
        assert_eq!(
            format_for_path(&target.path),
            Ok(ApprovedDocumentFormat::Utf8Text)
        );

        let markdown = directory.path().join("notes.md");
        fs::write(&markdown, "# Heading\n")?;
        assert_eq!(
            format_for_path(&markdown),
            Ok(ApprovedDocumentFormat::Markdown)
        );
        for name in ["upper.MD", "unknown.rtf", "missing"] {
            assert_eq!(
                format_for_path(&directory.path().join(name)),
                Err(ApprovedDocumentError::UnsupportedFormat)
            );
        }

        let oversized = directory.path().join("oversized.txt");
        fs::write(&oversized, vec![b'x'; MAX_DOCUMENT_BYTES + 1])?;
        let target = validate_direct_target(&oversized)?;
        assert_eq!(
            read_validated_file(&target.path, target.identity),
            Err(ApprovedDocumentError::DocumentTooLarge)
        );

        let invalid_utf8 = directory.path().join("invalid.txt");
        fs::write(&invalid_utf8, [0xff])?;
        let target = validate_direct_target(&invalid_utf8)?;
        assert_eq!(
            read_validated_file(&target.path, target.identity),
            Err(ApprovedDocumentError::InvalidUtf8)
        );

        let nul = directory.path().join("nul.txt");
        fs::write(&nul, b"before\0after")?;
        let target = validate_direct_target(&nul)?;
        assert_eq!(
            read_validated_file(&target.path, target.identity),
            Err(ApprovedDocumentError::ProhibitedNul)
        );

        let control = directory.path().join("control.txt");
        fs::write(&control, b"before\x1bafter")?;
        let target = validate_direct_target(&control)?;
        assert_eq!(
            read_validated_file(&target.path, target.identity),
            Err(ApprovedDocumentError::ProhibitedControlCharacter)
        );
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn symlink_and_hard_link_targets_fail_closed() -> Result<(), Box<dyn std::error::Error>> {
        use std::os::unix::fs::symlink;

        let directory = TestDirectory::new()?;
        let original = directory.path().join("original.txt");
        fs::write(&original, "approved")?;

        let symlink_path = directory.path().join("symlink.txt");
        symlink(&original, &symlink_path)?;
        assert!(matches!(
            validate_direct_target(&symlink_path),
            Err(ApprovedDocumentError::SymlinkRejected)
        ));

        let hard_link = directory.path().join("hard-link.txt");
        fs::hard_link(&original, &hard_link)?;
        assert!(matches!(
            validate_direct_target(&original),
            Err(ApprovedDocumentError::HardLinkRejected)
        ));
        assert!(matches!(
            validate_direct_target(&hard_link),
            Err(ApprovedDocumentError::HardLinkRejected)
        ));

        let nested = directory.path().join("nested");
        fs::create_dir(&nested)?;
        fs::write(nested.join("member.md"), "approved member")?;
        let linked_directory = directory.path().join("linked-directory");
        symlink(&nested, &linked_directory)?;
        let attribution = live_personal_attribution()?;
        let root_task_id = attribution.root_task_id().clone();
        let mut reader = ApprovedDocumentReader::new();
        let root_id = reader.register_root(
            root_task_id.clone(),
            directory.path(),
            ApplicationDocumentControlProof::for_test(),
        )?;
        assert!(matches!(
            reader.register_root_member(
                root_task_id,
                &root_id,
                ApprovedRelativePath::new("linked-directory/member.md")?,
                ApplicationDocumentControlProof::for_test(),
            ),
            Err(ApprovedDocumentError::SymlinkRejected)
        ));
        Ok(())
    }

    #[test]
    fn replacement_or_in_place_change_invalidates_registered_identity(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let directory = TestDirectory::new()?;
        let path = directory.path().join("document.md");
        fs::write(&path, "original")?;
        let registered = validate_direct_target(&path)?;

        fs::write(&path, "changed-content")?;
        assert_eq!(
            read_validated_file(&registered.path, registered.identity),
            Err(ApprovedDocumentError::PathIdentityChanged)
        );
        Ok(())
    }

    #[test]
    fn registered_root_components_and_during_read_changes_fail_closed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let directory = TestDirectory::new()?;
        let root_path = directory.path().join("root");
        let nested = root_path.join("nested");
        fs::create_dir_all(&nested)?;
        let member = nested.join("member.txt");
        fs::write(&member, "registered member")?;

        let attribution = live_personal_attribution()?;
        let root_task_id = attribution.root_task_id().clone();
        let mut reader = ApprovedDocumentReader::new();
        let root_id = reader.register_root(
            root_task_id.clone(),
            &root_path,
            ApplicationDocumentControlProof::for_test(),
        )?;
        let document_id = reader.register_root_member(
            root_task_id,
            &root_id,
            ApprovedRelativePath::new("nested/member.txt")?,
            ApplicationDocumentControlProof::for_test(),
        )?;
        let grant = DocumentAccessGrant::from_live_attribution(
            attribution,
            document_id,
            AgentId::KnowledgeDocument,
            DocumentOperation::Read,
            LiveDocumentAccessProof::for_test(),
        )?;

        fs::rename(&nested, root_path.join("old-nested"))?;
        fs::create_dir(&nested)?;
        fs::write(&member, "replacement member")?;
        assert!(matches!(
            reader.prepare_read(&grant),
            Err(ApprovedDocumentError::PathIdentityChanged)
        ));

        let raced = directory.path().join("raced.txt");
        fs::write(&raced, "before")?;
        let target = validate_direct_target(&raced)?;
        assert_eq!(
            read_validated_file_with_hook(&target.path, target.identity, || {
                fs::write(&raced, "changed during read")
                    .map_err(|_| ApprovedDocumentError::ReadFailed)
            }),
            Err(ApprovedDocumentError::PathIdentityChanged)
        );

        #[cfg(unix)]
        {
            use std::{
                fs::FileTimes,
                time::{Duration, SystemTime},
            };

            let same_size = directory.path().join("same-size.txt");
            fs::write(&same_size, "same-size")?;
            let target = validate_direct_target(&same_size)?;
            assert_eq!(
                read_validated_file_with_hook(&target.path, target.identity, || {
                    fs::write(&same_size, "new-value")
                        .map_err(|_| ApprovedDocumentError::ReadFailed)?;
                    let file = fs::OpenOptions::new()
                        .write(true)
                        .open(&same_size)
                        .map_err(|_| ApprovedDocumentError::ReadFailed)?;
                    file.set_times(
                        FileTimes::new()
                            .set_modified(SystemTime::UNIX_EPOCH + Duration::from_secs(1)),
                    )
                    .map_err(|_| ApprovedDocumentError::ReadFailed)
                }),
                Err(ApprovedDocumentError::PathIdentityChanged)
            );

            let final_path = directory.path().join("final-path.txt");
            let moved_path = directory.path().join("moved-open-handle.txt");
            fs::write(&final_path, "original")?;
            let target = validate_direct_target(&final_path)?;
            assert_eq!(
                read_validated_file_with_hook(&target.path, target.identity, || {
                    fs::rename(&final_path, &moved_path)
                        .and_then(|()| fs::write(&final_path, "replaced"))
                        .map_err(|_| ApprovedDocumentError::ReadFailed)
                }),
                Err(ApprovedDocumentError::PathIdentityChanged)
            );
        }
        Ok(())
    }

    #[test]
    fn root_and_document_capacity_failures_leave_existing_entries_unchanged(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let directory = TestDirectory::new()?;
        let attribution = live_personal_attribution()?;
        let root_task_id = attribution.root_task_id().clone();
        let mut reader = ApprovedDocumentReader::new();

        for index in 0..MAX_APPROVED_ROOTS {
            let path = directory.path().join(format!("root-{index}"));
            fs::create_dir(&path)?;
            reader.register_root(
                root_task_id.clone(),
                path,
                ApplicationDocumentControlProof::for_test(),
            )?;
        }
        let extra_root = directory.path().join("extra-root");
        fs::create_dir(&extra_root)?;
        assert!(matches!(
            reader.register_root(
                root_task_id.clone(),
                extra_root,
                ApplicationDocumentControlProof::for_test(),
            ),
            Err(ApprovedDocumentError::RootLimitExceeded)
        ));
        assert_eq!(reader.roots.len(), MAX_APPROVED_ROOTS);

        for index in 0..MAX_APPROVED_DOCUMENTS {
            let path = directory.path().join(format!("document-{index}.txt"));
            fs::write(&path, format!("document {index}"))?;
            reader.register_document(
                root_task_id.clone(),
                ApprovedDocumentSource::UserSelectedFile,
                path,
                ApplicationDocumentControlProof::for_test(),
            )?;
        }
        let extra_document = directory.path().join("extra-document.txt");
        fs::write(&extra_document, "extra document")?;
        assert!(matches!(
            reader.register_document(
                root_task_id,
                ApprovedDocumentSource::UserSelectedFile,
                extra_document,
                ApplicationDocumentControlProof::for_test(),
            ),
            Err(ApprovedDocumentError::DocumentLimitExceeded)
        ));
        assert_eq!(reader.documents.len(), MAX_APPROVED_DOCUMENTS);
        Ok(())
    }

    #[test]
    fn selected_reference_reserve_abort_commit_and_replay_are_linear(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let directory = TestDirectory::new()?;
        let path = directory.path().join("selected.md");
        fs::write(&path, "# Approved\nUntrusted document content.")?;
        let attribution = live_personal_attribution()?;
        let root_task_id = attribution.root_task_id().clone();
        let mut reader = ApprovedDocumentReader::new();
        let document_id = reader.register_document(
            root_task_id,
            ApprovedDocumentSource::UserSelectedFile,
            &path,
            ApplicationDocumentControlProof::for_test(),
        )?;
        let grant = DocumentAccessGrant::from_live_attribution(
            attribution,
            document_id.clone(),
            AgentId::KnowledgeDocument,
            DocumentOperation::Summarize,
            LiveDocumentAccessProof::for_test(),
        )?;

        let prepared = reader.prepare_read(&grant)?;
        assert_eq!(
            prepared.content(),
            "# Approved\nUntrusted document content."
        );
        assert_eq!(prepared.source(), ApprovedDocumentSource::UserSelectedFile);
        assert_eq!(prepared.descriptor().document_id(), &document_id);
        assert!(matches!(
            reader.prepare_read(&grant),
            Err(ApprovedDocumentError::ReferenceReserved)
        ));
        reader.abort_read(prepared);

        let prepared = reader.prepare_read(&grant)?;
        let commit = reader.commit_read(prepared);
        assert_eq!(commit.descriptor().document_id(), &document_id);
        assert_eq!(
            commit.descriptor().operation(),
            DocumentOperation::Summarize
        );
        assert_eq!(commit.byte_count(), fs::metadata(&path)?.len() as usize);
        assert!(matches!(
            reader.prepare_read(&grant),
            Err(ApprovedDocumentError::ReferenceConsumed)
        ));
        assert_eq!(
            reader.revoke(&document_id, ApplicationDocumentControlProof::for_test()),
            Err(ApprovedDocumentError::ReferenceConsumed)
        );
        assert!(reader
            .documents
            .get(&document_id)
            .is_some_and(|entry| entry.target.is_none()));
        reader.cleanup_document(&document_id);
        assert!(matches!(
            reader.prepare_read(&grant),
            Err(ApprovedDocumentError::DocumentNotFound)
        ));
        Ok(())
    }

    #[test]
    fn approved_root_member_succeeds_and_revocation_blocks_read(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let directory = TestDirectory::new()?;
        let nested = directory.path().join("nested");
        fs::create_dir(&nested)?;
        fs::write(nested.join("facts.txt"), "fact one\nfact two")?;
        let attribution = live_personal_attribution()?;
        let root_task_id = attribution.root_task_id().clone();
        let mut reader = ApprovedDocumentReader::new();
        let root_id = reader.register_root(
            root_task_id.clone(),
            directory.path(),
            ApplicationDocumentControlProof::for_test(),
        )?;
        let document_id = reader.register_root_member(
            root_task_id.clone(),
            &root_id,
            ApprovedRelativePath::new("nested/facts.txt")?,
            ApplicationDocumentControlProof::for_test(),
        )?;
        let grant = DocumentAccessGrant::from_live_attribution(
            attribution,
            document_id.clone(),
            AgentId::KnowledgeDocument,
            DocumentOperation::ExtractFacts,
            LiveDocumentAccessProof::for_test(),
        )?;
        let prepared = reader.prepare_read(&grant)?;
        assert_eq!(prepared.content(), "fact one\nfact two");
        reader.abort_read(prepared);
        reader.revoke(&document_id, ApplicationDocumentControlProof::for_test())?;
        assert!(reader
            .documents
            .get(&document_id)
            .is_some_and(|entry| entry.target.is_none()));
        assert!(matches!(
            reader.prepare_read(&grant),
            Err(ApprovedDocumentError::ReferenceRevoked)
        ));
        reader.cleanup_root(&root_task_id);
        assert!(matches!(
            reader.prepare_read(&grant),
            Err(ApprovedDocumentError::DocumentNotFound)
        ));
        Ok(())
    }
}
