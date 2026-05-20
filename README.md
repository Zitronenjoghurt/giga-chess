[![](https://img.shields.io/crates/v/giga-chess)](https://crates.io/crates/giga-chess)
[![Tests](https://github.com/Zitronenjoghurt/giga-chess/actions/workflows/tests.yml/badge.svg)](https://github.com/Zitronenjoghurt/giga-chess/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/Zitronenjoghurt/giga-chess/graph/badge.svg?token=UM6T22YO17)](https://codecov.io/gh/Zitronenjoghurt/giga-chess)
![](http://tokei.lemon.industries/b1/github/Zitronenjoghurt/giga-chess?category=code&type=Rust&logo=https://simpleicons.org/icons/rust.svg)

# giga-chess

A rust chess library built for performance, handling game logic and legal/best move generation.

## Example

### Fools Mate

```text
8 ♜ ♞ ♝ ■ ♚ ♝ ♞ ♜
7 ♟ ♟ ♟ ♟ ■ ♟ ♟ ♟
6 □ ■ □ ■ ♟ ■ □ ■
5 ■ □ ■ □ ■ □ ■ □
4 □ ■ □ ■ □ ■ ♙ ♛
3 ■ □ ■ □ ■ ♙ ■ □
2 ♙ ♙ ♙ ♙ ♙ ■ □ ♙
1 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
  A B C D E F G H
```

```rust
pub use giga_chess::prelude::*;

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
```