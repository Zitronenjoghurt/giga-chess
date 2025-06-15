use giga_chess::prelude::*;
use rand::prelude::IteratorRandom;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn run_stupid_game(engine: &Arc<Engine>, delay_ms: u64) -> impl Iterator<Item = String> + '_ {
    let meta = PGNMetadata::now()
        .event("Stupid Game")
        .white("Stupid White")
        .black("Stupid Black");
    let mut game = Game::new(engine, meta);

    std::iter::from_fn(move || {
        let moves = game.legal_moves();
        if moves.is_empty() {
            println!(
                "Winner: {:?}, Status: {:?}\n{}\n{}",
                game.winner(),
                game.status(),
                game.get_fen_string(),
                game.get_pgn()
            );
            return None;
        }

        let possible_moves_algebraic = game
            .legal_moves_algebraic(engine)
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        let chosen = *moves.iter().choose(&mut rand::rng())?;
        game.play_move(engine, chosen);

        thread::sleep(Duration::from_millis(delay_ms));

        Some(format!(
            "# {} | {:?}: {}\n{}{:?}\n",
            game.full_moves(),
            game.side_to_move().opposite(),
            game.latest_move_algebraic().unwrap(),
            game.board().to_string(),
            possible_moves_algebraic,
        ))
    })
}
