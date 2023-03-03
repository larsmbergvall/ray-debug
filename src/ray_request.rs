use crate::meta::Meta;
use crate::payloads::html_payload::HtmlPayload;
use crate::payloads::log_payload::LogPayload;
use crate::payloads::payload::Payload;
use crate::ray_color::RayColor;
use serde::Serialize;
use std::error::Error;

const HOST: &str = "http://127.0.0.1";
const PORT: u32 = 23517;

#[derive(Serialize, Debug)]
pub struct RayRequest {
    pub uuid: String,
    pub payloads: Vec<Payload>,
    pub meta: Meta,
}

impl RayRequest {
    pub fn new(payloads: Vec<Payload>, meta: Meta, uuid: Option<String>) -> Self {
        let uuid = match uuid {
            Some(v) => v,
            None => uuid::Uuid::new_v4().to_string(),
        };

        Self {
            uuid,
            payloads,
            meta,
        }
    }

    pub fn log<T: Into<String>>(value: T, uuid: Option<String>) -> Self {
        let payload = LogPayload::new(value);

        Self::new(vec![Payload::Log(payload)], Meta::new(), uuid)
    }

    pub fn html<T: Into<String>>(html: T, uuid: Option<String>) -> Self {
        let payload = HtmlPayload::new(html);

        Self::new(vec![Payload::Html(payload)], Meta::new(), uuid)
    }

    pub fn green(&self) -> &Self {
        self.color(RayColor::Green)
    }

    pub fn orange(&self) -> &Self {
        self.color(RayColor::Orange)
    }

    pub fn red(&self) -> &Self {
        self.color(RayColor::Red)
    }

    pub fn purple(&self) -> &Self {
        self.color(RayColor::Purple)
    }

    pub fn blue(&self) -> &Self {
        self.color(RayColor::Blue)
    }

    pub fn gray(&self) -> &Self {
        self.color(RayColor::Gray)
    }

    fn color(&self, _color: RayColor) -> &Self {
        todo!();
    }

    pub fn charles(&self) -> &Self {
        todo!();
    }

    pub async fn send(self) -> Result<Self, Box<dyn Error>> {
        let request = reqwest::Client::new();
        let result = request
            .post(format!("{}:{}", HOST, PORT))
            .json(&self)
            .send()
            .await?;

        dbg!(result);

        Ok(self)
    }
}
