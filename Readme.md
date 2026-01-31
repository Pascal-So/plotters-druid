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

## Version Compatibility Table

There's a lot of [0.x versions](https://0ver.org/) floating around here, so I've
assembled a table to help you figure out which version of `plotters-druid` or
`plotters-piet` you need.

| druid | plotters-druid | piet-common | plotters-piet | plotters |
| --- | --- | -- | -- | -- |
| 0.7.0 | 0.2.0 | 0.3.2 | 0.3.1 | 0.3 |
| 0.8.2 / 0.8.3 | 0.3.0 | 0.6.1 / 0.6.2 | 0.3.2 | 0.3 |
| commit [b27ea6a](https://github.com/linebender/druid/tree/b27ea6a618c32f9ea0e8a56822f9487d23401c0d) | commit [44f35a0](https://github.com/Pascal-So/plotters-druid/tree/44f35a0afe8a0c81b8b592d6eeb23bd56e3e830e) | 0.7.0-cairo18 | 0.3.3 | 0.3 |
|  | | >= 0.4, < 0.9 | 0.3.4 | 0.3 |

If you run into problems with multiple versions of `piet` in your dependency
graph then check out the [dependency duplication](https://doc.rust-lang.org/cargo/reference/resolver.html#unexpected-dependency-duplication)
section of the Cargo Book. `cargo update` with the [`--precise`](https://doc.rust-lang.org/cargo/commands/cargo-update.html#option-cargo-update---precise)
flag should usually be able to fix the issue.

See also [this cargo issue](https://github.com/rust-lang/cargo/issues/9029) for
more information about version ranges.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
