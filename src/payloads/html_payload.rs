use crate::origin::Origin;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct HtmlPayload {
    #[serde(rename = "type")]
    payload_type: String,

    content: HashMap<String, String>,
    pub origin: Origin,
}

impl HtmlPayload {
    pub fn new<T: Into<String>>(value: T) -> Self {
        let mut content = HashMap::new();
        content.insert("content".to_string(), value.into());
        content.insert("label".to_string(), "HTML".to_string());

        Self {
            payload_type: "custom".to_string(),
            origin: Origin::capture(None),
            content,
        }
    }
}
