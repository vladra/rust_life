use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cell {
    state: CellState,
}

impl Cell {
    pub fn new(is_alive: bool) -> Self {
        Self {
            state: if is_alive {
                CellState::Alive
            } else {
                CellState::Dead
            },
        }
    }

    pub fn state(&self) -> &CellState {
        &self.state
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Coordinates {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
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
                let cell = Cell::new(get_rand_bool());

                cells.insert(coordinates, cell);
            }
        }

        Self::new_with_cells(width, height, cells)
    }

    pub fn new_with_cells(width: u16, height: u16, cells: HashMap<Coordinates, Cell>) -> Self {
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn get(&self, x: u16, y: u16) -> &Cell {
        let coordinates = Coordinates { x, y };

        self.cells
            .get(&coordinates)
            .expect("Cell for coordinates {x}, {y} is not found")
    }

    pub fn get_neighbors(&self, coordinates: &Coordinates) -> Vec<&Cell> {
        let cx = coordinates.x as i32;
        let cy = coordinates.y as i32;
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

impl<'a> IntoIterator for &'a Grid {
    type Item = (&'a Coordinates, &'a Cell);
    type IntoIter = std::collections::hash_map::Iter<'a, Coordinates, Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter()
    }
}

fn wrap_coords(coord: i32, max: u16) -> u16 {
    let max = max as i32;
    (((coord % max) + max) % max) as u16
}

fn get_rand_bool() -> bool {
    fastrand::choice([true, false, false, false]).unwrap()
}
