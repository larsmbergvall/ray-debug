use crate::origin::Origin;
use crate::payloads::payload;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct LogPayload {
    #[serde(rename = "type")]
    payload_type: String,
    content: HashMap<String, Vec<String>>,
    origin: Origin,
}

impl LogPayload {
    pub fn new<T: Serialize>(value: T) -> Self {
        Self {
            payload_type: "log".to_string(),
            content: payload::content_or_panic(value, false),
            origin: Origin::capture(Some(5)),
        }
    }
}
