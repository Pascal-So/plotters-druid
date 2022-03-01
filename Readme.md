# Plotters Widget for Druid

[![crates.io](https://img.shields.io/crates/v/plotters-druid.svg?logo=rust)](https://crates.io/crates/plotters-druid)
[![docs.rs badge](https://docs.rs/plotters-druid/badge.svg)](https://docs.rs/plotters-druid)

Use [Plotters](https://crates.io/crates/plotters) to draw plots in [Druid](https://crates.io/crates/druid).

## Examples

### [Simple](https://github.com/Pascal-So/plotters-druid/blob/main/examples/simple.rs)

This draws the [basic x² plot from the plotters example](https://docs.rs/plotters/0.3.1/plotters/#quick-start), filling out the entire window. The size of the plotting area changes when resizing the window.

![Simple example](https://raw.githubusercontent.com/Pascal-So/plotters-druid/main/examples/plotters-druid-simple-example.png)

```bash
cargo run --example simple
```

### [Interactive](https://github.com/Pascal-So/plotters-druid/blob/main/examples/interactive.rs)

In this example we use a value from the druid data to manipulate the plot.

![Interactive example](https://raw.githubusercontent.com/Pascal-So/plotters-druid/main/examples/plotters-druid-interactive-example.gif)

```bash
cargo run --example interactive
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
