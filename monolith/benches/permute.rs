use core::array;

use criterion::{Criterion, criterion_group, criterion_main};
use p3_field::PrimeCharacteristicRing;
use p3_mersenne_31::Mersenne31;
use p3_monolith::{MonolithMdsMatrixMersenne31, MonolithMersenne31};

fn bench_monolith(c: &mut Criterion) {
    monolith::<12>(c);
    monolith::<16>(c);
}

fn monolith<const WIDTH: usize>(c: &mut Criterion)
{
    let mds = MonolithMdsMatrixMersenne31::<5>;
    let monolith: MonolithMersenne31<WIDTH, 5> = MonolithMersenne31::new(mds);

    let mut input = array::from_fn(Mersenne31::from_usize);

    let name = format!("monolith::<Mersenne31, {}>", WIDTH);
    c.bench_function(name.as_str(), |b| {
        b.iter(|| monolith.permutation(&mut input))
    });
}

criterion_group!(benches, bench_monolith);
criterion_main!(benches);
