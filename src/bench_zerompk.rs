use criterion::{black_box, Criterion};
use zerompk::{FromMessagePack, ToMessagePack};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: ToMessagePack + for<'de> FromMessagePack<'de> + PartialEq,
{
    const BUFFER_LEN: usize = 10_000_000;
    let mut group = c.benchmark_group(format!("{}/zerompk", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            zerompk::to_msgpack(black_box(data), black_box(&mut serialize_buffer)).unwrap();
            black_box(())
        })
    });

    let deserialize_buffer = zerompk::to_msgpack_vec(data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(zerompk::from_msgpack::<T>(black_box(&deserialize_buffer)).unwrap());
        })
    });

    crate::bench_size(name, "zerompk", deserialize_buffer.as_slice());

    assert!(zerompk::from_msgpack::<T>(black_box(&deserialize_buffer)).unwrap() == *data);

    group.finish();
}

// Since zerompk's current borrowed decoding support is limited,
// we will not include it in our benchmarks.
