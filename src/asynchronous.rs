use crate::helpers;
use crate::ray_request::RayRequest;
use serde::Serialize;
use std::error::Error;

pub async fn ray_log<T: Into<String>>(value: T) -> Result<RayRequest, Box<dyn Error>> {
    RayRequest::log(value.into(), None).send_async().await
}

pub async fn ray<T: Serialize>(value: &T) -> Result<RayRequest, Box<dyn Error>> {
    let json = helpers::get_json(value, true);

    RayRequest::html(helpers::json_to_html(json), None)
        .send_async()
        .await
}
