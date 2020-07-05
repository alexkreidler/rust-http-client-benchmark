// use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion::*;
use httpbench::fibonacci;

use httpbench::*;

pub fn criterion_benchmark_fib(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

pub fn criterion_reqwest(c: &mut Criterion) {
    let mut group = c.benchmark_group("log_scale_example");
    group.sample_size(10);
    // group.plot_config(plot_config);

    let urls = setup();

    for i in [1].iter() {
        group.bench_with_input(BenchmarkId::new("Reqwest", i), i, |b, _| {
            b.iter(|| {
                let out = <httpbench::Reqwest as Requestor>::make_reqs(urls.clone()).unwrap();
                // print!("{:#?} ", out);
            })
            // b.iter_batched(setup, |urls| {
            //     // let h = httpbench::Reqwest {};
            //     let out = <httpbench::Reqwest as Requestor>::make_reqs(urls).unwrap();
            //     print!("{:#?} ", out);
            // }, BatchSize::PerIteration)
        });
    }
    group.finish();
}

pub fn criterion_clone(c: &mut Criterion) {
    let urls = setup();
    c.bench_function("Cloning URLs vector", |b| {
        b.iter(|| {
            black_box(urls.clone());
            ()
        })
    });
    // group.plot_config(plot_config);
}

criterion_group!(benches, criterion_clone, criterion_reqwest);
criterion_main!(benches);
