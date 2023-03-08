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
