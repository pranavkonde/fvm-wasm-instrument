use criterion::{
    criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup, Criterion,
    Throughput,
};

use fvm_wasm_instrument::{gas_metering, stack_limiter};

use std::{
    fs::{read, read_dir},
    path::PathBuf,
};

fn fixture_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("benches");
    path.push("fixtures");
    path
}

fn any_fixture<F, M>(group: &mut BenchmarkGroup<M>, f: F)
where
    F: Fn(&[u8]),
    M: Measurement,
{
    for entry in read_dir(fixture_dir()).unwrap() {
        let entry = entry.unwrap();
        let bytes = read(&entry.path()).unwrap();
        group.throughput(Throughput::Bytes(bytes.len().try_into().unwrap()));
        group.bench_with_input(
            entry.file_name().to_str().unwrap(),
            &bytes,
            |bench, input| bench.iter(|| f(input)),
        );
    }
}

fn gas_metering(c: &mut Criterion) {
    let mut group = c.benchmark_group("Gas Metering");
    any_fixture(&mut group, |raw_wasm| {
        gas_metering::inject(raw_wasm, &gas_metering::ConstantCostRules::default(), "env").unwrap();
    });
}

fn stack_height_limiter(c: &mut Criterion) {
    let mut group = c.benchmark_group("Stack Height Limiter");
    any_fixture(&mut group, |raw_wasm| {
        stack_limiter::inject(raw_wasm, 128).unwrap();
    });
}

criterion_group!(benches, gas_metering, stack_height_limiter);
criterion_main!(benches);
