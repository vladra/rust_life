use std::{thread::sleep, time::Duration};

use game::Game;

mod display;
mod game;
mod grid;

fn main() {
    let mut long_world_found = false;
    let mut i = 0;
    let mut game = Game::new(40);

    while !long_world_found {
        i += 1;
        game.reset();
        println!("Starting {} attempt!", i);
        // sleep(Duration::new(1, 0));

        while game.is_alive {
            if game.generation > 10 {
                long_world_found = true;
                display::clear_terminal();
                display::render(&game);
                sleep(Duration::new(0, 400));
            }

            game.next_gen();
        }

        println!(
            "Game over! Your world leaved for {} generations.",
            game.generation
        );
        // sleep(Duration::new(1, 0));
    }
}
