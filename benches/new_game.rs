#![feature(test)]
#[cfg(test)]

extern crate congalife;
extern crate test;

use congalife::Game;
use test::Bencher;

#[bench]
fn bench_new_game_64(b: &mut Bencher) {
    b.iter(|| Game::new(64, 0.2));
}

#[bench]
fn bench_new_game_128(b: &mut Bencher) {
    b.iter(|| Game::new(128, 0.2));
}

#[bench]
fn bench_new_game_512(b: &mut Bencher) {
    b.iter(|| Game::new(512, 0.2));
}

#[bench]
fn bench_new_game_1024(b: &mut Bencher) {
    b.iter(|| Game::new(1024, 0.2));
}
