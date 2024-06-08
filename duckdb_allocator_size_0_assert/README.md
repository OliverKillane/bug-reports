## DuckDB allocator failure case.
```bash
cargo test
```
```
running 2 tests
duckdb_allocator_size_0_assert-55c63b607dff2947: duckdb/src/common/allocator.cpp:122: duckdb::data_t* duckdb::Allocator::AllocateData(duckdb::idx_t): Assertion `size > 0' failed.
test tests::correct_behaviour ... ok
error: test failed, to rerun pass `--lib`
```

Fails when an enum is used in a check constraint.
See the working, and failing versions in [./src/lib.rs](./src/lib.rs).
