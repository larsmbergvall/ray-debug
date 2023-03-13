use serde::Serialize;
use serde_json::{Number, Value};

use crate::helpers;
use crate::ray_request::RayRequest;

pub trait Rayable {
    fn into_ray_request(self) -> RayRequest;
}

impl<T: serde::Serialize> Rayable for &T {
    fn into_ray_request(self) -> RayRequest {
        to_html_request(&self)
    }
}

impl Rayable for String {
    fn into_ray_request(self) -> RayRequest {
        to_html_request(&self)
    }
}

impl Rayable for &str {
    fn into_ray_request(self) -> RayRequest {
        RayRequest::log(self, None)
    }
}

impl Rayable for Value {
    fn into_ray_request(self) -> RayRequest {
        to_html_request(&self)
    }
}

impl Rayable for Number {
    fn into_ray_request(self) -> RayRequest {
        to_html_request(&self)
    }
}

impl Rayable for i64 {
    fn into_ray_request(self) -> RayRequest {
        to_html_request(&self)
    }
}

impl<T: serde::Serialize> Rayable for Vec<T> {
    fn into_ray_request(self) -> RayRequest {
        to_html_request(&self)
    }
}

fn to_html_request<T: Serialize>(value: &T) -> RayRequest {
    let json = helpers::get_json(value, false);
    let serde_value = serde_json::from_str(&json).unwrap();

    RayRequest::html(helpers::value_to_html(&serde_value), None)
}
