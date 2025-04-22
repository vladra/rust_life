mod grid;

fn main() {
    let grid = grid::Grid::new(20, 10);
    println!("{grid}");

    let corner_top_cell = grid.get(0, 0);
    println!("{corner_top_cell:?}");
    let mid_cell = grid.get(2, 3);
    println!("{mid_cell:?}");
    let corner_bottom_cell = grid.get(19, 9);
    println!("{corner_bottom_cell:?}");

    let neighboors = grid.get_neighbors(mid_cell);
    println!("{neighboors:?}");
}
