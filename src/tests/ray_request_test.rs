use crate::meta::Meta;
use crate::origin::Origin;
use crate::payloads::color_payload::ColorPayload;
use crate::payloads::html_payload::HtmlPayload;
use crate::payloads::log_payload::LogPayload;
use crate::payloads::payload::Payload;
use crate::ray_color::RayColor;
use crate::ray_request::RayRequest;
use crate::tests::helpers::get_first_payload_as_json_value;
use serde_json::json;

#[test]
fn it_has_correct_structure_when_logging_text() {
    let expected_json = json!({
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
    });

    let request = RayRequest::new(
        vec![Payload::Log(LogPayload::new("foo"))],
        Meta::test(),
        Some("some-totally-real-uuid".to_string()),
    )
    .with_origin(Origin::test());
    let request_payload_value = get_first_payload_as_json_value(&request);

    assert_eq!(request_payload_value, expected_json);
}

#[test]
fn it_has_correct_structure_when_doing_a_charles() {
    let expected_json = json!({
        "type": "log",
        "content": {
            "values": [
                "🎶 🎹 🎷 🕺"
            ]
        },
        "origin": {
            "function_name": "test_function",
            "file": "file_name.rs",
            "line_number": 0,
            "hostname": "test_hostname"
        }
    });

    let request = RayRequest::new(
        vec![Payload::Log(LogPayload::charles())],
        Meta::test(),
        Some("some-totally-real-uuid".to_string()),
    )
    .with_origin(Origin::test());
    let request_payload_value = get_first_payload_as_json_value(&request);

    assert_eq!(request_payload_value, expected_json);
}

#[test]
fn it_has_correct_structure_when_logging_struct() {
    let expected_json = json!({
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
    });

    let request = RayRequest::new(
        vec![Payload::Html(HtmlPayload::new("<div>foo</div>"))],
        Meta::test(),
        Some("some-totally-real-uuid".to_string()),
    )
    .with_origin(Origin::test());

    let request_payload_value = get_first_payload_as_json_value(&request);

    assert_eq!(request_payload_value, expected_json);
}

#[test]
fn it_has_correct_structure_when_changing_color_of_payload() {
    let expected_json = json!({
      "type": "color",
      "content": {
        "color": "green"
      },
      "origin": {
        "function_name": "test_function",
        "file": "file_name.rs",
        "line_number": 0,
        "hostname": "test_hostname"
      }
    });

    let request = RayRequest::new(
        vec![Payload::Color(ColorPayload::new(RayColor::Green))],
        Meta::test(),
        Some("some-totally-real-uuid".to_string()),
    )
    .with_origin(Origin::test());

    let request_payload_value = get_first_payload_as_json_value(&request);

    assert_eq!(request_payload_value, expected_json);
}
