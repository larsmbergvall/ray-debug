use crate::payloads::log_payload::LogPayload;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Payload {
    Log(LogPayload),
}
