# ray-debug

A Rust adapter for Spatie's fantastic debugging tool, [Ray](https://spatie.be/products/ray).

**This is a work in progress, so breaking changes are likely to occur!**

Only basic debugging is implemented at the moment, so you can use it like:

```rust
use ray_rs::{ray, ray_log};

// ...
fn do_stuff() {
    // ...
    ray(&some_struct);
    ray_log("foo");
}
```