To run the transpiler, you must have Docker installed. It will take a while as it needs to build `c2rust`. Subsequent runs will be much faster since Docker caches the build.

```
git clone https://github.com/EsotericSoftware/spine-runtimes.git
cd spine-runtimes
git checkout d9935741c2b84abea85e961489ed71b5b473aa64    # For Spine 4.1
git checkout 674fab37f0d7e2dedb4ab4c32ae09dcb8ca67ca3    # For Spine 3.8
cd ..
cargo run
```

A `spine_c.rs` is created in the `out/` directory. It may not be useable as-is, especially on Rust stable, and requires manual cleanup. Warnings can mostly be fixed using `cargo fix`. Once `spine_c.rs` is prepared, replace the appropriate `src/c/spine_c_X.Y.rs` in the base project. 
