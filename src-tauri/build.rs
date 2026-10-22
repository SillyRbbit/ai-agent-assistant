use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=voice/CortexaSpeechHelper.swift");
    println!("cargo:rerun-if-changed=voice/Helper-Info.plist");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        let root = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap_or_default());
        let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_default());
        let helper = out.join("CortexaSpeechHelper");
        let arch = match std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
            Ok("aarch64") => "arm64-apple-macos14.0",
            Ok("x86_64") => "x86_64-apple-macos14.0",
            _ => std::process::exit(1),
        };
        let status = Command::new("/usr/bin/xcrun")
            .args([
                "swiftc",
                "-swift-version",
                "6",
                "-warnings-as-errors",
                "-parse-as-library",
                "-O",
            ])
            .args(["-target", arch])
            .arg(root.join("voice/CortexaSpeechHelper.swift"))
            .args([
                "-Xlinker",
                "-sectcreate",
                "-Xlinker",
                "__TEXT",
                "-Xlinker",
                "__info_plist",
                "-Xlinker",
            ])
            .arg(root.join("voice/Helper-Info.plist"))
            .arg("-o")
            .arg(&helper)
            .status();
        if !matches!(status, Ok(s) if s.success()) {
            std::process::exit(1);
        }
        let bundle = root.join("voice-bin");
        if std::fs::create_dir_all(&bundle).is_err()
            || std::fs::copy(&helper, bundle.join("CortexaSpeechHelper")).is_err()
        {
            std::process::exit(1);
        }
        println!(
            "cargo:rustc-env=CORTEXA_SPEECH_HELPER_DEV={}",
            helper.display()
        );
    }
    tauri_build::build();
}
