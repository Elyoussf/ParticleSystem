use super::types::{Direction, Position, ShareQueue};

pub struct Snake {
    pub head: Position,
    pub tail: Position,
    pub length: usize,
    pub instance: String,
}

impl Snake {
    pub fn spawn(head: Position, tail: Position, length: i32) -> Snake {
        // the height and width here are the the allowed to achieve not the original height and width of the matrix
        // That means if the the head goes beyond these boundaries it will fail/respawn from the other side
        // 1 ----> height allowed  vertically
        // 1 ---> width allowed horizontally
        // we assume in the code the matrix is enoughly huge
        Snake {
            head,
            tail,
            length: length as usize,
            instance: "=".to_string(),
        }
    }
    pub fn move_snake(breaks: ShareQueue) {
        let breaks_copy = breaks.lock().unwrap(); // To avoid mutex poisning
    }
    pub fn bait_1v1_check(self: &mut Snake, bait: Position) {
        if self.head.x == bait.x && self.head.y == bait.y {
            self.length += 1;
        }
    }
}
