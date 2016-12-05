extern crate congalife;

use congalife::{Game, State};

#[test]
fn test_preserves_square_on_corner() {
    let start = vec![
        State::Alive, State::Alive, State::Dead,
        State::Alive, State::Alive, State::Dead,
        State::Dead, State::Dead, State::Dead
    ];
	let game: Game = Game::from(start.clone());
    game.advance_toroidally();

    let new_game = game.get_current_read_lock();
    assert_eq!(*new_game, start);
}

#[test]
fn test_x_pattern_around_edges() {
    let start = vec![
        State::Alive, State::Dead, State::Alive,
        State::Dead, State::Alive, State::Dead,
        State::Alive, State::Dead, State::Alive
    ];
    let game: Game = Game::from(start.clone());
    game.advance_toroidally();

    let new_game = game.get_current_read_lock();
    assert_eq!(*new_game, vec![
        State::Dead, State::Dead, State::Dead,
        State::Dead, State::Dead, State::Dead,
        State::Dead, State::Dead, State::Dead
    ]);
}

#[test]
fn test_blinker() {
    let start = vec![
        State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
        State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
        State::Dead, State::Alive, State::Alive, State::Alive, State::Dead,
        State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
        State::Dead, State::Dead, State::Dead, State::Dead, State::Dead
    ];
    let game: Game = Game::from(start.clone());
    game.advance_toroidally();

    let new_game = game.get_current_read_lock();
    assert_eq!(*new_game, vec![
        State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
        State::Dead, State::Dead, State::Alive, State::Dead, State::Dead,
        State::Dead, State::Dead, State::Alive, State::Dead, State::Dead,
        State::Dead, State::Dead, State::Alive, State::Dead, State::Dead,
        State::Dead, State::Dead, State::Dead, State::Dead, State::Dead
    ]);
}

#[test]
fn test_glider() {
    let start = vec![
        State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
        State::Dead, State::Dead, State::Alive, State::Dead, State::Dead,
        State::Dead, State::Dead, State::Dead, State::Alive, State::Dead,
        State::Dead, State::Alive, State::Alive, State::Alive, State::Dead,
        State::Dead, State::Dead, State::Dead, State::Dead, State::Dead
    ];
    let game: Game = Game::from(start.clone());
    game.advance_toroidally();

    let new_game = game.get_current_read_lock();
    assert_eq!(*new_game, vec![
        State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
        State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
        State::Dead, State::Alive, State::Dead, State::Alive, State::Dead,
        State::Dead, State::Dead, State::Alive, State::Alive, State::Dead,
        State::Dead, State::Dead, State::Alive, State::Dead, State::Dead
    ]);
}

#[test]
fn test_glider_across_corner() {
    let start = vec![
        State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
        State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
        State::Dead, State::Dead, State::Dead, State::Alive, State::Dead,
        State::Dead, State::Dead, State::Dead, State::Dead, State::Alive,
        State::Dead, State::Dead, State::Alive, State::Alive, State::Alive
    ];
    let game: Game = Game::from(start.clone());

    game.advance_toroidally();
    let _next = {
        let new_game = game.get_current_read_lock();
        assert_eq!(*new_game, vec![
            State::Dead, State::Dead, State::Dead, State::Alive, State::Dead,
            State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
            State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
            State::Dead, State::Dead, State::Alive, State::Dead, State::Alive,
            State::Dead, State::Dead, State::Dead, State::Alive, State::Alive
        ]);
    };

    game.advance_toroidally();
    let _next = {
        let new_game = game.get_current_read_lock();
        assert_eq!(*new_game, vec![
            State::Dead, State::Dead, State::Dead, State::Alive, State::Alive,
            State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
            State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
            State::Dead, State::Dead, State::Dead, State::Dead, State::Alive,
            State::Dead, State::Dead, State::Alive, State::Dead, State::Alive
        ]);
    };

    game.advance_toroidally();
    let _next = {
        let new_game = game.get_current_read_lock();
        assert_eq!(*new_game, vec![
            State::Dead, State::Dead, State::Dead, State::Alive, State::Alive,
            State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
            State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
            State::Dead, State::Dead, State::Dead, State::Alive, State::Dead,
            State::Alive, State::Dead, State::Dead, State::Dead, State::Alive
        ]);
    };

    game.advance_toroidally();
    let _next = {
        let new_game = game.get_current_read_lock();
        assert_eq!(*new_game, vec![
            State::Alive, State::Dead, State::Dead, State::Alive, State::Alive,
            State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
            State::Dead, State::Dead, State::Dead, State::Dead, State::Dead,
            State::Dead, State::Dead, State::Dead, State::Dead, State::Alive,
            State::Alive, State::Dead, State::Dead, State::Dead, State::Dead
        ]);
    };
}
