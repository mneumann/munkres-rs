use criterion::{criterion_group, criterion_main, Criterion};
use munkres::{solve_assignment, WeightMatrix};

fn gen_matrix(n: usize) -> Vec<i32> {
    (0..n * n)
        .map(|i| {
            let row = i / n;
            let col = i % n;
            (row * col) as i32
        })
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "solve_assignment",
        |b: &mut criterion::Bencher, n: &usize| {
            let n = *n;
            let matrix = gen_matrix(n);
            b.iter(|| {
                let mut weights: WeightMatrix<i32> = WeightMatrix::from_row_vec(n, matrix.clone());
                let _matching = solve_assignment(&mut weights);
            })
        },
        vec![10, 50, 100],
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
