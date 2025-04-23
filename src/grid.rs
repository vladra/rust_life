use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum CellState {
    Alive,
    Dead,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cell {
    coordinates: Coordinates,
    state: CellState,
}

impl Cell {
    pub fn new(coordinates: Coordinates, is_alive: bool) -> Self {
        Self {
            coordinates,
            state: if is_alive {
                CellState::Alive
            } else {
                CellState::Dead
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug)]
pub struct Grid {
    width: u16,
    height: u16,
    cells: HashMap<Coordinates, Cell>,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get(x, y);

                match cell.state {
                    CellState::Alive => write!(f, "#")?,
                    CellState::Dead => write!(f, "_")?,
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        let mut cells = HashMap::new();

        for y in 0..height {
            for x in 0..width {
                let coordinates = Coordinates { x, y };
                let cell = Cell::new(coordinates.clone(), get_rand_bool());

                cells.insert(coordinates, cell);
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

        self.cells
            .get(&coordinates)
            .expect("Cell for coordinates {x}, {y} is not found")
    }

    pub fn get_neighbors(&self, cell: &Cell) -> Vec<&Cell> {
        let cx = cell.coordinates.x as i32;
        let cy = cell.coordinates.y as i32;
        let width = self.width;
        let height = self.height;

        let mut neighbors = Vec::with_capacity(8);

        // Check bounds of the world
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = wrap_coords(cx + dx, width);
                let ny = wrap_coords(cy + dy, height);

                neighbors.push(self.get(nx, ny))
            }
        }

        neighbors
    }
}

fn wrap_coords(coord: i32, max: u16) -> u16 {
    let max = max as i32;
    (((coord % max) + max) % max) as u16
}

fn get_rand_bool() -> bool {
    fastrand::choice([true, false, false, false]).unwrap()
}
