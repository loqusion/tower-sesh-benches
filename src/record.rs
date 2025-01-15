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

fn serialize_simple_direct(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();

    g.bench_function("direct", |b| {
        b.iter(|| {
            black_box(serde_json::to_string(black_box(&data)).unwrap());
        })
    });
}

fn serialize_simple_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();

    g.bench_function("value", |b| {
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

fn serialize_simple_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();

    g.bench_function("string", |b| {
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

fn serialize_big_direct(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample_vec(SAMPLE_SIZE);

    g.bench_function("direct", |b| {
        b.iter(|| {
            black_box(serde_json::to_string(black_box(&data)).unwrap());
        })
    });
}

fn serialize_big_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample_vec(SAMPLE_SIZE);

    g.bench_function("value", |b| {
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

fn serialize_big_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample_vec(SAMPLE_SIZE);

    g.bench_function("string", |b| {
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

fn serialize_complex_direct(g: &mut BenchmarkGroup<WallTime>) {
    let data = ComplexData::sample();

    g.bench_function("direct", |b| {
        b.iter(|| {
            black_box(serde_json::to_string(black_box(&data)).unwrap());
        })
    });
}

fn serialize_complex_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = ComplexData::sample();

    g.bench_function("value", |b| {
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

fn serialize_complex_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = ComplexData::sample();

    g.bench_function("string", |b| {
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

fn deserialize_simple_direct(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();
    let buf = serde_json::to_string(&data).unwrap();

    g.bench_function("direct", |b| {
        b.iter(|| {
            let mut data: Data = serde_json::from_str(black_box(&buf)).unwrap();
            black_box(&mut data);
        })
    });
}

fn deserialize_simple_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();
    type Map = HashMap<String, serde_json::Value>;
    let map: Map = HashMap::from([("data".into(), serde_json::to_value(data).unwrap())]);
    let buf = serde_json::to_string(&map).unwrap();

    g.bench_function("value", |b| {
        b.iter(|| {
            let map: Map = serde_json::from_str(black_box(&buf)).unwrap();
            let mut data: Data = map
                .get("data")
                .and_then(|value| serde_json::from_value(value.clone()).ok())
                .unwrap();
            black_box(&mut data);
        })
    });
}

fn deserialize_simple_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();
    type Map = HashMap<String, String>;
    let map: Map = HashMap::from([("data".into(), serde_json::to_string(&data).unwrap())]);
    let buf = serde_json::to_string(&map).unwrap();

    g.bench_function("string", |b| {
        b.iter(|| {
            let map: Map = serde_json::from_str(black_box(&buf)).unwrap();
            let mut data: Data = map
                .get("data")
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap();
            black_box(&mut data);
        })
    });
}

fn deserialize_big_direct(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample_vec(SAMPLE_SIZE);
    let buf = serde_json::to_string(&data).unwrap();

    g.bench_function("direct", |b| {
        b.iter(|| {
            let mut data: Vec<Data> = serde_json::from_str(black_box(&buf)).unwrap();
            black_box(&mut data);
        })
    });
}

fn deserialize_big_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample_vec(SAMPLE_SIZE);
    type Map = HashMap<String, serde_json::Value>;
    let map: Map = HashMap::from([("data".into(), serde_json::to_value(data).unwrap())]);
    let buf = serde_json::to_string(&map).unwrap();

    g.bench_function("value", |b| {
        b.iter(|| {
            let map: Map = serde_json::from_str(black_box(&buf)).unwrap();
            let mut data: Vec<Data> = map
                .get("data")
                .and_then(|value| serde_json::from_value(value.clone()).ok())
                .unwrap();
            black_box(&mut data);
        })
    });
}

fn deserialize_big_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample_vec(SAMPLE_SIZE);
    type Map = HashMap<String, String>;
    let map: Map = HashMap::from([("data".into(), serde_json::to_string(&data).unwrap())]);
    let buf = serde_json::to_string(&map).unwrap();

    g.bench_function("string", |b| {
        b.iter(|| {
            let map: Map = serde_json::from_str(black_box(&buf)).unwrap();
            let mut data: Vec<Data> = map
                .get("data")
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap();
            black_box(&mut data);
        })
    });
}

fn deserialize_complex_direct(g: &mut BenchmarkGroup<WallTime>) {
    let data = ComplexData::sample();
    let buf = serde_json::to_string(&data).unwrap();

    g.bench_function("direct", |b| {
        b.iter(|| {
            let mut data: ComplexData = serde_json::from_str(black_box(&buf)).unwrap();
            black_box(&mut data);
        })
    });
}

fn deserialize_complex_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = ComplexData::sample();
    type Map = HashMap<String, serde_json::Value>;
    let map: Map = HashMap::from([("data".into(), serde_json::to_value(data).unwrap())]);
    let buf = serde_json::to_string(&map).unwrap();

    g.bench_function("value", |b| {
        b.iter(|| {
            let map: Map = serde_json::from_str(black_box(&buf)).unwrap();
            let mut data: ComplexData = map
                .get("data")
                .and_then(|value| serde_json::from_value(value.clone()).ok())
                .unwrap();
            black_box(&mut data);
        })
    });
}

fn deserialize_complex_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = ComplexData::sample();
    type Map = HashMap<String, String>;
    let map: Map = HashMap::from([("data".into(), serde_json::to_string(&data).unwrap())]);
    let buf = serde_json::to_string(&map).unwrap();

    g.bench_function("string", |b| {
        b.iter(|| {
            let map: Map = serde_json::from_str(black_box(&buf)).unwrap();
            let mut data: ComplexData = map
                .get("data")
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap();
            black_box(&mut data);
        })
    });
}

fn get_simple_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();

    g.bench_function("value", |b| {
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

fn get_simple_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();

    g.bench_function("string", |b| {
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

fn get_complex_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = ComplexData::sample();

    g.bench_function("value", |b| {
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

fn get_complex_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = ComplexData::sample();

    g.bench_function("string", |b| {
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

fn insert_simple_value(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();

    g.bench_function("value_get_mut", |b| {
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

    g.bench_function("value_as_data", |b| {
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

fn insert_simple_string(g: &mut BenchmarkGroup<WallTime>) {
    let data = Data::sample();

    g.bench_function("string", |b| {
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

    g.bench_function("value_get_mut", |b| {
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

fn bench_serialize_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize_simple");
    serialize_simple_direct(&mut group);
    serialize_simple_value(&mut group);
    serialize_simple_string(&mut group);
    group.finish();
}

fn bench_serialize_big(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize_big");
    serialize_big_direct(&mut group);
    serialize_big_value(&mut group);
    serialize_big_string(&mut group);
    group.finish();
}

fn bench_serialize_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize_complex");
    serialize_complex_direct(&mut group);
    serialize_complex_value(&mut group);
    serialize_complex_string(&mut group);
    group.finish();
}

fn bench_deserialize_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("deserialize_simple");
    deserialize_simple_direct(&mut group);
    deserialize_simple_value(&mut group);
    deserialize_simple_string(&mut group);
    group.finish();
}

fn bench_deserialize_big(c: &mut Criterion) {
    let mut group = c.benchmark_group("deserialize_big");
    deserialize_big_direct(&mut group);
    deserialize_big_value(&mut group);
    deserialize_big_string(&mut group);
    group.finish();
}

fn bench_deserialize_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("deserialize_complex");
    deserialize_complex_direct(&mut group);
    deserialize_complex_value(&mut group);
    deserialize_complex_string(&mut group);
    group.finish();
}

fn bench_get_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_simple");
    get_simple_value(&mut group);
    get_simple_string(&mut group);
    group.finish();
}

fn bench_get_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_complex");
    get_complex_value(&mut group);
    get_complex_string(&mut group);
    group.finish();
}

fn bench_insert_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_simple");
    insert_simple_value(&mut group);
    insert_simple_string(&mut group);
    group.finish();
}

fn bench_insert_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_complex");
    insert_complex_value(&mut group);
    insert_complex_string(&mut group);
    group.finish();
}

criterion_group!(serialize_simple, bench_serialize_simple);
criterion_group!(serialize_big, bench_serialize_big);
criterion_group!(serialize_complex, bench_serialize_complex);
criterion_group!(deserialize_simple, bench_deserialize_simple);
criterion_group!(deserialize_big, bench_deserialize_big);
criterion_group!(deserialize_complex, bench_deserialize_complex);
criterion_group!(get_simple, bench_get_simple);
criterion_group!(get_complex, bench_get_complex);
criterion_group!(insert_simple, bench_insert_simple);
criterion_group!(insert_complex, bench_insert_complex);

criterion_main!(
    serialize_simple,
    serialize_big,
    serialize_complex,
    deserialize_simple,
    deserialize_big,
    deserialize_complex,
    get_simple,
    get_complex,
    insert_simple,
    insert_complex,
);
