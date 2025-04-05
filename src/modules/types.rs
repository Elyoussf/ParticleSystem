use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub type ShareQueue = Arc<Mutex<VecDeque<(Position, Direction)>>>;
// This will store a queue of position where to turn in a conncurrent way so that many would
