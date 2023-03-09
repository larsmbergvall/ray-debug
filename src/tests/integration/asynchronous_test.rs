use crate::asynchronous;
use crate::tests::TestUser;

#[ignore]
#[tokio::test]
async fn async_ray_log_works() {
    asynchronous::ray_log("async_ray_log_works()")
        .await
        .unwrap();
}

#[ignore]
#[tokio::test]
async fn async_ray_works() {
    // Used just for testing sending requests...
    let user = TestUser {
        name: "::name::".into(),
        age: 42,
        country: "SE".into(),
        email: "async_ray_works@localhost.example".into(),
    };

    asynchronous::ray(&user).await.unwrap();
}
