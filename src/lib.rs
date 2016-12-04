extern crate rand;
extern crate rayon;

use self::rand::Rng;
use std::sync::{Arc, Mutex, RwLock};
use rayon::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum State {
	Dead,
	Alive,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Coord {
	x: usize,
	y: usize,
}

#[derive(Clone)]
pub struct Game {
	size: usize,
	coords: Arc< Vec<Coord> >,
	current: Arc<RwLock< Vec<State> >>,
	next: Arc<Mutex< Vec<State> >>,
}

impl From<Vec<State>> for Game {
	fn from(preset_game: Vec<State>) -> Self {
		let size: usize = (preset_game.len() as f64).sqrt() as usize;
		assert!(size >= 3);

		let mut coords = vec![Coord {x: 0, y: 0}; size*size];

		for (i, coord) in coords.iter_mut().enumerate() {
			coord.y = i/size;
			coord.x = i - coord.y * size;
		}

		Game {
			size: size,
			coords: Arc::new(coords),
			current: Arc::new(RwLock::new(preset_game)),
			next: Arc::new(Mutex::new(vec![State::Dead; size*size]))
		}
	}
}

impl Game {
	pub fn new(size: usize, percent_chance_for_cell_to_be_alive: f32) -> Game {
		assert!(size >= 3);
		assert!(percent_chance_for_cell_to_be_alive < 1.0);
		assert!(percent_chance_for_cell_to_be_alive > 0.0);

		let mut coords = vec![Coord {x: 0, y: 0}; size*size];
		let mut board  = vec![State::Dead; size*size];

		let mut rng = rand::thread_rng(); // Cache thread-local rng

		for (i, state) in board.iter_mut().enumerate() {
			coords[i].y = i/size;
			coords[i].x = i - coords[i].y * size;
			*state = match rng.next_f32() {
				i if i < percent_chance_for_cell_to_be_alive => State::Alive,
				_ => State::Dead,
			};
		}

		Game {
			size: size,
			coords: Arc::new(coords),
			current: Arc::new(RwLock::new(board)),
			next: Arc::new(Mutex::new(vec![State::Dead; size*size])),
		}
	}

	pub fn size(&self) -> usize {
		self.size
	}

	pub fn get_current_read_lock(&self) -> std::sync::RwLockReadGuard< Vec<State> > {
		self.current.read().unwrap()
	}

	pub fn advance(&self) {
		self.advance_with(Game::count_neighbors);
	}

	pub fn advance_toroidally(&self) {
		self.advance_with(Game::count_neighbors_toroidally);
	}

	fn advance_with<F>(&self, counting_func: F)
		where F: Fn(&Coord, &Vec<State>, usize) -> u8 + Sync + 'static {
		let mut next_guard = self.next.lock().unwrap();
		let current_guard = self.current.read().unwrap();

		self.coords.par_iter()
			.enumerate()
			.map(| (i, coord) | {
				let neighbor_count = counting_func(coord, &*current_guard, self.size);
				Game::calculate_next_state(current_guard[i], neighbor_count)
			})
			.collect_into(&mut *next_guard);

		// Exchange read lock for write lock
		drop(current_guard);
		let mut current_guard = self.current.write().unwrap();

		current_guard.clone_from(&*next_guard);
	}

	fn count_neighbors(coord: &Coord, board: &Vec<State>, size: usize) -> u8 {
		let ul = if coord.y >= 1 && coord.x != 0 { board[(coord.y - 1) * size + coord.x - 1] } else { State::Dead };
		let uu = if coord.y >= 1 { board[(coord.y - 1) * size + coord.x + 0] } else { State::Dead };
		let ur = if coord.y >= 1 && coord.x != size - 1 { board[(coord.y - 1) * size + coord.x + 1] } else { State::Dead };
		let l  = if coord.x != 0 { board[coord.y * size + coord.x - 1] } else { State::Dead };
		let r  = if coord.x != size - 1 { board[coord.y * size + coord.x + 1] } else { State::Dead };
		let dl = if coord.y != size - 1 && coord.x != 0 { board[(coord.y + 1) * size + coord.x - 1] } else { State::Dead };
		let dd = if coord.y != size - 1 { board[(coord.y + 1) * size + coord.x + 0] } else { State::Dead };
		let dr = if coord.y != size - 1 && coord.x != size - 1 { board[(coord.y + 1) * size + coord.x + 1] } else { State::Dead };

		let mut neighbors: u8 = 0;
		if ul == State::Alive { neighbors += 1; }
		if uu == State::Alive { neighbors += 1; }
		if ur == State::Alive { neighbors += 1; }
		if l  == State::Alive { neighbors += 1; }
		if r  == State::Alive { neighbors += 1; }
		if dl == State::Alive { neighbors += 1; }
		if dd == State::Alive { neighbors += 1; }
		if dr == State::Alive { neighbors += 1; }

		neighbors
	}

	fn count_neighbors_toroidally(coord: &Coord, board: &Vec<State>, size: usize) -> u8 {

		let ul = if coord.y >= 1 {
			if coord.x != 0 { board[(coord.y - 1) * size + coord.x - 1] } // normal case
			else { board[coord.y * size - 1] } // left edge
		} else {
			if coord.x != 0 { board[(size - 1) * size + coord.x - 1] } // top edge
			else { board[size * size - 1] } // top-left corner
		};


		let uu = if coord.y >= 1 {
			board[(coord.y - 1) * size + coord.x + 0] // normal case
		} else {
			board[(size - 1) * size + coord.x + 0] // top edge
		};

		let ur = if coord.y >= 1 {
			if coord.x != size - 1 { board[(coord.y - 1) * size + coord.x + 1] } // normal case
			else { board[(coord.y - 1) * size + 0] } // right edge
		} else {
			if coord.x != size - 1 { board[(size - 1) * size + coord.x + 1] } // top edge
			else { board[(size - 1) * size + 0] } // top-right corner
		};

		let l = if coord.x != 0 {
			board[coord.y * size + coord.x - 1] // normal case
		} else {
			board[(coord.y + 1) * size - 1] // left edge
		};

		let r = if coord.x != size - 1 {
			board[coord.y * size + coord.x + 1] // normal case
		} else {
			board[coord.y * size + 0] // right edge
		};

		let dl = if coord.y != size - 1 {
			if coord.x != 0 { board[(coord.y + 1) * size + coord.x - 1] } // normal case
			else { board[(coord.y + 2) * size - 1] } // left edge
		} else {
			if coord.x != 0 { board[coord.x - 1] } // bottom edge
			else { board[size - 1] } // bottom-left corner
		};

		let dd = if coord.y != size - 1 {
			board[(coord.y + 1) * size + coord.x + 0] // normal case
		} else {
			board[coord.x + 0] // bottom edge
		};

		let dr = if coord.y != size - 1 {
			if coord.x != size - 1 { board[(coord.y + 1) * size + coord.x + 1] } // normal case
			else { board[(coord.y + 1) * size] } // right edge
		} else {
			if coord.x != size - 1 { board[coord.x + 1] } // bottom edge
			else { board[0] } // bottom-right corner
		};

		let mut neighbors: u8 = 0;
		if ul == State::Alive { neighbors += 1; }
		if uu == State::Alive { neighbors += 1; }
		if ur == State::Alive { neighbors += 1; }
		if l  == State::Alive { neighbors += 1; }
		if r  == State::Alive { neighbors += 1; }
		if dl == State::Alive { neighbors += 1; }
		if dd == State::Alive { neighbors += 1; }
		if dr == State::Alive { neighbors += 1; }

		neighbors
	}

	fn calculate_next_state(state: State, number_of_neighbors: u8) -> State {
		match state {
			State::Dead => {
				if number_of_neighbors != 3 { State::Dead } else { State::Alive }
			},
			State::Alive => {
				if number_of_neighbors < 2 || number_of_neighbors > 3 { State::Dead } else { State::Alive }
			},
		}
	}
}
