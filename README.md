[![](https://img.shields.io/crates/v/giga-chess)](https://crates.io/crates/giga-chess)
[![Tests](https://github.com/Zitronenjoghurt/giga-chess/actions/workflows/tests.yml/badge.svg)](https://github.com/Zitronenjoghurt/giga-chess/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/Zitronenjoghurt/giga-chess/graph/badge.svg?token=UM6T22YO17)](https://codecov.io/gh/Zitronenjoghurt/giga-chess)
![](http://tokei.lemon.industries/b1/github/Zitronenjoghurt/giga-chess?category=code&type=Rust&logo=https://simpleicons.org/icons/rust.svg)

# giga-chess

A rust chess library built for performance, handling game logic and legal/best move generation.

## Example

```rust
use giga_chess::prelude::*;

fn main() {
    let engine = Engine::initialize();

    let mut game = Game::new(&engine, PGNMetadata::now());
    let moves = game.legal_moves();

    // Choose some kind of move
    let chosen_move = *moves.iter().nth(0).unwrap();
    game.play_move(&engine, chosen_move);

    println!("{}", game.board().to_string());
}
```