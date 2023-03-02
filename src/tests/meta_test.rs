use crate::meta::Meta;
use rustc_version::version_meta;

#[test]
fn it_gets_correct_data() {
    let version = version_meta();
    let expected = match version {
        Ok(version_meta) => version_meta.semver.to_string(),
        _ => panic!("Could not get version"),
    };

    let meta = Meta::new();

    assert_eq!(expected, meta.rustc_version);
    assert_eq!("ray", meta.project_name);
    assert_eq!("0.1.0", meta.project_version);
}
