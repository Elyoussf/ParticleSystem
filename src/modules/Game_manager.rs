// use rand::seq::IndexedRandom;

// use super::Container::Container;
// use super::Particle::Particle;

// use std::thread;
// use std::time::{Duration, Instant};


// pub fn Game_manager(){
//     let mut board = Container::new();
//     let mut Alive_particles: Vec<Particle> = Vec::new();
//     let game_period = Duration::from_secs(100);
//     let start = Instant::now();
//     while Instant::now() - start <= game_period {
//         let mut new_particle = Particle::spawn(board.matrix.len() as i32, board.matrix[0].len() as i32);
//         board.matrix[new_particle.pos.x as usize][new_particle.pos.y as usize] = new_particle.symbol.clone();
        
//         let _ = Alive_particles.iter_mut().map(|e| {
//             if e.lifetime == 0{
//                 board.matrix[e.pos.x as usize][e.pos.y as usize] = " ".to_string();
//                 println!("Dead!!");
//             }else{
//                 e.lifetime-=1;
//                 board.matrix[e.pos.x as usize][e.pos.y as usize] = " ".to_string();
//                 e.change_position();
//                 board.matrix[e.pos.x as usize][e.pos.y as usize] = e.symbol.clone();
//             }

//         });
//         if Alive_particles.is_empty(){
//             Alive_particles.push(new_particle);
//         }
        
        
//         // We keep Just the Particles  that are still alive   
//         Alive_particles.retain(|e| e.lifetime!=0);
//         println!("{} Alive",Alive_particles.len());
//         thread::sleep(Duration::from_millis(500));
//         std::process::Command::new("clear").status().unwrap();
//         board.render();
//         println!("Game period : {:?}",game_period);
//         println!("{:?} Seconds passed", Instant::now() - start);
        
//     }
// }



use super::Container::Container;
use super::Particle::Particle;
use std::thread;
use std::time::{Duration, Instant};

pub fn game_manager() {
    let mut board = Container::new();
    let mut alive_particles: Vec<Particle> = Vec::new();
    let game_period = Duration::from_secs(30);
    let start = Instant::now();

    while Instant::now() - start <= game_period {
        // Spawn new particle
        let new_particle = Particle::spawn(
            board.matrix.len() as i32-1,
            board.matrix[0].len() as i32-1
        );
        

        // First clear all current particle positions
        for particle in alive_particles.iter() {
            board.matrix[particle.pos.x as usize][particle.pos.y as usize] = " ".to_string();
        }

        // Update existing particles
        for particle in alive_particles.iter_mut() {
            
                particle.age();
                board.matrix[particle.pos.x as usize][particle.pos.y as usize] = " ".to_string();
                particle.change_position();

                // Only update board for living particles
                if particle.lifetime > 0{
                    board.matrix[particle.pos.x as usize][particle.pos.y as usize] = particle.symbol.clone();
                }
        }

        // Add new particle if none exist
        
        alive_particles.push(new_particle);
        
        
        alive_particles.retain(|particle| particle.lifetime != 0);
        
        // Logs and timer
        println!("{} Alive", alive_particles.len());
        thread::sleep(Duration::from_millis(100));
        std::process::Command::new("clear").status().unwrap();
        board.render();
        println!("Demo period: {:?}", game_period);
        println!("Time passed: {:?}", Instant::now() - start);
    }
}