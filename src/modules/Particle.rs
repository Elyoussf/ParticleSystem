use rand::{ Rng};
#[derive(Copy, Clone)]
pub struct Position{
   pub  x : i32,
   pub y : i32
}


pub struct Particle{
    pub symbol : String,
    pub pos : Position,
    height : i32,
    width : i32,
    velocity : i32,
    pub lifetime : i32,
    
}

impl Particle{
    pub fn spawn(height:i32,width : i32) -> Particle{
        let  mut rng = rand::rng();
        
        Particle { symbol:"@".to_string(),
        pos: Position { x: rng.random_range(1..=height-2), y: rng.random_range(1..=width-2) },
       
        velocity: 0,
        lifetime : rng.random_range(2..=10),
        height,
        width
    }
    }

    pub fn get_aged(&mut self){
        self.lifetime-=1;
        
    }

    pub fn change_position(&mut self){
        let current_i = self.pos.x;
        let current_j = self.pos.y;

        let mut  neighbors: Vec<Position> = Vec::new();

        if current_i > 0{
            neighbors.push(Position { x: current_i-1, y: current_j });
        }
        if current_i < self.height-1{
            neighbors.push(Position { x: current_i+1, y: current_j });
        }
        if current_j < self.width-1{
            neighbors.push(Position { x: current_i, y: current_j+1 });
        }
        if current_j > 0{
            neighbors.push(Position { x: current_j, y: current_j-1 });
        }
        if current_i > 0 && current_j > 0 {
            neighbors.push(Position { x: current_i-1, y: current_j-1 });
        }
        if current_i > 0 && current_j < self.width-1 {
            neighbors.push(Position { x: current_i-1, y: current_j+1 });
        }
        if current_i < self.height-1 && current_j > 0 {
            neighbors.push(Position { x: current_i+1, y: current_j-1 });
        }
        if current_i< self.height-1 && current_j < self.width{
            neighbors.push(Position { x: current_i-1, y: current_j+1 });
        }

        let n = neighbors.len();
        let  mut rng = rand::rng();
        let next_position_index = rng.random_range(0..=n-1);
        self.pos = neighbors[next_position_index];
        
    }    
}