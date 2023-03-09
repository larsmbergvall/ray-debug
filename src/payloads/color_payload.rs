use crate::origin::Origin;
use crate::ray_color::RayColor;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct ColorPayload {
    #[serde(rename = "type")]
    payload_type: String,
    content: HashMap<String, String>,
    pub origin: Origin,
}

impl ColorPayload {
    pub fn new(color: RayColor) -> Self {
        let mut content = HashMap::new();
        content.insert("color".to_string(), color.to_string());

        Self {
            payload_type: "color".to_string(),
            content,
            origin: Origin::capture(None),
        }
    }
}
