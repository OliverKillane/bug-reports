//! A minimised example for with_inputs affecting time
//! - Each benchmark should have identical time

#[divan::bench(
    consts = [100,1000,10000,100000],
    max_time = 0.1
)]
fn with_sleep<const SLEEP_TIME: u64>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            use std::{thread, time::Duration};
            thread::sleep(Duration::from_micros(SLEEP_TIME))
        })
        .bench_values(|_| () ) 
}


#[divan::bench(
    consts = [1,10,100,1000],
)]
fn with_iterator<const SIZE: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            (0..SIZE).collect::<Vec<_>>()
        })
        .bench_values(|_| () ) 
}

#[divan::bench(
    consts = [1,10,100,1000],
)]
fn with_loop<const SIZE: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut a = Vec::with_capacity(SIZE);
            for i in 0..SIZE {
                a.push(i);
            }
            a
        })
        .bench_values(|_| () ) 
}


#[divan::bench(
    consts = [1,10,100,1000],
)]
fn with_nothing<const SIZE: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            ()
        })
        .bench_values(|_| () ) 
}

fn main() {
    divan::main()
}