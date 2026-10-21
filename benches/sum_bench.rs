use criterion::{
                black_box, 
                criterion_group, 
                criterion_main, 
                Criterion
            };

//use std::hint::black_box ;

fn sum_to(n: u64) -> u64 { (1..=n).sum() }

fn bench(c: &mut Criterion) {
 c.bench_function(
    "sum_to_1000", 
    |b| 
        //*
        b.iter(|| sum_to(black_box(1000))
        //*/
        //b.iter(|| black_box(sum_to(black_box(1000)))
    ));
}


criterion_group!(benches, bench);
criterion_main!(benches);
