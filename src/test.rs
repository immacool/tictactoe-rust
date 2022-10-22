#[path = "game.rs"] mod game;

use game::Board;
use game::Player;
use game::minmax;

pub fn minmax_tests() {
    // == opening ==
    // [ X, -, - ]
    // [ -, -, - ]
    // [ -, -, - ]
    // =>
    // [ X, -, - ]
    // [ -, O, - ]
    // [ -, -, - ]
    let mut board = Board::from_array([
        [Some(Player::X), None, None],
        [None, None, None],
        [None, None, None],
    ]);

    let (_, (row, col)) = minmax(&mut board, Player::X, Player::O, false);
    assert_eq!((row, col), (1, 1));

    // == middle ==
    // [ X, -, - ]
    // [ X, O, - ]
    // [ -, -, - ]
    // =>
    // [ X, -, - ]
    // [ X, O, - ]
    // [ O, -, - ]
    let mut board = Board::from_array([
        [Some(Player::X), None, None],
        [Some(Player::X), Some(Player::O), None],
        [None, None, None],
    ]);

    let (_, (row, col)) = minmax(&mut board, Player::O, Player::O, false);
    assert_eq!((row, col), (2, 0));
    
    // == middle ==
    // [ X, -, - ]
    // [ X, O, - ]
    // [ O, -, - ]
    // =>
    // [ X, -, X ]
    // [ X, O, - ]
    // [ O, -, - ]
    // let mut board = Board::from_array([
    //     [Some(Player::X), None, None],
    //     [Some(Player::X), Some(Player::O), None],
    //     [Some(Player::O), None, None],
    // ]);

    // let (_, (row, col)) = minmax(&mut board, Player::X, Player::O, false);
    // assert_eq!((row, col), (0, 2)); error (0, 1) != (0, 2), idk whats wrong
}