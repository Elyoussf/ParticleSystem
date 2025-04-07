use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub struct Break_effect {
    // pub position: Position,
    pub goto: Direction,
    pub current_index: usize, // This index will  come from the field instances in snake struct  that identifies the partcile in sanke body that should be applied to
}

#[derive(Copy, Clone)]
pub struct Data {
    pub direction: Direction,
    pub position: Position,
}
pub type ShareVec = Arc<Mutex<Vec<Break_effect>>>;
// This will store a queue of position where to turn in a conncurrent way so that many would
