use std::{cmp, collections::HashMap};

#[derive(Debug)]
enum State {
    Alive,
    Dead,
}

#[derive(Debug)]
pub struct Cell {
    coordinates: Coordinates,
    state: State,
}

impl Cell {
    pub fn new(coordinates: Coordinates, is_alive: bool) -> Self {
        match is_alive {
            true => Self {
                coordinates,
                state: State::Alive,
            },
            false => Self {
                coordinates,
                state: State::Dead,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Coordinates {
    x: u16,
    y: u16,
}

#[derive(Debug)]
pub struct Grid {
    width: u16,
    height: u16,
    cells: HashMap<Coordinates, Cell>,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 1..=self.height {
            for x in 1..=self.width {
                let cell = self.get(x, y);

                let _ = match cell.state {
                    State::Alive => write!(f, "#"),
                    State::Dead => write!(f, "_"),
                };
            }
            let _ = writeln!(f);
        }
        Ok(())
    }
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        let mut cells = HashMap::new();

        for y in 1..=height {
            for x in 1..=width {
                let coordinates = Coordinates { x, y };
                let cell = Cell::new(coordinates.clone(), get_rand_bool());

                cells.insert(coordinates, cell);
                println!("Grid::new creating cell {x}, {y}");
            }
        }

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, x: u16, y: u16) -> &Cell {
        let coordinates = Coordinates { x, y };
        match self.cells.get(&coordinates) {
            Some(cell) => cell,
            None => panic!("Cell for coordinates {x}, {y} is not found"),
        }
    }

    pub fn get_neighbors(&self, cell: &Cell) -> Vec<&Cell> {
        let cx = cell.coordinates.x;
        let cy = cell.coordinates.y;

        let mut neighbors = Vec::with_capacity(8);

        // Check bounds of the world
        let from_x = cmp::max(cx - 1, 1);
        let from_y = cmp::max(cy - 1, 1);
        let to_x = cmp::min(cx + 1, self.width);
        let to_y = cmp::min(cy + 1, self.height);

        for x in from_x..=to_x {
            for y in from_y..=to_y {
                // skip self
                if x == cx && y == cy {
                    continue;
                }

                let cell = self.get(x, y);
                neighbors.push(cell);
            }
        }

        neighbors
    }
}

fn get_rand_bool() -> bool {
    fastrand::choice([true, false, false, false]).unwrap()
}
