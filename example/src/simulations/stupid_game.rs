use giga_chess::engine::Engine;
use giga_chess::game::color::Color;
use giga_chess::game::Game;
use rand::prelude::IteratorRandom;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn run_stupid_game(engine: &Arc<Engine>, delay_ms: u64) -> impl Iterator<Item = String> + '_ {
    let mut game = Game::new(engine, Color::White);

    std::iter::from_fn(move || {
        let moves = game.legal_moves();
        if moves.is_empty() {
            println!(
                "Winner: {:?}, Status: {:?}\n{}",
                game.winner(),
                game.status(),
                game.get_fen_string()
            );
            return None;
        }

        let chosen = *moves.iter().choose(&mut rand::rng())?;
        game.play_move(engine, chosen);

        thread::sleep(Duration::from_millis(delay_ms));

        Some(format!(
            "# {} | {:?}: {}\n{}",
            game.full_moves(),
            game.side_to_move().opposite(),
            chosen,
            game.board().to_string()
        ))
    })
}
