use crate::simulations::stupid_game::run_stupid_game;
use giga_chess::engine::Engine;

mod simulations;

fn main() {
    let engine = Engine::initialize();
    for board in run_stupid_game(&engine, 2000) {
        println!("{}", board);
    }
}
