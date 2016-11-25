extern crate rand;

use self::rand::{thread_rng, Rng};
use std::sync::{Arc, Mutex, RwLock};

#[derive(Copy, Clone)]
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

#[derive(Clone)]
pub struct Game {
	pub size: usize,
	current: Arc<RwLock<Vec<State>>>,
	next: Arc<Mutex<Vec<State>>>,
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
			current: Arc::new(RwLock::new(board.clone())),
			next: Arc::new(Mutex::new(board.clone())),
		}
	}

	pub fn get_current_read_lock(&self) -> std::sync::RwLockReadGuard<Vec<State>> {
		self.current.read().unwrap()
	}

	pub fn advance(&self) {
		// Get read lock for 'current' and mutex lock for 'next'
		// N.B. By locking 'next' first, this ensures that if another thread tries
		// 		to call 'advance', that thread will block until this is done.
		let mut next = self.next.lock().unwrap();
		let current = self.current.read().unwrap();

		for y in 0..self.size {
			for x in 0..self.size {
				let ul = if y >= 1 && x != 0 { current[(y - 1) * self.size + x - 1].clone()} else {State::Dead};
				let uu = if y >= 1 {current[(y - 1) * self.size + x + 0].clone()} else {State::Dead};
				let ur = if y >= 1 && x != self.size - 1 { current[(y - 1) * self.size + x + 1].clone()} else {State::Dead};
				let l = if x != 0 {current[y * self.size + x - 1].clone()} else {State::Dead};
				let r = if x != self.size - 1 {current[y * self.size + x + 1].clone()} else {State::Dead};
				let dl = if y != self.size - 1 && x != 0 {current[(y + 1) * self.size + x - 1].clone()} else {State::Dead};
				let dd = if y != self.size - 1 {current[(y + 1) * self.size + x + 0].clone()} else {State::Dead};
				let dr = if y != self.size - 1 && x != self.size - 1 {current[(y + 1) * self.size + x + 1].clone()} else {State::Dead};

				let neighbors = vec![ul, uu, ur, l, r, dl, dd, dr].into_iter().filter(is_alive).count();

				next[y * self.size + x] = match current[y * self.size + x] {
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

		// Exchange read lock for write lock
		drop(current);
		let mut current = self.current.write().unwrap();

		*current = next.clone();
	}

}
