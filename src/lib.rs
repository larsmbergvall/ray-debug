use std::error::Error;

use serde::Serialize;

use crate::ray_request::RayRequest;

mod ray_color;
mod ray_request;

pub fn ray_log<T: Into<String>>(value: T) -> Result<RayRequest, Box<dyn Error>> {
    RayRequest::log(value.into(), None).send()
}

pub fn ray<T: Serialize>(value: &T) -> Result<RayRequest, Box<dyn Error>> {
    let json = helpers::get_json(value, false);
    let serde_value = serde_json::from_str(&json).unwrap();

    RayRequest::html(helpers::value_to_html(&serde_value), None).send()
}

pub mod asynchronous;
mod helpers;
mod meta;
mod origin;
mod payloads;
#[cfg(test)]
mod tests;
