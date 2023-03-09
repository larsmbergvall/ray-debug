# ray-debug

[![Rust](https://github.com/larsmbergvall/ray-debug/actions/workflows/rust.yml/badge.svg)](https://github.com/larsmbergvall/ray-debug/actions/workflows/rust.yml)

A Rust adapter for Spatie's fantastic debugging tool, [Ray](https://spatie.be/products/ray).

**This is a work in progress, so breaking changes are likely to occur! Also, not all Ray features are yet implemented!**

## Setup

By default, Ray is listening on http://localhost:23517. If you need to use another host or port, you can specify env
variables for this:

```dotenv
RAY_HOST=http://localhost
RAY_PORT=23517
```

## Usage

Only basic debugging is implemented at the moment, so you can use it like:

```rust
use ray_debug::{ray, ray_log};

// ...
fn do_stuff() {
    // ...
    ray(&some_struct);
    ray_log("foo");
    
    // To set color:
    ray(&some_struct).unwrap().orange();
    ray_log("foo").unwrap().green();
}
```
