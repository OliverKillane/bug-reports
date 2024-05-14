## Sleep `with_inputs` affects benchmark time

The following benchmarks all contain nothing, but they do not, because of differences in the `with_inputs`.

```bash
minimised_example  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ with_nothing                  │               │               │               │         │
│  ├─ 100          0.131 ns      │ 1.411 ns      │ 0.132 ns      │ 0.145 ns      │ 100     │ 819200
│  ├─ 1000         0.131 ns      │ 0.147 ns      │ 0.132 ns      │ 0.132 ns      │ 100     │ 819200
│  ├─ 10000        0.131 ns      │ 0.145 ns      │ 0.132 ns      │ 0.132 ns      │ 100     │ 819200
│  ╰─ 100000       0.131 ns      │ 0.145 ns      │ 0.132 ns      │ 0.132 ns      │ 100     │ 819200
╰─ with_sleep                    │               │               │               │         │
   ├─ 100          0.39 ns       │ 0.39 ns       │ 0.39 ns       │ 0.39 ns       │ 1       │ 512
   ├─ 1000         1.568 ns      │ 1.568 ns      │ 1.568 ns      │ 1.568 ns      │ 1       │ 64
   ├─ 10000        12.45 ns      │ 12.45 ns      │ 12.45 ns      │ 12.45 ns      │ 1       │ 8
   ╰─ 100000       86.83 ns      │ 86.83 ns      │ 86.83 ns      │ 86.83 ns      │ 1       │ 1
```

## Replicate
```bash
cargo bench
```