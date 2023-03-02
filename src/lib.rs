use crate::ray_request::RayRequest;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;

mod ray_color;
mod ray_request;

pub async fn ray<T: Serialize>(value: T) -> Result<RayRequest, Box<dyn Error>> {
    RayRequest::log(value, None).send().await
}

fn is_valid_json() -> bool {
    false
}

fn get_json<T: Serialize>(value: T, pretty_print: bool) -> String {
    match pretty_print {
        true => match serde_json::to_string_pretty(&value) {
            Ok(json) => json,
            Err(e) => panic!("{}", e),
        },
        false => match serde_json::to_string(&value) {
            Ok(json) => json,
            Err(e) => panic!("{}", e),
        },
    }
}

fn content_or_panic<T: Serialize>(value: T, pretty_print: bool) -> HashMap<String, Vec<String>> {
    let mut content = HashMap::new();
    content.insert("values".to_string(), vec![get_json(&value, pretty_print)]);

    content
}

mod meta;
mod origin;
mod payloads;
#[cfg(test)]
mod tests;
