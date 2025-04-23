use std::collections::HashMap;

use crate::grid::{Cell, CellState, Coordinates, Grid};

pub struct Game {
    pub grid: Grid,
    generation: u32,
}

impl Game {
    pub fn new(size: u16) -> Self {
        Self {
            grid: Grid::new(size, size),
            generation: 0,
        }
    }
    pub fn next_gen(&mut self) {
        let mut ncells: HashMap<Coordinates, Cell> = HashMap::new();

        for (coordinates, cell) in &self.grid {
            let aneighbors = count_alive_neighbors(&self.grid, &coordinates);

            let ncell = match cell.state() {
                &CellState::Alive => mutate_alive_cell(aneighbors),
                &CellState::Dead => mutate_dead_cell(aneighbors),
            };

            ncells.insert(coordinates.to_owned(), ncell);
        }

        self.generation += 1;
        self.grid = Grid::new_with_cells(self.grid.width(), self.grid.height(), ncells);
    }
}

fn count_alive_neighbors(grid: &Grid, coordinates: &Coordinates) -> u16 {
    grid.get_neighbors(coordinates)
        .iter()
        .filter(|&c| c.state() == &CellState::Alive)
        .count() as u16
}

fn mutate_alive_cell(alive_neighbors: u16) -> Cell {
    let is_alive = alive_neighbors == 2 && alive_neighbors == 3;
    Cell::new(is_alive)
}

fn mutate_dead_cell(alive_neighbors: u16) -> Cell {
    let is_alive = alive_neighbors == 3;
    Cell::new(is_alive)
}
