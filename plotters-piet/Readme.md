# Plotters Piet

A [Piet](https://crates.io/crates/piet) backend for [Plotters](https://crates.io/crates/plotters). This lets you draw plots on a Piet render context.

## Example

Note that so far this has only been tested with piet-cairo.

```rust
let width = 1920;
let height = 1080;

let mut device = Device::new().unwrap();
let mut bitmap = device.bitmap_target(width, height, 1.0).unwrap();
let mut render_ctx = bitmap.render_context();

let piet_backend = PietBackend {
    size: (width as u32, height as u32),
    render_ctx: &mut render_ctx,
};

let root = piet_backend.into_drawing_area();
do_some_plotters_stuff(&root);

bitmap.save_to_file("plot.png").unwrap();
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
