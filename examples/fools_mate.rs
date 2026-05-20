//! A simple example of playing a fool's mate.
//! ```text
//!  8 ♜ ♞ ♝ ■ ♚ ♝ ♞ ♜
//!  7 ♟ ♟ ♟ ♟ ■ ♟ ♟ ♟
//!  6 □ ■ □ ■ ♟ ■ □ ■
//!  5 ■ □ ■ □ ■ □ ■ □
//!  4 □ ■ □ ■ □ ■ ♙ ♛
//!  3 ■ □ ■ □ ■ ♙ ■ □
//!  2 ♙ ♙ ♙ ♙ ♙ ■ □ ♙
//!  1 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
//!    A B C D E F G H
//!  ```
use giga_chess::prelude::*;

fn main() {
    let mut game = Game::default();
    game.play(F2, F3).unwrap();
    game.play(E7, E6).unwrap();
    game.play(G2, G4).unwrap();
    game.play(D8, H4).unwrap();
    println!("{}", game.pretty_grid());

    assert_eq!(
        game.outcome(),
        Some(GameOutcome::Decisive {
            winner: Color::Black,
            reason: DecisiveReason::Checkmate
        })
    );
}
