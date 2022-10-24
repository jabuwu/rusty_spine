# rusty_spine

Spine runtime for Rust (and wasm!) transpiled from the official C Runtime. Supports [Spine 3.8 and 4.1](http://esotericsoftware.com/).

```
[dependencies]
rusty_spine = "0.3.1"
```

[Online demo!](https://jabuwu.github.io/rusty_spine/)

## Rust API

A Rust API is provided on top of the C API, because working with C APIs directly in Rust is tedious and error prone. It's made to be as thin a wrapper as possible.

The Rust API is unstable and changes often.

[Rust API Documentation](https://docs.rs/rusty_spine/latest/rusty_spine/index.html)

## C API

If you wish to use the C API directly (either because the Rust API lacks a feature or for performance reasons), it is available under the `rusty_spine::c` import. [See the C example](https://github.com/jabuwu/rusty_spine/blob/main/examples/c.rs).

[C API Documentation](http://en.esotericsoftware.com/spine-c)

## License

Because this project uses the official Spine Runtime, you must follow the Spine Runtimes License Agreement. If using the `libc` crate (via the `use_libc` feature), then see the [libc crate](https://crates.io/crates/libc) for licensing. If using the built-in (wasm compatible) libc implementation, you must follow the BSD 3-clause license of The Regents of the University of California. See the `LICENSE` file for complete licenses. The Rust code is licensed under dual MIT / Apache-2.0 but with no attribution necessary. All contributions must agree to this licensing.

## Features

### use_libc

Default: no

A small subset of libc is provided in this repo to avoid a dependency on [libc](https://crates.io/crates/libc) and to allow the code to run in the `wasm32-unknown-unknown` target. However, it's possible to rely on the OS implementation of libc instead by using the `use_libc` feature.

### draw_functions

Default: yes

Provides [helper functions](https://github.com/jabuwu/rusty_spine/tree/main/src/draw) for generating mesh data, as well as the `SkeletonController` helper struct.

### egui_debugger

Default: no

Provides an [egui](https://github.com/emilk/egui) debugger window for viewing skeleton and animation state. See it in action by running the `bevy` example with this feature enabled:

`cargo run --release --example bevy --features egui_debugger`

### mint

Default: yes

Provides additional math functions using [mint](https://docs.rs/mint).

### spine38

Default: no

Builds a Spine 3.8-compatible runtime instead of a 4.1-compatible one. Enable this feature if you are loading files saved with Spine 3.8.x instead of 4.1.x.
