use crate::payloads::html_payload::HtmlPayload;
use crate::payloads::log_payload::LogPayload;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Payload {
    Log(LogPayload),
    Html(HtmlPayload),
}
