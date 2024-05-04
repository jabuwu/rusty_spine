# rusty_spine

[Spine](http://esotericsoftware.com/) runtime for Rust (and wasm!) transpiled from the official C Runtime.

```
[dependencies]
rusty_spine = "0.7"
```

[Online demo!](https://jabuwu.github.io/rusty_spine/)

## Supported Versions

| rusty_spine      | Spine |
| ---------------- | ----- |
| 0.8 (unreleased) | 4.2   |
| 0.1 - 0.7        | 4.1   |

## Rust API

A Rust API is provided on top of the C API, because working with C APIs directly in Rust is tedious and error prone. It's made to be as thin a wrapper as possible.

The Rust API is mostly stable, but does not support the [Timeline API](http://en.esotericsoftware.com/spine-api-reference#Timeline) or custom attachment loaders. A few accessors might be missing as well. Most projects do not need these, but if your's does, please open an issue.

[Rust API Documentation](https://docs.rs/rusty_spine/latest/rusty_spine/index.html)

## C API

If you wish to use the C API directly (either because the Rust API lacks a feature or for performance reasons), it is available under the [rusty_spine::c](https://docs.rs/rusty_spine/latest/rusty_spine/c/index.html) import. [See the C example](https://github.com/jabuwu/rusty_spine/blob/main/examples/c.rs).

[C API Documentation](http://en.esotericsoftware.com/spine-c)

## Engine Integrations

- [Bevy](https://github.com/jabuwu/bevy_spine) - ([WASM Demo](https://jabuwu.github.io/bevy_spine_demos/))

To integrate into your own engine, see the [miniquad example](https://github.com/jabuwu/rusty_spine/blob/main/examples/miniquad.rs)

## License

Because this project uses the official Spine Runtime, you must follow the Spine Runtimes License Agreement. If using the `libc` crate (via the `libc` feature), then see the [libc crate](https://crates.io/crates/libc) for licensing. If using the built-in (wasm compatible) libc implementation, you must follow the BSD 3-clause license of The Regents of the University of California. See the `LICENSE` file for complete licenses. The Rust code is licensed under dual MIT / Apache-2.0 but with no attribution necessary. All contributions must agree to this licensing.

## Features

### libc

Default: no

A small subset of libc is provided in this repo to avoid a dependency on [libc](https://crates.io/crates/libc) and to allow the code to run in the `wasm32-unknown-unknown` target. However, it's possible to rely on the OS implementation of libc instead by using the `libc` feature.

### draw_functions

Default: yes

Provides [helper functions](https://github.com/jabuwu/rusty_spine/tree/main/src/draw) for generating mesh data, as well as the `SkeletonController` helper struct.

### mint

Default: yes

Provides additional math functions using [mint](https://docs.rs/mint).
