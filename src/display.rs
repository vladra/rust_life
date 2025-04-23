use crate::grid::{CellState, Grid};

pub fn render(grid: &Grid) {
    let width = grid.width();
    let height = grid.height();

    print!("┌");
    for _ in 0..height {
        print!("─");
    }
    println!("┐");

    for x in 0..width {
        print!("│");
        for y in 0..height {
            let cell = grid.get(x, y);

            if cell.state() == &CellState::Alive {
                print!("█");
            } else {
                print!(".");
            }
        }
        println!("│");
    }

    print!("└");
    for _ in 0..height {
        print!("─");
    }
    println!("┘");
}

pub fn clear_terminal() {
    print!("{}[2J", 27 as char);
}
