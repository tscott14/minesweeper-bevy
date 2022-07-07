use bevy::prelude::*;

#[derive(Component)]
pub struct Coordinates {
    pub x_coord: isize,
    pub y_coord: isize,
}

impl Coordinates {
    pub fn new(x: isize, y: isize) -> Self {
        Self {
            x_coord: x,
            y_coord: y,
        }
    }
}