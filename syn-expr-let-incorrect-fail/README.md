## Potential bug in `syn::ExprLet` parsing

ExprLet fails to parse `let x = SomeStruct { }`
- Documentation does not specify any expressions that cannot exist here ([docs](https://docs.rs/syn/latest/syn/struct.ExprLet.html))
- see [./src/lib.rs](./src/lib.rs)

## Replicate
```bash
RUST_BACKTRACE=1 cargo test
```
