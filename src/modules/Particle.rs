use rand::{thread_rng, Rng};

struct Position{
    x : i32,
    y : i32
}


pub struct Particle{
    symbol : String,
    pos : Position,
    Velocity : i32,

    matrix : &mut Vec<Vec<String>>
}

impl Particle{
    pub fn spawn(height:i32,width : i32,grid : &mut Vec<Vec<String>>) -> Particle{
        let  mut rng = rand::rng();
        
        Particle { symbol:"@".to_string(), pos: Position { x: rng.random_range(0..=height-1), y: rng.random_range(0..=width-1) }, Velocity: 0,matrix:grid }
    }

    pub fn move_me(&self){

    }
    
}