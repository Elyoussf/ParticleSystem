use super::types::Position;
use rand::Rng;

#[derive(Clone)]
pub struct Particle {
    pub symbol: String,
    pub pos: Position,
    pub height: i32,
    pub width: i32,

    pub lifetime: i32,
}

impl Particle {
    pub fn spawn(height: i32, width: i32) -> Particle {
        let mut rng = rand::thread_rng();

        Particle {
            symbol: "1".to_string(),
            pos: Position {
                x: rng.gen_range(1..=height - 2) as usize,
                y: rng.gen_range(1..=width - 2) as usize,
            },

            lifetime: rng.gen_range(2..=100),
            height,
            width,
        }
    }

    pub fn age(&mut self) {
        self.lifetime -= 1;
    }

    pub fn change_position_in_liquid(&mut self) {
        let current_i = self.pos.x;
        let current_j = self.pos.y;

        let mut neighbors: Vec<Position> = Vec::new();

        if current_i > 1 {
            neighbors.push(Position {
                x: current_i - 1,
                y: current_j,
            });
        }
        if current_i < (self.height - 1) as usize {
            neighbors.push(Position {
                x: current_i + 1,
                y: current_j,
            });
        }
        if current_j < (self.width - 1) as usize {
            neighbors.push(Position {
                x: current_i,
                y: current_j + 1,
            });
        }
        if current_j > 1 {
            neighbors.push(Position {
                x: current_i,
                y: current_j - 1,
            });
        }

        // Add diagonal neighbors
        if current_i > 1 && current_j > 1 {
            neighbors.push(Position {
                x: current_i - 1,
                y: current_j - 1,
            });
        }
        if current_i > 1 && current_j < (self.width - 1) as usize {
            neighbors.push(Position {
                x: current_i - 1,
                y: current_j + 1,
            });
        }
        if current_i < (self.height - 1) as usize && current_j > 1 {
            neighbors.push(Position {
                x: current_i + 1,
                y: current_j - 1,
            });
        }
        if current_i < (self.height - 1) as usize && current_j < (self.width - 1) as usize {
            neighbors.push(Position {
                x: current_i + 1,
                y: current_j + 1,
            });
        }

        let n = neighbors.len();
        let mut rng = rand::thread_rng();
        let next_position_index = rng.gen_range(0..n);
        self.pos = neighbors[next_position_index];
    }
}
