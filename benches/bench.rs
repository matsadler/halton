#![feature(test)]

extern crate halton;
extern crate test;

fn baseline(base: u8, index: usize) -> f64 {
    let mut index = index;
    let mut result = 0.0;
    let mut factor = 1.0 / base as f64;
    while index > 0 {
        result += factor * (index % base as usize) as f64;
        factor /= base as f64;
        index /= base as usize;
    }
    result
}

#[bench]
fn bench_base_two_baseline(b: &mut test::Bencher) {
    let mut i = 0;
    b.iter(|| {
        let res = baseline(2, i);
        i += 1;
        res
    })
}

#[bench]
fn bench_base_two(b: &mut test::Bencher) {
    let mut seq = halton::Sequence::new(2);
    b.iter(|| seq.next())
}

#[bench]
fn bench_base_seventeen_baseline(b: &mut test::Bencher) {
    let mut i = 0;
    b.iter(|| {
        let res = baseline(17, i);
        i += 1;
        res
    })
}

#[bench]
fn bench_base_seventeen(b: &mut test::Bencher) {
    let mut seq = halton::Sequence::new(17);
    b.iter(|| seq.next())
}

#[bench]
fn bench_one_million_base_two_baseline(b: &mut test::Bencher) {
    b.iter(|| {
        let mut res = 0.0;
        for i in 0..1_000_000 {
            res = baseline(2, i);
        }
        res
    })
}

#[bench]
fn bench_one_million_base_two(b: &mut test::Bencher) {
    b.iter(|| {
        let mut res = 0.0;
        for i in halton::Sequence::new(2).take(1_000_000) {
            res = i;
        }
        res
    })
}

#[bench]
fn bench_one_million_base_seventeen_baseline(b: &mut test::Bencher) {
    b.iter(|| {
        let mut res = 0.0;
        for i in 0..1_000_000 {
            res = baseline(17, i);
        }
        res
    })
}

#[bench]
fn bench_one_million_base_seventeen(b: &mut test::Bencher) {
    b.iter(|| {
        let mut res = 0.0;
        for i in halton::Sequence::new(17).take(1_000_000) {
            res = i;
        }
        res
    })
}
