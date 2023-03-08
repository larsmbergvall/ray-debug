use crate::meta::Meta;
use crate::origin::Origin;
use crate::payloads::html_payload::HtmlPayload;
use crate::payloads::log_payload::LogPayload;
use crate::payloads::payload::Payload;
use crate::ray_request::RayRequest;
use crate::tests::helpers::expected_json;

#[test]
fn it_has_correct_structure_when_logging_text() {
    let payload = LogPayload::new("foo");
    let payload_json = r#"
    {
      "type": "log",
      "content": {
        "values": [
          "foo"
        ]
      },
      "origin": {
        "function_name": "test_function",
        "file": "file_name.rs",
        "line_number": 0,
        "hostname": "test_hostname"
      }
    }
"#;

    let expected_json = expected_json(Some("some-totally-real-uuid"), payload_json);

    let request = RayRequest::new(
        vec![Payload::Log(payload)],
        Meta::test(),
        Some("some-totally-real-uuid".to_string()),
    )
    .with_origin(Origin::test());

    let json = serde_json::to_string_pretty(&request).unwrap();

    assert_eq!(json, expected_json);
}

#[test]
fn it_has_correct_structure_when_logging_struct() {
    let payload = HtmlPayload::new("<div>foo</div>");
    let payload_json = r#"
    {
      "type": "custom",
      "content": {
        "content": "<div>foo</div>",
        "label": "HTML"
      },
      "origin": {
        "function_name": "test_function",
        "file": "file_name.rs",
        "line_number": 0,
        "hostname": "test_hostname"
      }
    }
"#;

    let expected_json = expected_json(Some("some-totally-real-uuid"), payload_json);

    let request = RayRequest::new(
        vec![Payload::Html(payload)],
        Meta::test(),
        Some("some-totally-real-uuid".to_string()),
    )
    .with_origin(Origin::test());

    let json = serde_json::to_string_pretty(&request).unwrap();

    assert_eq!(json, expected_json);
}
