use crate::{
    cell::{CellState, CellType},
    minesweeper::Cordinates,
};

pub struct Dimension {
    pub width: usize,
    pub height: usize,
}

pub struct Field {
    pub dimensions: Dimension,
    pub cell_size: usize,
    pub bomb_count: usize,
    pub cells: Vec<(CellType, CellState)>,
}

impl Field {
    pub fn new(width: usize, height: usize, cell_size: usize, bomb_count: usize) -> Self {
        let dimensions = Dimension { width, height };
        let cells = vec![(CellType::EMPTY(None), CellState::UNKNOWN); width * height];
        let mut field = Field {
            dimensions,
            cell_size,
            bomb_count,
            cells,
        };
        field.populate_bombs(bomb_count);
        field
    }

    fn populate_bombs(&mut self, count: usize) {
        let size = &self.dimensions;
        let cells = &mut self.cells;
        for i in 0..size.width * size.height {
            cells[i] = (CellType::EMPTY(None), CellState::UNKNOWN);
        }
        for _ in 0..count {
            loop {
                let x = rand::random::<usize>() % size.width;
                let y = rand::random::<usize>() % size.height;
                if cells[x + y * size.width].0 != CellType::BOMB {
                    cells[x + y * size.width] = (CellType::BOMB, CellState::UNKNOWN);
                    break;
                }
            }
        }
    }

    fn cordnates_to_index(&self, coords: &Cordinates) -> usize {
        let width = self.dimensions.width as isize;
        (coords.x_coord + coords.y_coord * width) as usize
    }

    pub fn at(&self, coords: &Cordinates) -> Option<(CellType, CellState)> {
        let width = self.dimensions.width as isize;
        let height = self.dimensions.height as isize;
        if coords.x_coord < width
            && coords.x_coord >= 0
            && coords.y_coord < height
            && coords.y_coord >= 0
        {
            let index = self.cordnates_to_index(&coords);
            Some(self.cells[index].clone())
        } else {
            None
        }
    }

    pub fn at_mut(&mut self, coords: &Cordinates) -> Option<&mut (CellType, CellState)> {
        let width = self.dimensions.width as isize;
        let height = self.dimensions.height as isize;
        if coords.x_coord < width
            && coords.x_coord >= 0
            && coords.y_coord < height
            && coords.y_coord >= 0
        {
            let index = self.cordnates_to_index(&coords);
            Some(&mut self.cells[index])
        } else {
            None
        }
    }

    pub fn count_bomb_neighbors(&self, coords: &Cordinates) -> u8 {
        let width = self.dimensions.width as isize;
        let height = self.dimensions.height as isize;
        let cxc = coords.x_coord as isize;
        let cyc = coords.y_coord as isize;

        let mut sum = 0u8;
        for y in -1..2 {
            for x in -1..2 {
                let xc = x + cxc;
                let yc = y + cyc;

                if xc < 0 || xc >= width || yc < 0 || yc >= height {
                    continue;
                }

                let index = self.cordnates_to_index(&Cordinates::new(xc, yc));
                match self.cells[index] {
                    (CellType::BOMB, _) => sum += 1,
                    _ => {}
                }
            }
        }
        sum
    }

    pub fn set_cell_type(&mut self, coords: &Cordinates, cell_type: CellType) {
        if let Some(cell) = self.at_mut(&coords) {
            cell.0 = cell_type;
        }
    }

    pub fn set_cell_state(&mut self, coords: &Cordinates, cell_state: CellState) {
        if let Some(cell) = self.at_mut(&coords) {
            cell.1 = cell_state;
        }
    }

    pub fn reset(&mut self) {
        self.populate_bombs(self.bomb_count);
    }
}
