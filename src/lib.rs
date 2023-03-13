use std::error::Error;

use crate::meta::Meta;
use crate::payloads::log_payload::LogPayload;
use crate::payloads::payload::Payload;

use crate::ray_request::RayRequest;
use crate::rayable::Rayable;

mod ray_color;
mod ray_request;

#[deprecated(since = "0.0.4", note = "This will be removed; use ray() instead")]
pub fn ray_log<T: Into<String>>(value: T) -> Result<RayRequest, Box<dyn Error>> {
    RayRequest::log(value.into(), None).send()
}

pub fn ray<T: Rayable>(value: T) -> Result<RayRequest, Box<dyn Error>> {
    let request: RayRequest = value.into_ray_request();

    request.send()
}

pub fn ray_charles() -> Result<RayRequest, Box<dyn Error>> {
    RayRequest::new(
        vec![Payload::Log(LogPayload::charles())],
        Meta::default(),
        None,
    )
    .send()
}

pub mod asynchronous;
mod helpers;
mod meta;
mod origin;
mod payloads;
pub mod rayable;
#[cfg(test)]
mod tests;
