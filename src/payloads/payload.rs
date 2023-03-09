use crate::origin::Origin;
use crate::payloads::color_payload::ColorPayload;
use crate::payloads::html_payload::HtmlPayload;
use crate::payloads::log_payload::LogPayload;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Payload {
    Log(LogPayload),
    Html(HtmlPayload),
    Color(ColorPayload),
}

impl Payload {
    pub fn set_origin(&mut self, origin: Origin) {
        match self {
            Payload::Log(v) => v.origin = origin,
            Payload::Html(v) => v.origin = origin,
            Payload::Color(v) => v.origin = origin,
        }
    }
}
