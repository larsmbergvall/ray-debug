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

#[ignore]
#[test]
fn generic_ray_works_with_serializable_struct() {
    let user = TestUser {
        name: "::name::".into(),
        age: 42,
        country: "SE".into(),
        email: "sync_ray_works@localhost.example".into(),
    };

    if let Err(e) = ray(&user).unwrap().green() {
        panic!("{}", e);
    }

    if let Err(e) = ray("&str").unwrap().green() {
        panic!("{}", e);
    }

    let a_string_ref = String::from("&a_string");
    let a_string = String::from("a_string");

    if let Err(e) = ray(&a_string_ref).unwrap().green() {
        panic!("{}", e);
    }

    if let Err(e) = ray(a_string).unwrap().green() {
        panic!("{}", e);
    }

    if let Err(e) = ray(123).unwrap().green() {
        panic!("{}", e);
    }

    let tiny_int: u8 = 5;

    if let Err(e) = ray(&tiny_int).unwrap().green() {
        panic!("{}", e);
    }
}
