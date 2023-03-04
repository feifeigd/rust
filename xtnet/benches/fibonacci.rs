
#[macro_use]
extern crate criterion;
extern crate xtnet;

use xtnet::{fast_fibonacci, slow_fibonacci};
use criterion::Criterion;

fn slow_fibonacci_benchmark(c:&mut Criterion){
    c.bench_function("slow fibonacci 8", |b|b.iter(||slow_fibonacci(8)));
}

fn fast_fibonacci_benchmark(c:&mut Criterion){
    c.bench_function("fast fibonacci 8", |b|b.iter(||fast_fibonacci(8)));
}

criterion_group!(fib_bench, slow_fibonacci_benchmark, fast_fibonacci_benchmark);
criterion_main!(fib_bench);
