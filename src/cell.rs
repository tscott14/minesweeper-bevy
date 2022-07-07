#[derive(Clone, Copy, PartialEq)]
pub enum CellType {
    EMPTY(Option<u8>),
    BOMB,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    UNKNOWN,
    FLAGGED,
    EXPOSED,
}
