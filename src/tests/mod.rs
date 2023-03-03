use serde::{Deserialize, Serialize};

mod meta_test;
mod origin_test;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestUser {
    pub name: String,
    pub age: u8,
    pub country: String,
    pub email: String,
}

#[tokio::test]
async fn it_works() {
    // Used just for testing sending requests...
    todo!()
}
