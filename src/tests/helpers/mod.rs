pub fn expected_json<T: Into<String>>(uuid: Option<T>, payload_json: T) -> String {
    let uuid = match uuid {
        Some(v) => v.into(),
        None => "some-uuid".to_string(),
    };

    let expected_json = r#"{
  "uuid": "{UUID}",
  "payloads": [{PAYLOAD}  ],
  "meta": {
    "rustc_version": "1.0",
    "project_name": "test_project",
    "project_version": "1.0"
  }
}"#;

    expected_json
        .replace("{UUID}", &uuid)
        .replace("{PAYLOAD}", &payload_json.into())
}
