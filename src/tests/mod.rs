use serde::{Deserialize, Serialize};

mod helpers;
mod integration;
mod meta_test;
mod origin_test;
mod ray_request_test;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestUser {
    pub name: String,
    pub age: u8,
    pub country: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestPost {
    pub author: TestUser,
    pub title: String,
    pub year: u16,
}
