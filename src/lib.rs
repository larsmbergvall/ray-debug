use std::error::Error;

use serde::Serialize;

use crate::ray_request::RayRequest;

mod ray_color;
mod ray_request;

pub fn ray_log<T: Into<String>>(value: T) -> Result<RayRequest, Box<dyn Error>> {
    RayRequest::log(value.into(), None).send()
}

pub fn ray<T: Serialize>(value: &T) -> Result<RayRequest, Box<dyn Error>> {
    let json = helpers::get_json(value, true);

    RayRequest::html(helpers::json_to_html(json), None).send()
}

pub mod asynchronous;
mod helpers;
mod meta;
mod origin;
mod payloads;
#[cfg(test)]
mod tests;
