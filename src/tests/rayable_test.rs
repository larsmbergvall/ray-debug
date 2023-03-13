use crate::payloads::payload::Payload;
use crate::ray_request::RayRequest;
use crate::rayable::Rayable;
use crate::tests::TestUser;
use std::any::{Any, TypeId};

#[test]
fn it_can_convert_string_to_ray_request() {
    let string_value = String::from("foo");

    let request = assert_it_can_convert_value(string_value);

    match request.payloads.first().unwrap() {
        Payload::Html(_) => {}
        _ => panic!("Payload is not HTML type!"),
    };
}

#[test]
fn it_can_convert_string_slice_to_ray_request() {
    let request = assert_it_can_convert_value("foo");

    match request.payloads.first().unwrap() {
        Payload::Log(_) => {}
        _ => panic!("Payload is not Log type!"),
    };
}

#[test]
fn it_can_convert_number_to_ray_request() {
    let request = assert_it_can_convert_value(123);

    match request.payloads.first().unwrap() {
        Payload::Html(_) => {}
        _ => panic!("Payload is not HTML type!"),
    };
}

#[test]
fn it_can_convert_serializable_struct_to_ray_request() {
    let user = TestUser::default();

    let request = assert_it_can_convert_value(&user);

    match request.payloads.first().unwrap() {
        Payload::Html(_) => {}
        _ => panic!("Payload is not HTML type!"),
    };
}

#[test]
fn it_can_convert_vec_of_serializable_structs_to_ray_request() {
    let user = TestUser::default();
    let user2 = TestUser::default();

    let vec = vec![user, user2];

    let request = assert_it_can_convert_value(&vec);

    match request.payloads.first().unwrap() {
        Payload::Html(_) => {}
        _ => panic!("Payload is not HTML type!"),
    };
}

fn assert_it_can_convert_value<T: Rayable>(value: T) -> RayRequest {
    let request = rayable(value);

    assert_eq!(request.type_id(), TypeId::of::<RayRequest>());

    request
}

fn rayable<T: Rayable>(value: T) -> RayRequest {
    value.into_ray_request()
}
