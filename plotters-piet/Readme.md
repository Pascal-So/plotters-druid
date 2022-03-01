# Plotters Piet

[![crates.io](https://img.shields.io/crates/v/plotters-piet.svg?logo=rust)](https://crates.io/crates/plotters-piet)
[![docs.rs badge](https://docs.rs/plotters-piet/badge.svg)](https://docs.rs/plotters-piet)

A [Piet](https://crates.io/crates/piet) backend for [Plotters](https://crates.io/crates/plotters). This lets you draw plots on a Piet render context.

Currently the piet dependency is at 0.3 because that's what druid depends on. The code is *almost* compatible with
piet 0.4 and 0.5, required changes are listed in comments in the code. The main purpose of this crate is to serve
as a building block for [plotters-druid](https://github.com/Pascal-So/plotters-druid) and therefore compatibility
with the current druid version takes priority over tracking the latest piet version.

Note that so far this has only been tested with piet-cairo and piet-direct2d.

Examples can be found in the [examples directory](https://github.com/Pascal-So/plotters-druid/tree/main/plotters-piet/examples).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
