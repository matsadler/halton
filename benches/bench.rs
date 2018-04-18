#![feature(test)]

extern crate halton;
extern crate test;

#[bench]
fn bench_number_base_two(b: &mut test::Bencher) {
    let mut i = 0;
    b.iter(|| {
        let res = halton::number(2, i);
        i += 1;
        res
    })
}

#[bench]
fn bench_sequence_base_two(b: &mut test::Bencher) {
    let mut seq = halton::Sequence::new(2);
    b.iter(|| seq.next())
}

#[bench]
fn bench_number_base_seventeen(b: &mut test::Bencher) {
    let mut i = 0;
    b.iter(|| {
        let res = halton::number(17, i);
        i += 1;
        res
    })
}

#[bench]
fn bench_sequence_base_seventeen(b: &mut test::Bencher) {
    let mut seq = halton::Sequence::new(17);
    b.iter(|| seq.next())
}

#[bench]
fn bench_number_one_million_base_two(b: &mut test::Bencher) {
    b.iter(|| {
        let mut res = 0.0;
        for i in 0..1_000_000 {
            res = halton::number(2, i);
        }
        res
    })
}

#[bench]
fn bench_sequence_one_million_base_two(b: &mut test::Bencher) {
    b.iter(|| {
        let mut res = 0.0;
        for i in halton::Sequence::new(2).take(1_000_000) {
            res = i;
        }
        res
    })
}

#[bench]
fn bench_number_one_million_base_seventeen(b: &mut test::Bencher) {
    b.iter(|| {
        let mut res = 0.0;
        for i in 0..1_000_000 {
            res = halton::number(17, i);
        }
        res
    })
}

#[bench]
fn bench_sequence_one_million_base_seventeen(b: &mut test::Bencher) {
    b.iter(|| {
        let mut res = 0.0;
        for i in halton::Sequence::new(17).take(1_000_000) {
            res = i;
        }
        res
    })
}
