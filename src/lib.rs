use crate::ray_request::RayRequest;
use serde::Serialize;
use std::error::Error;

mod ray_color;
mod ray_request;

pub async fn ray_log<T: Into<String>>(value: T) -> Result<RayRequest, Box<dyn Error>> {
    RayRequest::log(value.into(), None).send().await
}

pub async fn ray<T: Serialize>(value: &T) -> Result<RayRequest, Box<dyn Error>> {
    let json = get_json(value, true);

    RayRequest::html(helpers::json_to_html(json), None)
        .send()
        .await
}

fn get_json<T: Serialize>(value: &T, pretty_print: bool) -> String {
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

mod helpers;
mod meta;
mod origin;
mod payloads;
#[cfg(test)]
mod tests;
