use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn size_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("imagesize");

    for file in walkdir::WalkDir::new("tests/images").into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            let path = file.path();
            group.bench_with_input(BenchmarkId::from_parameter(path.display()), &path, |b, &file| {
                b.iter(|| imagesize::size(black_box(file)))
            });
        }
    }

    group.finish();
}

criterion_group!(benches, size_benchmarks);
criterion_main!(benches);