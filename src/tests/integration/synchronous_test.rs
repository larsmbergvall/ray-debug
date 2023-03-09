use crate::tests::{TestPost, TestUser};
use crate::{ray, ray_charles, ray_log};

// Ignore these tests by default, so they don't run in CI. They expect ray or ray-proxy
// to be running anyway so it's not practical.

#[ignore]
#[test]
fn sync_ray_log_works() {
    if let Err(e) = ray_log("sync_ray_log_works()") {
        panic!("{}", e);
    }
}

#[ignore]
#[test]
fn sync_ray_charles_works() {
    if let Err(e) = ray_charles() {
        panic!("{}", e);
    }
}

#[ignore]
#[test]
fn sync_ray_works() {
    let user = TestUser {
        name: "::name::".into(),
        age: 42,
        country: "SE".into(),
        email: "sync_ray_works@localhost.example".into(),
    };

    let post = TestPost {
        title: "Foo".to_string(),
        year: 1980,
        author: user,
    };

    if let Err(e) = ray(&[post]).unwrap().green() {
        panic!("{}", e);
    }
}
