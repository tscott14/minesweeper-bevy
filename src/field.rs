use crate::{
    cell::{CellState, CellType},
    coordinates::Coordinates,
    dimensions::Dimensions,
};

pub struct Field {
    pub dimensions: Dimensions,
    pub cell_size: usize,
    pub bomb_count: usize,
    pub cells: Vec<(CellType, CellState)>,
}

impl Field {
    pub fn new(width: usize, height: usize, cell_size: usize, bomb_count: usize) -> Self {
        let dimensions = Dimensions::new(width, height);
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
        for i in 0..size.get_width() * size.get_height() {
            cells[i] = (CellType::EMPTY(None), CellState::UNKNOWN);
        }
        for _ in 0..count {
            loop {
                let x = rand::random::<usize>() % size.get_width();
                let y = rand::random::<usize>() % size.get_height();
                if cells[x + y * size.get_width()].0 != CellType::BOMB {
                    cells[x + y * size.get_width()] = (CellType::BOMB, CellState::UNKNOWN);
                    break;
                }
            }
        }
    }

    fn coordinates_to_index(&self, coords: &Coordinates) -> usize {
        let width = self.dimensions.get_width() as isize;
        (coords.x_coord + coords.y_coord * width) as usize
    }

    pub fn get_clone(&self, coords: &Coordinates) -> Option<(CellType, CellState)> {
        let width = self.dimensions.get_width() as isize;
        let height = self.dimensions.get_height() as isize;
        if coords.x_coord < width
            && coords.x_coord >= 0
            && coords.y_coord < height
            && coords.y_coord >= 0
        {
            let index = self.coordinates_to_index(&coords);
            Some(self.cells[index].clone())
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, coords: &Coordinates) -> Option<&mut (CellType, CellState)> {
        let width = self.dimensions.get_width() as isize;
        let height = self.dimensions.get_height() as isize;
        if coords.x_coord < width
            && coords.x_coord >= 0
            && coords.y_coord < height
            && coords.y_coord >= 0
        {
            let index = self.coordinates_to_index(&coords);
            Some(&mut self.cells[index])
        } else {
            None
        }
    }

    pub fn count_bomb_neighbors(&self, coords: &Coordinates) -> u8 {
        let width = self.dimensions.get_width() as isize;
        let height = self.dimensions.get_height() as isize;
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

                let index = self.coordinates_to_index(&Coordinates::new(xc, yc));
                match self.cells[index] {
                    (CellType::BOMB, _) => sum += 1,
                    _ => {}
                }
            }
        }
        sum
    }

    pub fn set_cell_type(&mut self, coords: &Coordinates, cell_type: CellType) {
        if let Some(cell) = self.get_mut(&coords) {
            cell.0 = cell_type;
        }
    }

    pub fn set_cell_state(&mut self, coords: &Coordinates, cell_state: CellState) {
        if let Some(cell) = self.get_mut(&coords) {
            cell.1 = cell_state;
        }
    }

    pub fn reset(&mut self) {
        self.populate_bombs(self.bomb_count);
    }
}
