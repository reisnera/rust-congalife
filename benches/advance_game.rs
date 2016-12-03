#![feature(test)]
#[cfg(test)]

extern crate congalife;
extern crate test;

use congalife::Game;
use test::Bencher;

#[bench]
fn bench_advance_game_64(b: &mut Bencher) {
	let game: Game = Game::new(64, 0.2);
    b.iter(|| game.advance());
}

#[bench]
fn bench_advance_game_128(b: &mut Bencher) {
    let game: Game = Game::new(128, 0.2);
    b.iter(|| game.advance());
}

#[bench]
fn bench_advance_game_512(b: &mut Bencher) {
    let game: Game = Game::new(512, 0.2);
    b.iter(|| game.advance());
}

#[bench]
fn bench_advance_game_1024(b: &mut Bencher) {
    let game: Game = Game::new(1024, 0.2);
    b.iter(|| game.advance());
}

#[bench]
fn bench_advance_game_2048(b: &mut Bencher) {
    let game: Game = Game::new(2048, 0.2);
    b.iter(|| game.advance());
}

// #[bench]
// fn bench_advance_game_toroidally_64(b: &mut Bencher) {
// 	let game: Game = Game::new(64, 0.2);
//     b.iter(|| game.advance_toroidally());
// }

// #[bench]
// fn bench_advance_game_toroidally_128(b: &mut Bencher) {
//     let game: Game = Game::new(128, 0.2);
//     b.iter(|| game.advance_toroidally());
// }

// #[bench]
// fn bench_advance_game_toroidally_512(b: &mut Bencher) {
//     let game: Game = Game::new(512, 0.2);
//     b.iter(|| game.advance_toroidally());
// }
