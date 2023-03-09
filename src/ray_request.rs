use crate::meta::Meta;
use crate::origin::Origin;
use crate::payloads::color_payload::ColorPayload;
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

        Self::new(vec![Payload::Log(payload)], Meta::default(), uuid)
    }

    pub fn html<T: Into<String>>(html: T, uuid: Option<String>) -> Self {
        let payload = HtmlPayload::new(html);

        Self::new(vec![Payload::Html(payload)], Meta::default(), uuid)
    }

    pub fn green(self) -> Result<Self, Box<dyn Error>> {
        self.color(RayColor::Green)
    }

    pub fn orange(self) -> Result<Self, Box<dyn Error>> {
        self.color(RayColor::Orange)
    }

    pub fn red(self) -> Result<Self, Box<dyn Error>> {
        self.color(RayColor::Red)
    }

    pub fn purple(self) -> Result<Self, Box<dyn Error>> {
        self.color(RayColor::Purple)
    }

    pub fn blue(self) -> Result<Self, Box<dyn Error>> {
        self.color(RayColor::Blue)
    }

    pub fn gray(self) -> Result<Self, Box<dyn Error>> {
        self.color(RayColor::Gray)
    }

    fn color(mut self, color: RayColor) -> Result<Self, Box<dyn Error>> {
        self.payloads = vec![Payload::Color(ColorPayload::new(color))];

        self.send()
    }

    pub fn charles(&self) -> &Self {
        todo!();
    }

    pub fn with_meta(mut self, meta: Meta) -> Self {
        self.meta = meta;

        self
    }

    pub fn with_origin(mut self, origin: Origin) -> Self {
        for payload in &mut self.payloads {
            payload.set_origin(origin.clone());
        }

        self
    }

    pub fn send(self) -> Result<Self, Box<dyn Error>> {
        let request = reqwest::blocking::Client::new();

        let result = request.post(Self::url()).json(&self).send();

        if result.is_ok() {
            return Ok(self);
        }

        Err(Box::new(result.err().unwrap()))
    }

    pub async fn send_async(self) -> Result<Self, Box<dyn Error>> {
        let request = reqwest::Client::new();

        match request.post(Self::url()).json(&self).send().await {
            Ok(_) => Ok(self),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn url() -> String {
        format!("{}:{}", HOST, PORT)
    }
}
