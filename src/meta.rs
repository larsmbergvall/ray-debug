use rustc_version::version_meta;
use serde::Serialize;

const PKG_NAME: Option<&str> = option_env!("CARGO_PKG_NAME");
const PKG_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Serialize, Debug)]
pub struct Meta {
    pub rustc_version: String,
    pub project_name: String,
    pub project_version: String,
}

impl Meta {
    pub fn new(rustc_version: String, project_name: String, project_version: String) -> Self {
        Self {
            rustc_version,
            project_name,
            project_version,
        }
    }

    pub fn test() -> Self {
        Self {
            rustc_version: "1.0".to_string(),
            project_name: "test_project".to_string(),
            project_version: "1.0".to_string(),
        }
    }
}

impl Default for Meta {
    fn default() -> Self {
        let version = version_meta();
        let rustc_version = match version {
            Ok(version_meta) => version_meta.semver.to_string(),
            _ => "Unknown".to_string(),
        };

        Self {
            rustc_version,
            project_name: PKG_NAME.unwrap_or("Unknown").to_string(),
            project_version: PKG_VERSION.unwrap_or("Unknown").to_string(),
        }
    }
}
