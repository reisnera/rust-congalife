extern crate rand;
extern crate rayon;
// extern crate crossbeam;

use self::rand::Rng;
use std::sync::{Arc, Mutex, RwLock};
use rayon::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Coord {
	x: usize,
	y: usize,
}

#[derive(Clone)]
pub struct Game {
	size: usize,
	coords: Arc< Vec<Coord> >,
	current: Arc<RwLock< Vec<bool> >>,
	next: Arc<Mutex< Vec<bool> >>,
}

// impl From<Vec<State>> for Game {
// 	fn from(preset_game: Vec<State>) -> Self {
// 		let size: usize = (preset_game.len() as f64).sqrt() as usize;
// 		assert!(size >= 3);

// 		Game {
// 			size: size,
// 			current: Arc::new(RwLock::new(preset_game.clone())),
// 			next: Arc::new(Mutex::new(preset_game.clone()))
// 		}
// 	}
// }

impl Game {
	pub fn new(size: usize, percent_chance_for_cell_to_be_alive: f32) -> Game {
		assert!(size >= 3);
		assert!(percent_chance_for_cell_to_be_alive < 1.0);
		assert!(percent_chance_for_cell_to_be_alive > 0.0);

		let mut coords = vec![Coord {x: 0, y: 0}; size*size];
		let mut board  = vec![false; size*size];

		let mut rng = rand::thread_rng(); // Cache thread-local rng

		for (i, state) in board.iter_mut().enumerate() {
			coords[i].y = i/size;
			coords[i].x = i - coords[i].y * size;
			*state = match rng.next_f32() {
				i if i < percent_chance_for_cell_to_be_alive => true,
				_ => false,
			};
		}

		Game {
			size: size,
			coords: Arc::new(coords),
			current: Arc::new(RwLock::new(board)),
			next: Arc::new(Mutex::new(Vec::with_capacity(size))),
		}
	}

	pub fn size(&self) -> usize {
		self.size
	}

	pub fn get_current_read_lock(&self) -> std::sync::RwLockReadGuard< Vec<bool> > {
		self.current.read().unwrap()
	}

	pub fn advance(&self) {
		let mut next_guard = self.next.lock().unwrap();
		let current_guard = self.current.read().unwrap();

		self.coords.par_iter()
			.enumerate()
			.map(| (i, coord) | {
				let neighbor_count = self.count_neighbors(coord, &*current_guard);
				Game::calculate_next_state(current_guard[i], neighbor_count)
			})
			.collect_into(&mut *next_guard);

		// Exchange read lock for write lock
		drop(current_guard);
		let mut current_guard = self.current.write().unwrap();

		current_guard.par_iter_mut()
			.enumerate()
			.for_each(| (i, state) | {
				*state = next_guard[i];
			});
	}

	fn count_neighbors(&self, coord: &Coord, board: &Vec<bool>) -> u8 {
		let ul = if coord.y >= 1 && coord.x != 0 { board[(coord.y - 1) * self.size + coord.x - 1] } else { false };
		let uu = if coord.y >= 1 { board[(coord.y - 1) * self.size + coord.x + 0] } else { false };
		let ur = if coord.y >= 1 && coord.x != self.size - 1 { board[(coord.y - 1) * self.size + coord.x + 1] } else { false };
		let l  = if coord.x != 0 { board[coord.y * self.size + coord.x - 1] } else { false };
		let r  = if coord.x != self.size - 1 { board[coord.y * self.size + coord.x + 1] } else { false };
		let dl = if coord.y != self.size - 1 && coord.x != 0 { board[(coord.y + 1) * self.size + coord.x - 1] } else { false };
		let dd = if coord.y != self.size - 1 { board[(coord.y + 1) * self.size + coord.x + 0] } else { false };
		let dr = if coord.y != self.size - 1 && coord.x != self.size - 1 { board[(coord.y + 1) * self.size + coord.x + 1] } else { false };

		let mut neighbors: u8 = 0;
		if ul == true { neighbors += 1; }
		if uu == true { neighbors += 1; }
		if ur == true { neighbors += 1; }
		if l  == true { neighbors += 1; }
		if r  == true { neighbors += 1; }
		if dl == true { neighbors += 1; }
		if dd == true { neighbors += 1; }
		if dr == true { neighbors += 1; }

		neighbors
	}

	// Calculates the next state for this cell (but does not set it!)
	fn calculate_next_state(state: bool, number_of_neighbors: u8) -> bool {
		match state {
			false => {
				if number_of_neighbors != 3 { false } else { true }
			},
			true => {
				if number_of_neighbors < 2 || number_of_neighbors > 3 { false } else { true }
			},
		}
	}

	// pub fn advance_toroidally(&self) {
	// 	// Get read lock for 'current' and mutex lock for 'next'
	// 	// N.B. By locking 'next' first, this ensures that if another thread tries
	// 	// 		to call 'advance', that thread will block until this is done.
	// 	let mut next = self.next.lock().unwrap();
	// 	let current = self.current.read().unwrap();

	// 	for y in 0..self.size {
	// 		for x in 0..self.size {
	// 			let ul = if y >= 1 {
	// 				if x != 0 { current[(y - 1) * self.size + x - 1] } // normal case
	// 				else { current[y * self.size - 1] } // left edge
	// 			} else {
	// 				if x != 0 { current[(self.size - 1) * self.size + x - 1] } // top edge
	// 				else { current[self.size * self.size - 1] } // top-left corner
	// 			};


	// 			let uu = if y >= 1 {
	// 				current[(y - 1) * self.size + x + 0] // normal case
	// 			} else {
	// 				current[(self.size - 1) * self.size + x + 0] // top edge
	// 			};

	// 			let ur = if y >= 1 {
	// 				if x != self.size - 1 { current[(y - 1) * self.size + x + 1] } // normal case
	// 				else { current[(y - 1) * self.size + 0] } // right edge
	// 			} else {
	// 				if x != self.size - 1 { current[(self.size - 1) * self.size + x + 1] } // top edge
	// 				else { current[(self.size - 1) * self.size + 0] } // top-right corner
	// 			};

	// 			let l = if x != 0 {
	// 				current[y * self.size + x - 1] // normal case
	// 			} else {
	// 				current[(y + 1) * self.size - 1] // left edge
	// 			};

	// 			let r = if x != self.size - 1 {
	// 				current[y * self.size + x + 1] // normal case
	// 			} else {
	// 				current[y * self.size + 0] // right edge
	// 			};

	// 			let dl = if y != self.size - 1 {
	// 				if x != 0 { current[(y + 1) * self.size + x - 1] } // normal case
	// 				else { current[(y + 2) * self.size - 1] } // left edge
	// 			} else {
	// 				if x != 0 { current[x - 1] } // bottom edge
	// 				else { current[self.size - 1] } // bottom-left corner
	// 			};

	// 			let dd = if y != self.size - 1 {
	// 				current[(y + 1) * self.size + x + 0] // normal case
	// 			} else {
	// 				current[x + 0] // bottom edge
	// 			};

	// 			let dr = if y != self.size - 1 {
	// 				if x != self.size - 1 { current[(y + 1) * self.size + x + 1] } // normal case
	// 				else { current[(y + 1) * self.size] } // right edge
	// 			} else {
	// 				if x != self.size - 1 { current[x + 1] } // bottom edge
	// 				else { current[0] } // bottom-right corner
	// 			};

	// 			let mut neighbors: usize = 0;
	// 			if ul == State::Alive { neighbors += 1; }
	// 			if uu == State::Alive { neighbors += 1; }
	// 			if ur == State::Alive { neighbors += 1; }
	// 			if l == State::Alive { neighbors += 1; }
	// 			if r == State::Alive { neighbors += 1; }
	// 			if dl == State::Alive { neighbors += 1; }
	// 			if dd == State::Alive { neighbors += 1; }
	// 			if dr == State::Alive { neighbors += 1; }

	// 			next[y * self.size + x] = Game::get_next_cell_from_current_cell_and_neighbors(current[y * self.size + x], neighbors);
	// 		}
	// 	}

	// 	// Exchange read lock for write lock
	// 	drop(current);
	// 	let mut current = self.current.write().unwrap();

	// 	*current = next.clone();
	// }
}
