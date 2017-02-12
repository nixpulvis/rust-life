extern crate test;

use self::test::Bencher;
use board::Board;
use rayon;

#[bench]
fn bench_random(b: &mut Bencher) {
    let brd = Board::new(200,200);
    b.iter(|| brd.random());
}

#[bench]
fn bench_ten_generations(b: &mut Bencher) {
    let mut brd = Board::new(200,200).random();
    b.iter(|| for _ in 0..10 { brd = brd.next_generation(); });
}

#[bench]
fn bench_ten_parallel_generations(b: &mut Bencher) {
    let mut brd = Board::new(200,200).random();
    rayon::initialize(rayon::Configuration::new()).unwrap();

    b.iter(|| for _ in 0..10 { brd = brd.parallel_next_generation(); });
}
