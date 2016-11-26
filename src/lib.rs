extern crate rand;

use self::rand::{thread_rng, Rng};
use std::sync::{Arc, Mutex, RwLock};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum State {
	Dead,
	Alive,
}

#[derive(Clone)]
pub struct Game {
	pub size: usize,
	current: Arc<RwLock<Vec<State>>>,
	next: Arc<Mutex<Vec<State>>>,
}

impl From<Vec<State>> for Game {
	fn from(preset_game: Vec<State>) -> Self {
		let size: usize = (preset_game.len() as f64).sqrt() as usize;
		assert!(size >= 3);

		Game {
			size: size,
			current: Arc::new(RwLock::new(preset_game.clone())),
			next: Arc::new(Mutex::new(preset_game.clone()))
		}
	}
}

impl Game {
	pub fn new(size: usize, percent_chance_for_cell_to_be_alive: f64) -> Game {
		assert!(size >= 3);
		assert!(percent_chance_for_cell_to_be_alive < 1.0);
		assert!(percent_chance_for_cell_to_be_alive >= 0.0);

		let mut board: Vec<State> = vec![State::Dead; size * size];
				
		for index in 0..size*size {
			board[index] = match thread_rng().next_f64() {
				i if i < percent_chance_for_cell_to_be_alive => State::Alive,
				_ => State::Dead,
			};
		}

		Game {
			size: size,
			current: Arc::new(RwLock::new(board.clone())),
			next: Arc::new(Mutex::new(board.clone())),
		}
	}

	pub fn get_current_read_lock(&self) -> std::sync::RwLockReadGuard<Vec<State>> {
		self.current.read().unwrap()
	}

	fn get_next_state_from_current_state_and_neighbors(state: State, number_of_neighbors: usize) -> State {
		match state {
			State::Dead => {
				if number_of_neighbors != 3 {
					State::Dead
				} else {
					State::Alive
				}
			},
			State::Alive => {
				if number_of_neighbors < 2 || number_of_neighbors > 3 {
					State::Dead
				} else {
					State::Alive
				}
			}
		}
	}

	pub fn advance(&self) {
		// Get read lock for 'current' and mutex lock for 'next'
		// N.B. By locking 'next' first, this ensures that if another thread tries
		// 		to call 'advance', that thread will block until this is done.
		let mut next = self.next.lock().unwrap();
		let current = self.current.read().unwrap();

		for y in 0..self.size {
			for x in 0..self.size {
				let ul = if y >= 1 && x != 0 { current[(y - 1) * self.size + x - 1]} else {State::Dead};
				let uu = if y >= 1 {current[(y - 1) * self.size + x + 0]} else {State::Dead};
				let ur = if y >= 1 && x != self.size - 1 { current[(y - 1) * self.size + x + 1]} else {State::Dead};
				let l = if x != 0 {current[y * self.size + x - 1]} else {State::Dead};
				let r = if x != self.size - 1 {current[y * self.size + x + 1]} else {State::Dead};
				let dl = if y != self.size - 1 && x != 0 {current[(y + 1) * self.size + x - 1]} else {State::Dead};
				let dd = if y != self.size - 1 {current[(y + 1) * self.size + x + 0]} else {State::Dead};
				let dr = if y != self.size - 1 && x != self.size - 1 {current[(y + 1) * self.size + x + 1]} else {State::Dead};

				let mut neighbors: usize = 0;
				if ul == State::Alive { neighbors += 1; }
				if uu == State::Alive { neighbors += 1; }
				if ur == State::Alive { neighbors += 1; }
				if l == State::Alive { neighbors += 1; }
				if r == State::Alive { neighbors += 1; }
				if dl == State::Alive { neighbors += 1; }
				if dd == State::Alive { neighbors += 1; }
				if dr == State::Alive { neighbors += 1; }

				next[y * self.size + x] = Game::get_next_state_from_current_state_and_neighbors(current[y * self.size + x], neighbors);
			}
		}

		// Exchange read lock for write lock
		drop(current);
		let mut current = self.current.write().unwrap();

		*current = next.clone();
	}

	pub fn advance_toroidally(&self) {
		// Get read lock for 'current' and mutex lock for 'next'
		// N.B. By locking 'next' first, this ensures that if another thread tries
		// 		to call 'advance', that thread will block until this is done.
		let mut next = self.next.lock().unwrap();
		let current = self.current.read().unwrap();

		for y in 0..self.size {
			for x in 0..self.size {
				let ul = if y >= 1 {
					if x != 0 { current[(y - 1) * self.size + x - 1] } // normal case
					else { current[y * self.size - 1] } // left edge
				} else {
					if x != 0 { current[(self.size - 1) * self.size + x - 1] } // top edge
					else { current[self.size * self.size - 1] } // top-left corner
				};


				let uu = if y >= 1 {
					current[(y - 1) * self.size + x + 0] // normal case
				} else {
					current[(self.size - 1) * self.size + x + 0] // top edge
				};

				let ur = if y >= 1 {
					if x != self.size - 1 { current[(y - 1) * self.size + x + 1] } // normal case
					else { current[(y - 1) * self.size + 0] } // right edge
				} else {
					if x != self.size - 1 { current[(self.size - 1) * self.size + x + 1] } // top edge
					else { current[(self.size - 1) * self.size + 0] } // top-right corner
				};

				let l = if x != 0 {
					current[y * self.size + x - 1] // normal case
				} else {
					current[(y + 1) * self.size - 1] // left edge
				};

				let r = if x != self.size - 1 {
					current[y * self.size + x + 1] // normal case
				} else {
					current[y * self.size + 0] // right edge
				};

				let dl = if y != self.size - 1 {
					if x != 0 { current[(y + 1) * self.size + x - 1] } // normal case
					else { current[(y + 2) * self.size - 1] } // left edge
				} else {
					if x != 0 { current[x - 1] } // bottom edge
					else { current[self.size - 1] } // bottom-left corner
				};

				let dd = if y != self.size - 1 {
					current[(y + 1) * self.size + x + 0] // normal case
				} else {
					current[x + 0] // bottom edge
				};

				let dr = if y != self.size - 1 {
					if x != self.size - 1 { current[(y + 1) * self.size + x + 1] } // normal case
					else { current[(y + 1) * self.size] } // right edge
				} else {
					if x != self.size - 1 { current[x + 1] } // bottom edge
					else { current[0] } // bottom-right corner
				};

				let mut neighbors: usize = 0;
				if ul == State::Alive { neighbors += 1; }
				if uu == State::Alive { neighbors += 1; }
				if ur == State::Alive { neighbors += 1; }
				if l == State::Alive { neighbors += 1; }
				if r == State::Alive { neighbors += 1; }
				if dl == State::Alive { neighbors += 1; }
				if dd == State::Alive { neighbors += 1; }
				if dr == State::Alive { neighbors += 1; }

				next[y * self.size + x] = Game::get_next_state_from_current_state_and_neighbors(current[y * self.size + x], neighbors);
			}
		}

		// Exchange read lock for write lock
		drop(current);
		let mut current = self.current.write().unwrap();

		*current = next.clone();
	}
}
