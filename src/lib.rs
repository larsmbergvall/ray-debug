use crate::origin::Origin;
use crate::ray_request::RayRequest;
use std::backtrace::Backtrace;

mod payload;
mod ray_color;
mod ray_request;

pub fn ray<T: serde::Serialize + ToString>(value: T) -> RayRequest {
    let trace = Backtrace::capture();

    // let origin = Origin::new()
    todo!()
}

mod origin;
#[cfg(test)]
mod tests;
