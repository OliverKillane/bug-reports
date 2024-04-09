## Replicate
```bash
cargo version --verbose
```
```
cargo 1.77.1 (e52e36006 2024-03-26)
release: 1.77.1
commit-hash: e52e360061cacbbeac79f7f1215a7a90b6f08442
commit-date: 2024-03-26
host: x86_64-unknown-linux-gnu
libgit2: 1.7.2 (sys:0.18.2 vendored)
libcurl: 8.5.0-DEV (sys:0.4.70+curl-8.5.0 vendored ssl:OpenSSL/1.1.1w)
ssl: OpenSSL 1.1.1w  11 Sep 2023
os: Ubuntu 22.04 (jammy) [64-bit]
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
The issue can be resolved by commenting out the example [here](./library/Cargo.toml).