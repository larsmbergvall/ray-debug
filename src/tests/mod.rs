use crate::ray;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
async fn asdf() {
    match serde_json::from_str::<Value>("123") {
        Ok(e) => println!("It was ok: {:?}", e),
        Err(e) => println!("it failed: {:?}", e),
    };
    // ray("foo").await.unwrap();
    // ray("Hej").await.unwrap();
    // ray("Hello".to_string()).await.unwrap();
    // ray(123).await.unwrap();
    //
    // let t = TestUser {
    //     name: "SlangConny".to_string(),
    //     age: 30,
    //     country: "SWE".to_string(),
    //     email: "slang@conny.se".to_string(),
    // };
    //
    // ray(&t).await.unwrap();
}
