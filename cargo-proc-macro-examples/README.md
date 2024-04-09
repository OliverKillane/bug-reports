## Replicate
```bash
cargo --version
```
```
cargo 1.77.1 (e52e36006 2024-03-26)
```
```bash
cd application
cargo check
```
```
thread 'main' panicked at src/cargo/core/resolver/features.rs:323:14:
activated_features for invalid package: features did not find PackageId { name: "library", version: "0.1.0", source: "/home/oliverkillane/files/bug-reports/cargo-proc-macro-examples/library" } NormalOrDev
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```