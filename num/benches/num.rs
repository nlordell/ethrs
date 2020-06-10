use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::any;
use std::ops::*;

fn arithmetic<U>(c: &mut Criterion)
where
    U: Add + Mul + Sub + Shl + Shr + Copy + From<u128>,
{
    let value = U::from(u128::MAX);

    c.bench_function(&format!("{}::add", any::type_name::<U>()), |b| {
        b.iter(|| black_box(value) + black_box(value))
    });

    c.bench_function(&format!("{}::mul", any::type_name::<U>()), |b| {
        b.iter(|| black_box(value) * black_box(value))
    });

    c.bench_function(&format!("{}::sub", any::type_name::<U>()), |b| {
        b.iter(|| black_box(value) - black_box(value))
    });

    c.bench_function(&format!("{}::shl", any::type_name::<U>()), |b| {
        b.iter(|| black_box(value) << black_box(U::from(21)))
    });

    c.bench_function(&format!("{}::shr", any::type_name::<U>()), |b| {
        b.iter(|| black_box(value) >> black_box(U::from(21)))
    });
}

fn intrinsics(c: &mut Criterion) {
    let value = ethrs_num::u256::new(u128::MAX) << 2u32;

    c.bench_function("u256::rotate_left", |b| {
        b.iter(|| black_box(value).rotate_left(black_box(21)))
    });

    c.bench_function("u256::rotate_right", |b| {
        b.iter(|| black_box(value).rotate_right(black_box(21)))
    });

    c.bench_function("u256::ctlz", |b| {
        b.iter(|| black_box(value).leading_zeros())
    });

    c.bench_function("u256::cttz", |b| {
        b.iter(|| black_box(value).trailing_zeros())
    });
}

criterion_group!(num, arithmetic::<ethrs_num::u256>, intrinsics);
criterion_group!(uint, arithmetic::<primitive_types::U256>);
criterion_main!(num, uint);
