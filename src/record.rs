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

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ComplexData {
    deeply: HashMap<String, Vec<HashMap<String, u8>>>,
}

impl ComplexData {
    fn sample() -> Self {
        let data = HashMap::from([
            ("value".into(), 4),
            ("another".into(), 6),
            ("yet_another".into(), 7),
        ]);
        let data = iter::repeat(data).take(10).collect::<Vec<_>>();
        let data = ["nested".into(), "data".into(), "is".into(), "cool".into()]
            .into_iter()
            .zip(iter::repeat(data))
            .collect::<HashMap<_, _>>();

        ComplexData { deeply: data }
    }
}

const SAMPLE_SIZE: usize = 50;

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

fn serialize_complex_to_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = ComplexData::sample();

    g.bench_function("to_value", |b| {
        b.iter(|| {
            black_box(serde_json::to_value(black_box(&data)).unwrap());
        })
    });
}

fn serialize_complex_to_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = ComplexData::sample();

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

fn double_serialize_complex_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = ComplexData::sample();

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

fn double_serialize_complex_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = ComplexData::sample();

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

fn get_from_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();

    g.bench_function("from_value", |b| {
        b.iter_batched(
            || serde_json::to_value(&data).unwrap(),
            |value| {
                let s = value.get("s").and_then(|s| s.as_str()).unwrap();
                black_box(s);
            },
            BatchSize::SmallInput,
        )
    });
}

fn get_from_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();

    g.bench_function("from_string", |b| {
        b.iter_batched(
            || serde_json::to_string(&data).unwrap(),
            |value| {
                let data = serde_json::from_str::<Data>(&value).unwrap();
                black_box(&data.s);
            },
            BatchSize::SmallInput,
        )
    });
}

fn get_complex_from_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = ComplexData::sample();

    g.bench_function("from_value", |b| {
        b.iter_batched(
            || serde_json::to_value(&data).unwrap(),
            |value| {
                let n = value
                    .get("deeply")
                    .and_then(|v| v.get("nested"))
                    .and_then(|v| v.get(3))
                    .and_then(|v| v.get("value"))
                    .and_then(|v| v.as_u64())
                    .unwrap();
                black_box(n);
            },
            BatchSize::SmallInput,
        )
    });
}

fn get_complex_from_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = ComplexData::sample();

    g.bench_function("from_string", |b| {
        b.iter_batched(
            || serde_json::to_string(&data).unwrap(),
            |value| {
                let data = serde_json::from_str::<ComplexData>(&value).unwrap();
                black_box(
                    data.deeply
                        .get("nested")
                        .and_then(|v| v.get(3))
                        .and_then(|m| m.get("value"))
                        .unwrap(),
                );
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

    g.bench_function("to_value_as_data", |b| {
        b.iter_batched(
            || serde_json::to_value(&data).unwrap(),
            |mut value| {
                let mut data = serde_json::from_value::<Data>(value).unwrap();
                data.s = black_box("good night, world!").into();
                value = serde_json::to_value(&data).unwrap();
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

fn insert_complex_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = ComplexData::sample();

    g.bench_function("value", |b| {
        b.iter_batched(
            || serde_json::to_value(&data).unwrap(),
            |mut value| {
                let v = value
                    .get_mut(black_box("deeply"))
                    .and_then(|v| v.get_mut(black_box("nested")))
                    .and_then(|v| v.get_mut(black_box(3)))
                    .and_then(|v| v.get_mut(black_box("value")))
                    .unwrap();
                *v = black_box(5).into();
                black_box(v);
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("value_as_data", |b| {
        b.iter_batched(
            || serde_json::to_value(&data).unwrap(),
            |mut value| {
                let mut data = serde_json::from_value::<ComplexData>(value).unwrap();
                let v = data
                    .deeply
                    .get_mut(black_box("nested"))
                    .and_then(|v| v.get_mut(black_box(3)))
                    .and_then(|m| m.get_mut(black_box("value")))
                    .unwrap();
                *v = black_box(5);
                value = serde_json::to_value(&data).unwrap();
                black_box(value);
            },
            BatchSize::SmallInput,
        )
    });
}

fn insert_complex_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = ComplexData::sample();

    g.bench_function("string", |b| {
        b.iter_batched(
            || serde_json::to_string(&data).unwrap(),
            |mut s| {
                let mut data = serde_json::from_str::<ComplexData>(&s).unwrap();
                let v = data
                    .deeply
                    .get_mut(black_box("nested"))
                    .and_then(|v| v.get_mut(black_box(3)))
                    .and_then(|m| m.get_mut(black_box("value")))
                    .unwrap();
                *v = black_box(5);
                s = serde_json::to_string(&data).unwrap();
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

fn bench_serialize_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize_complex");
    serialize_complex_to_value(&mut group);
    serialize_complex_to_string(&mut group);
    group.finish();
}

fn bench_double_serialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("double_serialize");
    double_serialize_value(&mut group);
    double_serialize_string(&mut group);
    group.finish();
}

fn bench_double_serialize_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("double_serialize_complex");
    double_serialize_complex_value(&mut group);
    double_serialize_complex_string(&mut group);
    group.finish();
}

fn bench_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("get");
    get_from_value(&mut group);
    get_from_string(&mut group);
    group.finish();
}

fn bench_get_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_complex");
    get_complex_from_value(&mut group);
    get_complex_from_string(&mut group);
    group.finish();
}

fn bench_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert");
    insert_to_value(&mut group);
    insert_to_string(&mut group);
    group.finish();
}

fn bench_insert_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_complex");
    insert_complex_value(&mut group);
    insert_complex_string(&mut group);
    group.finish();
}

criterion_group!(serialize, bench_serialize);
criterion_group!(serialize_big, bench_serialize_big);
criterion_group!(serialize_complex, bench_serialize_complex);
criterion_group!(double_serialize, bench_double_serialize);
criterion_group!(double_serialize_complex, bench_double_serialize_complex);
criterion_group!(get, bench_get);
criterion_group!(get_complex, bench_get_complex);
criterion_group!(insert, bench_insert);
criterion_group!(insert_complex, bench_insert_complex);

criterion_main!(
    serialize,
    serialize_big,
    serialize_complex,
    double_serialize,
    double_serialize_complex,
    get,
    get_complex,
    insert,
    insert_complex,
);
