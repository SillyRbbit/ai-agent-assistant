use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub architecture: &'static str,
    pub environment: &'static str,
    pub name: &'static str,
    pub secure_core: bool,
    pub target: &'static str,
    pub version: &'static str,
}

#[must_use]
pub fn current_app_info() -> AppInfo {
    AppInfo {
        architecture: std::env::consts::ARCH,
        environment: if cfg!(debug_assertions) {
            "development"
        } else {
            "production"
        },
        name: "AI Agent Assistant",
        secure_core: true,
        target: std::env::consts::OS,
        version: env!("CARGO_PKG_VERSION"),
    }
}

#[tauri::command]
pub(crate) fn get_app_info() -> AppInfo {
    current_app_info()
}

#[cfg(test)]
mod tests {
    use super::current_app_info;

    #[test]
    fn reports_expected_application_identity() {
        let info = current_app_info();

        assert_eq!(info.name, "AI Agent Assistant");
        assert!(!info.version.is_empty());
        assert!(info.secure_core);
    }
}
