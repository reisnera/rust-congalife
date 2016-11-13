#![feature(test)]
#[cfg(test)]

extern crate congalife;
extern crate test;

use congalife::{Game, advance};
use test::Bencher;

#[bench]
fn bench_advance_game_64(b: &mut Bencher) {
	let mut game: Game = Game::new(64);
    b.iter(|| advance(&mut game));
}

#[bench]
fn bench_advance_game_128(b: &mut Bencher) {
    let mut game: Game = Game::new(128);
    b.iter(|| advance(&mut game));
}

#[bench]
fn bench_advance_game_1024(b: &mut Bencher) {
    let mut game: Game = Game::new(1024);
    b.iter(|| advance(&mut game));
}