use std::{thread::sleep, time::Duration};

use game::Game;

mod display;
mod game;
mod grid;

fn main() {
    let mut game = Game::new(60);

    while game.is_alive {
        display::clear_terminal();
        display::render(&game.grid);

        game.next_gen();

        sleep(Duration::new(1, 100));
    }

    println!(
        "Game over! Your world leaved for {} generations.",
        game.generation
    )
}
