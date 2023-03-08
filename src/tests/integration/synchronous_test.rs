use crate::tests::TestUser;
use crate::{ray, ray_log};

#[test]
fn sync_ray_log_works() {
    if let Err(e) = ray_log("sync_ray_log_works()") {
        panic!("{}", e);
    }
}

#[test]
fn sync_ray_works() {
    // Used just for testing sending requests...
    let user = TestUser {
        name: "::name::".into(),
        age: 42,
        country: "SE".into(),
        email: "sync_ray_works@localhost.example".into(),
    };
    let user2 = TestUser {
        name: "::name2::".into(),
        age: 24,
        country: "SE".into(),
        email: "sync_ray_works_for_another_user@localhost.example".into(),
    };

    if let Err(e) = ray(&[user, user2]) {
        panic!("{}", e);
    }
}
