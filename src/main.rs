use std::{thread::sleep, time::Duration};

use game::Game;

mod display;
mod game;
mod grid;

fn main() {
    let mut game = Game::new(20);

    while game.is_alive {
        if game.generation > 0 {}

        display::clear_terminal();
        display::render(&game);
        sleep(Duration::new(0, 100));
        game.next_gen();
    }

    println!(
        "Game over! Your world leaved for {} generations.",
        game.generation
    );
}
