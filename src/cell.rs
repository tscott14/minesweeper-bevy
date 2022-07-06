#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CellType {
    EMPTY(Option<u8>),
    BOMB,
}

#[derive(Clone, Copy, PartialEq, Debug )]
pub enum CellState {
    UNKNOWN,
    FLAGGED,
    EXPOSED,
}