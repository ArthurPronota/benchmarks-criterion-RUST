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

// Макрос, используемый для определения группы функций для тестового каркаса (benchmark harness)
// создаёт fn benches() { ... }
criterion_group!(
    benches, // сгенерированная макросом: запускает все бенчмарки группы
    bench    // функция: описывает, что мерить
);

// макрос Criterion. Он генерирует настоящую функцию fn main(), которая запускает 
// бенчмарк-бинарник.
// создаёт fn main() { benches() }
criterion_main!(benches);
