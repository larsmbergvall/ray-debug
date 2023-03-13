use serde::{Deserialize, Serialize};

mod helpers;
mod integration;
mod meta_test;
mod origin_test;
mod ray_request_test;
mod rayable_test;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestUser {
    pub name: String,
    pub age: u8,
    pub country: String,
    pub email: String,
}

impl Default for TestUser {
    fn default() -> Self {
        Self {
            name: "John Doe".to_string(),
            age: 42,
            country: "SE".to_string(),
            email: "some_email@localhost.example".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestPost {
    pub author: TestUser,
    pub title: String,
    pub year: u16,
}
