extern crate rand;

use self::rand::{thread_rng, Rng};

#[derive(Clone)]
pub enum State {
	Dead,
	Alive,
}

fn is_alive(state: &State) -> bool {
	match state {
		&State::Alive => true,
		&State::Dead => false,
	}
}

pub struct Game {
	pub size: usize,
	current: Vec<State>,
	next: Vec<State>,
}

impl Game {
	pub fn new(size: usize) -> Game {
		let mut board: Vec<State> = vec![State::Dead; size * size];
				
		for index in 0..size*size {
            board[index] = match thread_rng().gen::<bool>() {
            	true => State::Alive,
            	false => State::Dead,
            };
        }

		Game {
			size: size,
			current: board.clone(),
			next: board.clone(),
		}
	}

	pub fn get_current(&self) -> &[State] {
		self.current.as_slice()
	}
}

pub fn advance(game: &mut Game) {
	for y in 0..game.size {
		for x in 0..game.size {
			let ul = if y >= 1 && x != 0 { game.current[(y - 1) * game.size + x - 1].clone()} else {State::Dead};
			let uu = if y >= 1 {game.current[(y - 1) * game.size + x + 0].clone()} else {State::Dead};
			let ur = if y >= 1 && x != game.size - 1 { game.current[(y - 1) * game.size + x + 1].clone()} else {State::Dead};
			let l = if x != 0 {game.current[y * game.size + x - 1].clone()} else {State::Dead};
			let r = if x != game.size - 1 {game.current[y * game.size + x + 1].clone()} else {State::Dead};
			let dl = if y != game.size - 1 && x != 0 {game.current[(y + 1) * game.size + x - 1].clone()} else {State::Dead};
			let dd = if y != game.size - 1 {game.current[(y + 1) * game.size + x + 0].clone()} else {State::Dead};
			let dr = if y != game.size - 1 && x != game.size - 1 {game.current[(y + 1) * game.size + x + 1].clone()} else {State::Dead};

			let neighbors = vec![ul, uu, ur, l, r, dl, dd, dr].into_iter().filter(is_alive).count();

			game.next[y * game.size + x] = match game.current[y * game.size + x] {
				State::Dead => {
					if neighbors == 3 {
						State::Alive
					} else {
						State::Dead
					}
				},
				State::Alive => {
					if neighbors < 2 || neighbors > 3 {
						State::Dead
					} else {
						State::Alive
					}
				}
			};
		}
	}

	game.current.clone_from(&game.next);
}