## `with_inputs` affects benchmark time
`with_inputs` affects the benchmark time, the [documentation specifies it does not](https://github.com/nvzqz/divan/blob/8768032314ec96dd67c007ba1f4d64ca3287be51/src/bench/mod.rs#L136).


The following benchmarks all contain nothing, but they do not, because of differences in the `with_inputs`.
```bash
minimised_example  fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ with_iterator                 │               │               │               │         │
│  ├─ 1            4.635 ns      │ 8.508 ns      │ 4.805 ns      │ 4.961 ns      │ 100     │ 51200
│  ├─ 10           4.567 ns      │ 54.52 ns      │ 5.218 ns      │ 5.777 ns      │ 100     │ 51200
│  ├─ 100          24.13 ns      │ 63.15 ns      │ 25.58 ns      │ 26.8 ns       │ 100     │ 25600
│  ╰─ 1000         195.8 ns      │ 294.7 ns      │ 205.1 ns      │ 207.8 ns      │ 100     │ 3200
├─ with_loop                     │               │               │               │         │
│  ├─ 1            4.424 ns      │ 5.201 ns      │ 4.442 ns      │ 4.58 ns       │ 100     │ 51200
│  ├─ 10           4.434 ns      │ 7.762 ns      │ 4.678 ns      │ 4.69 ns       │ 100     │ 51200
│  ├─ 100          24.35 ns      │ 510.4 ns      │ 27.41 ns      │ 37.2 ns       │ 100     │ 25600
│  ╰─ 1000         201.5 ns      │ 12.65 µs      │ 254.6 ns      │ 625.9 ns      │ 100     │ 3200
├─ with_nothing                  │               │               │               │         │
│  ├─ 1            0.108 ns      │ 10.69 ns      │ 0.109 ns      │ 0.322 ns      │ 100     │ 819200
│  ├─ 10           0.112 ns      │ 11.46 ns      │ 0.152 ns      │ 0.27 ns       │ 100     │ 819200
│  ├─ 100          0.108 ns      │ 10.28 ns      │ 0.132 ns      │ 0.234 ns      │ 100     │ 819200
│  ╰─ 1000         0.108 ns      │ 13.28 ns      │ 0.115 ns      │ 0.252 ns      │ 100     │ 819200
╰─ with_sleep                    │               │               │               │         │
   ├─ 100          0.498 ns      │ 0.498 ns      │ 0.498 ns      │ 0.498 ns      │ 1       │ 256
   ├─ 1000         1.19 ns       │ 1.19 ns       │ 1.19 ns       │ 1.19 ns       │ 1       │ 64
   ├─ 10000        11.86 ns      │ 11.86 ns      │ 11.86 ns      │ 11.86 ns      │ 1       │ 8
   ╰─ 100000       56.86 ns      │ 56.86 ns      │ 56.86 ns      │ 56.86 ns      │ 1       │ 1
```

## Replicate
```bash
cargo bench
```