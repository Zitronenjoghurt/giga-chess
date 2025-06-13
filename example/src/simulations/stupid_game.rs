use giga_chess::engine::Engine;
use giga_chess::game::color::Color;
use giga_chess::game::Game;
use rand::prelude::IndexedRandom;
use std::thread;
use std::time::Duration;

pub fn run_stupid_game(engine: &Engine, delay_ms: u64) -> impl Iterator<Item = String> + '_ {
    let mut game = Game::new(Color::White);

    std::iter::from_fn(move || {
        let moves = engine.generate_moves(&game);
        let chosen = moves.choose(&mut rand::rng())?;
        game.play_move(*chosen);

        thread::sleep(Duration::from_millis(delay_ms));

        Some(format!(
            "{:?}: {}\n{}",
            game.side_to_move.opposite(),
            chosen,
            game.board.to_string()
        ))
    })
}
