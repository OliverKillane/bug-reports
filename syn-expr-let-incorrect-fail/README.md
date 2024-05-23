## Potential bug in `syn::ExprLet` parsing

ExprLet fails to parse `let x = SomeStruct { }`
- Documentation does not specify any expressions that cannot exist here ([docs](https://docs.rs/syn/latest/syn/struct.ExprLet.html))
- see [./src/lib.rs](./src/lib.rs)

This is explicitly not allowed in `syn`, I think this is due to [allow_struct](https://github.com/dtolnay/syn/blob/747f42f235f8e1b5551a6aeca1d2779dce413408/src/expr.rs#L2120).

Workaround is easy, surround with `{ .. }` or `( .. )`.

## Replicate
```bash
RUST_BACKTRACE=1 cargo test
```
