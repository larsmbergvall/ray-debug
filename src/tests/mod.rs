use serde::{Deserialize, Serialize};

mod integration;
mod meta_test;
mod origin_test;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestUser {
    pub name: String,
    pub age: u8,
    pub country: String,
    pub email: String,
}
