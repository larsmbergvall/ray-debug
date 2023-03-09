use crate::origin::Origin;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct LogPayload {
    #[serde(rename = "type")]
    payload_type: String,
    content: HashMap<String, Vec<String>>,
    pub origin: Origin,
}

impl LogPayload {
    pub fn new<T: Into<String>>(value: T) -> Self {
        let mut content = HashMap::new();
        content.insert("values".to_string(), vec![value.into()]);

        Self {
            payload_type: "log".to_string(),
            content,
            origin: Origin::capture(Some(5)),
        }
    }
}
