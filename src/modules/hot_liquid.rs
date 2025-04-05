use super::container::Container;
use super::particle::Particle;
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
            board.matrix.len() as i32 - 1,
            board.matrix[0].len() as i32 - 1,
        );

        // First clear all current particle positions
        for particle in alive_particles.iter() {
            board.matrix[particle.pos.x as usize][particle.pos.y as usize] = " ".to_string();
        }

        // Update existing particles
        for particle in alive_particles.iter_mut() {
            particle.age();
            board.matrix[particle.pos.x as usize][particle.pos.y as usize] = " ".to_string();
            particle.change_position_in_liquid();

            // Only update board for living particles
            if particle.lifetime > 0 {
                board.matrix[particle.pos.x as usize][particle.pos.y as usize] =
                    particle.symbol.clone();
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
