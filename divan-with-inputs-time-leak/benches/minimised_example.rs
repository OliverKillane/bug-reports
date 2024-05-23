//! Benchmark should have no time

#[divan::bench(
    consts = [100,1000,10000,100000],
)]
fn with_sleep<const SLEEP_TIME: u64>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            use std::{thread, time::Duration};
            thread::sleep(Duration::from_micros(SLEEP_TIME))
        })
        .bench_refs(|_| ())
}

/// An even simpler sleep with mm_pause
#[divan::bench(
    consts = [100,1000,10000,100000],
)]
fn with_mm_pause<const SLEEP_ITERS: u64>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            for _ in 0..SLEEP_ITERS {
                unsafe {
                    core::arch::x86_64::_mm_pause();
                }
            }
        })
        .bench_refs(|_| ())
}

#[divan::bench(
    consts = [100,1000,10000,100000],
)]
fn with_nothing<const SLEEP_TIME: u64>(bencher: divan::Bencher) {
    bencher.with_inputs(|| ()).bench_refs(|_| ())
}

fn main() {
    divan::main()
}
