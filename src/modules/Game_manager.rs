use super::Container::Container;
use super::Particle::Particle;

use std::thread;
use std::time::{Duration, Instant};


pub fn Game_manager(){
    let mut board = Container::new();
    let mut Alive_particles: Vec<Particle> = Vec::new();
    let game_period = Duration::from_secs(100);
    let start = Instant::now();
    while Instant::now() - start <= game_period {
        let mut new_particle = Particle::spawn(board.matrix.len() as i32, board.matrix[0].len() as i32);
        board.matrix[new_particle.pos.x as usize][new_particle.pos.y as usize] = new_particle.symbol.clone();
        
        let _ = Alive_particles.iter_mut().map(|e| {
            if e.lifetime == 0{
                board.matrix[new_particle.pos.x as usize][new_particle.pos.y as usize] = " ".to_string();
            }else{
                e.get_aged();
                board.matrix[new_particle.pos.x as usize][new_particle.pos.y as usize] = " ".to_string();
                e.change_position();
                board.matrix[new_particle.pos.x as usize][new_particle.pos.y as usize] = new_particle.symbol.clone();
            }

        });
        Alive_particles.push(new_particle);
        
        // We keep Just the Particles  that are still alive   
        Alive_particles.retain(|e| e.lifetime!=0);
        thread::sleep(Duration::from_millis(100));
        std::process::Command::new("clear").status().unwrap();
        board.render();
        println!("Game period : {:?}",game_period);
        println!("{:?} Seconds passed", Instant::now() - start);
    }
}



