use std::{
    borrow::Borrow,
    collections::HashMap as StdHashMap,
    hash::{BuildHasher, Hash, RandomState},
    iter,
    sync::{Arc, Mutex},
};

use dashmap::DashMap;
use divan::black_box;

const THREADS: &[usize] = &[0, 1, 2, 4, 8, 16];
const LENS: &[usize] = &[1, 2, 4, 8];

const NUM_KEYS: u64 = 5000;

const NUM_KEYS_ERROR_MESSAGE: &str = "\
    `NUM_KEYS` is not large enough to cover all iterations\n\
    lower the iteration count with `sample_count` or `sample_size`, or increase `NUM_KEYS`\
";

trait ConcurrentHashMap<K, V>: Default + Send + Sync {
    fn insert(&self, key: K, val: V) -> Option<V>;
    fn get<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;
}

type MutexHashMap<K, V, S = RandomState> = Mutex<StdHashMap<K, V, S>>;

impl<K: Eq + Hash, V: Clone, S: BuildHasher + Default> ConcurrentHashMap<K, V>
    for MutexHashMap<K, V, S>
where
    K: Send,
    V: Send,
    S: Send + Sync,
{
    fn insert(&self, key: K, val: V) -> Option<V> {
        self.lock().unwrap().insert(key, val)
    }

    fn get<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.lock().unwrap().get(key).cloned()
    }
}

impl<K: Eq + Hash, V: Clone, S: BuildHasher + Clone + Default> ConcurrentHashMap<K, V>
    for DashMap<K, V, S>
where
    K: Send + Sync,
    V: Send + Sync,
    S: Send + Sync,
{
    fn insert(&self, key: K, val: V) -> Option<V> {
        self.insert(key, val)
    }

    fn get<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(key).as_deref().cloned()
    }
}

impl<K: Eq + Hash, V: Clone, H: BuildHasher + Default> ConcurrentHashMap<K, V>
    for scc::HashMap<K, V, H>
where
    K: Send + Sync,
    V: Send + Sync,
    H: Send + Sync,
{
    fn insert(&self, key: K, val: V) -> Option<V> {
        self.upsert(key, val)
    }

    fn get<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(key).as_deref().cloned()
    }
}

pub fn main() {
    divan::main();
}

#[divan::bench(
    types = [MutexHashMap<String, String>, DashMap<String, String>, scc::HashMap<String, String>],
    args = LENS,
    threads = THREADS
)]
fn insert<H: ConcurrentHashMap<String, String>>(bencher: divan::Bencher, len: usize) {
    let map = Arc::new(H::default());

    let keys = iter::repeat_with(|| "hello".to_owned());
    let values = iter::repeat_with(|| "world".to_owned());

    bencher
        .counter(len)
        .with_inputs(|| (keys, values))
        .bench_values(|(mut key, mut value)| {
            let map = Arc::clone(&map);
            for _ in 0..len {
                map.insert(key.next().unwrap(), value.next().unwrap());
            }
        });
}

#[divan::bench(
    types = [MutexHashMap<String, String>, DashMap<String, String>, scc::HashMap<String, String>],
    args = LENS,
    threads = THREADS
)]
fn get<H: ConcurrentHashMap<String, String>>(bencher: divan::Bencher, len: usize) {
    let map = Arc::new(H::default());

    let keys = populate_map(map.as_ref(), || "world".to_owned());
    let keys_iter = MutexIter::new(keys.into_iter());

    bencher
        .counter(len)
        .with_inputs(|| keys_iter.next().expect(NUM_KEYS_ERROR_MESSAGE))
        .bench_values(|key| {
            let map = Arc::clone(&map);
            for _ in 0..len {
                let val = map.get(&key);
                black_box(val);
            }
        });
}

fn populate_map<F>(map: &impl ConcurrentHashMap<String, String>, f: F) -> Vec<String>
where
    F: Fn() -> String,
{
    let keys = (1..=NUM_KEYS).map(|n| n.to_string()).collect::<Vec<_>>();

    for key in &keys {
        map.insert(key.clone(), f());
    }

    keys
}

struct MutexIter<I> {
    iter: Mutex<I>,
}

impl<I, T> MutexIter<I>
where
    I: Iterator<Item = T>,
{
    fn new(iter: I) -> MutexIter<I> {
        let iter = Mutex::new(iter);
        MutexIter { iter }
    }

    #[track_caller]
    fn next(&self) -> Option<T> {
        self.iter.lock().unwrap().next()
    }
}
