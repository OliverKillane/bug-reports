## Sleep `with_inputs` affects benchmark time

The following benchmarks all contain nothing, but they do not, because of differences in the `with_inputs`.

```bash
minimised_example  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ with_mm_pause                 │               │               │               │         │
│  ├─ 100          0.101 ns      │ 0.146 ns      │ 0.123 ns      │ 0.123 ns      │ 2       │ 16384
│  ├─ 1000         0.298 ns      │ 0.298 ns      │ 0.298 ns      │ 0.298 ns      │ 1       │ 2048
│  ├─ 10000        1.116 ns      │ 1.116 ns      │ 1.116 ns      │ 1.116 ns      │ 1       │ 128
│  ╰─ 100000       8.053 ns      │ 8.053 ns      │ 8.053 ns      │ 8.053 ns      │ 1       │ 16
├─ with_nothing                  │               │               │               │         │
│  ├─ 100          0.09 ns       │ 0.093 ns      │ 0.091 ns      │ 0.091 ns      │ 100     │ 819200
│  ├─ 1000         0.09 ns       │ 0.094 ns      │ 0.091 ns      │ 0.091 ns      │ 100     │ 819200
│  ├─ 10000        0.09 ns       │ 1.363 ns      │ 0.091 ns      │ 0.108 ns      │ 100     │ 819200
│  ╰─ 100000       0.09 ns       │ 2.514 ns      │ 0.101 ns      │ 0.156 ns      │ 100     │ 819200
╰─ with_sleep                    │               │               │               │         │
   ├─ 100          0.323 ns      │ 0.323 ns      │ 0.323 ns      │ 0.323 ns      │ 1       │ 512
   ├─ 1000         1.616 ns      │ 1.616 ns      │ 1.616 ns      │ 1.616 ns      │ 1       │ 64
   ├─ 10000        10.11 ns      │ 10.11 ns      │ 10.11 ns      │ 10.11 ns      │ 1       │ 8
   ╰─ 100000       53.86 ns      │ 53.86 ns      │ 53.86 ns      │ 53.86 ns      │ 1       │ 1
```

## Replicate
```bash
cargo bench
```