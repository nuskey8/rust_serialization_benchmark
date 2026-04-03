use crate::datasets::BorrowableData;
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

pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: ToMessagePack + for<'de> FromMessagePack<'de> + BorrowableData,
    for<'a> T::Borrowed<'a>: ToMessagePack + FromMessagePack<'a>,
{
    bench(name, c, data);

    let mut group = c.benchmark_group(format!("{}/zerompk", name));

    let deserialize_buffer = zerompk::to_msgpack_vec(data).unwrap();
    let bdata = T::Borrowed::from(data);

    // The borrowed variant type should encode exactly the same as the owned type.
    let borrowed_buffer = zerompk::to_msgpack_vec(&bdata).unwrap();
    assert_eq!(borrowed_buffer, deserialize_buffer);

    // The borrowed value we decode should be equivalent to the input
    assert!(zerompk::from_msgpack::<T::Borrowed<'_>>(&deserialize_buffer).unwrap() == bdata);

    group.bench_function("borrow", |b| {
        b.iter(|| {
            black_box(
                zerompk::from_msgpack::<T::Borrowed<'_>>(black_box(&deserialize_buffer)).unwrap(),
            );
        })
    });

    group.finish();
}
