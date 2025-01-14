use std::{collections::HashMap, hint::black_box, iter};

use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BatchSize, BenchmarkGroup, Criterion,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Data {
    s: String,
    p: (u64, u64, u64),
}

impl Data {
    fn sample() -> Self {
        Data {
            s: "hello, world!".into(),
            p: (128, 512, 1024),
        }
    }

    fn sample_vec(n: usize) -> Vec<Self> {
        iter::repeat_with(Data::sample).take(n).collect()
    }
}

const SAMPLE_SIZE: usize = 100;

fn serialize_to_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();

    g.bench_function("to_value", |b| {
        b.iter(|| {
            black_box(serde_json::to_value(black_box(&data)).unwrap());
        })
    });
}

fn serialize_to_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();

    g.bench_function("to_string", |b| {
        b.iter(|| {
            black_box(serde_json::to_string(black_box(&data)).unwrap());
        })
    });
}

fn serialize_big_to_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample_vec(SAMPLE_SIZE);

    g.bench_function("to_value", |b| {
        b.iter(|| {
            black_box(serde_json::to_value(black_box(&data)).unwrap());
        })
    });
}

fn serialize_big_to_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample_vec(SAMPLE_SIZE);

    g.bench_function("to_string", |b| {
        b.iter(|| {
            black_box(serde_json::to_string(black_box(&data)).unwrap());
        })
    });
}

fn double_serialize_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();

    g.bench_function("to_value", |b| {
        b.iter_batched(
            || HashMap::<String, serde_json::Value>::from([("data".into(), Default::default())]),
            |mut map| {
                map.insert(
                    "data".into(),
                    serde_json::to_value(black_box(&data)).unwrap(),
                );
                black_box(serde_json::to_string(&map).unwrap());
            },
            BatchSize::SmallInput,
        )
    });
}

fn double_serialize_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();

    g.bench_function("to_string", |b| {
        b.iter_batched(
            || HashMap::<String, String>::from([("data".into(), Default::default())]),
            |mut map| {
                map.insert(
                    "data".into(),
                    serde_json::to_string(black_box(&data)).unwrap(),
                );
                black_box(serde_json::to_string(&map).unwrap());
            },
            BatchSize::SmallInput,
        )
    });
}

fn insert_to_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();

    g.bench_function("to_value_get_mut", |b| {
        b.iter_batched(
            || serde_json::to_value(&data).unwrap(),
            |mut value| {
                let s = value.get_mut("s").unwrap();
                *s = black_box("good night, world!").into();
                black_box(value);
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("to_value_as_object_mut", |b| {
        b.iter_batched(
            || serde_json::to_value(&data).unwrap(),
            |mut value| {
                let map = value.as_object_mut().unwrap();
                map.insert("s".to_owned(), black_box("good night, world!").into());
                black_box(value);
            },
            BatchSize::SmallInput,
        )
    });
}

fn insert_to_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();

    g.bench_function("to_string", |b| {
        b.iter_batched(
            || serde_json::to_string(&data).unwrap(),
            |mut s| {
                let mut v = serde_json::from_str::<Data>(&s).unwrap();
                v.s = black_box("good night, world!").into();
                s = serde_json::to_string(&v).unwrap();
                black_box(s);
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_serialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize");
    serialize_to_value(&mut group);
    serialize_to_string(&mut group);
    group.finish();
}

fn bench_serialize_big(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize_big");
    serialize_big_to_value(&mut group);
    serialize_big_to_string(&mut group);
    group.finish();
}

fn bench_double_serialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("double_serialize");
    double_serialize_value(&mut group);
    double_serialize_string(&mut group);
    group.finish();
}

fn bench_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert");
    insert_to_value(&mut group);
    insert_to_string(&mut group);
    group.finish();
}

criterion_group!(serialize, bench_serialize);
criterion_group!(serialize_big, bench_serialize_big);
criterion_group!(double_serialize, bench_double_serialize);
criterion_group!(insert, bench_insert);

criterion_main!(serialize, serialize_big, double_serialize, insert);
