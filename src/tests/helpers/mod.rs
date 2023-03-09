use crate::ray_request::RayRequest;
use serde_json::Value;

pub fn get_first_payload_as_json_value(request: &RayRequest) -> Value {
    let request_json_value =
        serde_json::to_value(request).expect("Failed to convert RayRequest to Value");

    let value = request_json_value
        .get("payloads")
        .expect("Failed to get payloads from RayRequest")
        .get(0)
        .expect("Failed to get first index of RayRequest -> payloads");

    value.clone()
}
