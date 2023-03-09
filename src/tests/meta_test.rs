use crate::meta::Meta;
use rustc_version::version_meta;

#[test]
fn it_gets_correct_data() {
    let rustc_version = match version_meta() {
        Ok(version_meta) => version_meta.semver.to_string(),
        _ => panic!("Could not get version"),
    };

    let meta = Meta::default();

    let pkg_name = option_env!("CARGO_PKG_NAME").unwrap_or("should fail if it gets this value");
    let pkg_version = option_env!("CARGO_PKG_VERSION").unwrap_or("this too");

    assert_eq!(rustc_version, meta.rustc_version);
    assert_eq!(pkg_name, meta.project_name);
    assert_eq!(pkg_version, meta.project_version);
}
