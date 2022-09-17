# rusty_spine

Spine runtime for Rust (and wasm!) transpiled from the official C Runtime.

## Rust API

A Rust API is provided on top of the C API, because working with C APIs directly in Rust is tedious and error prone. It's made to be as thin a wrapper as possible. Additionally, a `SkeletonController` is provided as a generic solution that should be good enough for most use cases. [See the simple example of this in action](https://github.com/jabuwu/rusty_spine/blob/main/examples/simple.rs).

The Rust API is unstable and changes often.

## C API

If you wish to use the C API directly (either because the Rust API lacks a feature or for performance reasons), it is available under the `rusty_spine::c` import. [See the C example](https://github.com/jabuwu/rusty_spine/blob/main/examples/c.rs).

## License

Because this project uses the official Spine Runtime, you must follow the Spine Runtimes License Agreement. If using the `libc` crate (via the `use_libc` feature), then see the [libc crate](https://crates.io/crates/libc) for licensing. If using the built-in (wasm compatible) libc implementation, you must follow the BSD 3-clause license of The Regents of the University of California. See the `LICENSE` file for complete licenses. The Rust code is licensed under dual MIT / Apache-2.0 but with no attribution necessary. All contributions must agree to this licensing.
