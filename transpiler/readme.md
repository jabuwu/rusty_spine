To run the transpiler, you must have Docker installed. It will take a while as it needs to build `c2rust`. Subsequent runs will be much faster since Docker caches the build.

```
git clone https://github.com/EsotericSoftware/spine-runtimes.git
cd spine-runtimes
git checkout 7a5c6995effdeaf6c4846884aae0d7fe37982f9f
cd ..
cargo run
```

A `spine_c.rs` is created in the `out/` directory. It may not be useable as-is, especially on Rust stable, and requires manual cleanup. Warnings can mostly be fixed using `cargo fix`.
