use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::any;
use std::ops::*;

fn arithmetic<U: Add + Mul + Sub + Copy + From<u128>>(c: &mut Criterion) {
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
}

criterion_group!(num, arithmetic::<num::u256>);
criterion_group!(uint, arithmetic::<primitive_types::U256>);
criterion_main!(num, uint);
