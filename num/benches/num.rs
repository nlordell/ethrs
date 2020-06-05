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

criterion_group!(num, arithmetic::<ethrs_num::u256>);
criterion_group!(uint, arithmetic::<primitive_types::U256>);
criterion_main!(num, uint);
