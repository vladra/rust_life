use game::Game;

mod game;
mod grid;

fn main() {
    let mut game = Game::new(20);
    println!("{}", game.grid);

    for i in 1..=10 {
        game.next_gen();
        println!("{}", game.grid);
    }
}
