mod grid;

fn main() {
    let grid = grid::Grid::new(20, 10);
    println!("{grid}");

    let corner_top_cell = grid.get(1, 1);
    println!("{corner_top_cell:?}");
    let mid_cell = grid.get(2, 3);
    println!("{mid_cell:?}");
    let corner_bottom_cell = grid.get(20, 10);
    println!("{corner_bottom_cell:?}");

    let neighboors = grid.get_neighbors(mid_cell);
    println!("{neighboors:?}");
}
