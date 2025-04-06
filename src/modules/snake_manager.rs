use super::{
    container::Container,
    particle::Particle,
    types::{self, Direction, Position, ShareVec},
};
use crate::utils::{
    self,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::process::Command;
use std::{
    clone,
    io::{self, Read, Write},
};
use std::{collections::HashMap, thread};

pub struct Snake {
    pub head: types::Data,
    pub tail: types::Data,
    pub length: usize,
    pub instances: Vec<Particle>, // This is the vector of the particle that makes the snake  Their index is important for break point identification
}

impl Snake {
    pub fn spawn(
        head: types::Data,
        tail: types::Data,
        length: i32,
        height: i32,
        width: i32,
    ) -> Snake {
        // the height and width here are the the allowed to achieve not the original height and width of the matrix
        // That means if the the head goes beyond these boundaries it will fail/respawn from the other side
        // 1 ----> height allowed  vertically
        // 1 ---> width allowed horizontally
        // we assume in the code the matrix is enoughly huge
        let h = head.position.clone();
        let t = tail.position.clone();
        let mut snk = Snake {
            head,
            tail,
            length: length as usize,
            instances: Vec::new(),
        };

        snk.instances.push(Particle {
            symbol: "=".to_string(),
            pos: h,
            lifetime: 0,
            height,
            width,
        });

        snk.instances.push(Particle {
            symbol: "=".to_string(),
            pos: t,
            lifetime: 0,
            height,
            width,
        });

        snk
    }

    pub fn break_exist(&mut self, breaks: &mut ShareVec) {
        // The caller should make a copy of Arc Then give it to this function
        let mut _breaks = breaks.lock().unwrap(); // To avoid mutex poisning
        _breaks.iter_mut().for_each(|brk| {
            let index = brk.current_index;

            if let Some(ins) = self.instances.get_mut(index) {
                match brk.goto {
                    Direction::Up => {
                        if ins.pos.x > 1 {
                            ins.pos.x -= 1
                        }
                    }
                    Direction::Down => {
                        if ins.pos.y < ins.height {
                            // ====> width and height are allowed to include   (real -1)
                            ins.pos.x += 1
                        }
                    }
                    Direction::Right => {
                        if ins.pos.y < ins.width {
                            ins.pos.y += 1
                        }
                    }
                    Direction::Left => {
                        if ins.pos.x > 1 {
                            ins.pos.x -= 1
                        }
                    }
                }

                brk.current_index += 1
            } else {
                println!("The index of particle that break expects is not quite correct!!!");
            }
        });
        _breaks.retain(|e| e.current_index < self.length - 1); // A mutable borrow occurs here
    }

    pub fn bait_1v1_check(self: &mut Snake, bait: Position) {
        if self.head.x == bait.x && self.head.y == bait.y {
            self.length += 1;
        }
    }

    pub fn move_accordingly(&mut self , new_dir :  Direction){
        match new_dir{
            Direction::Up{
            self.head.direction = new_dir;

            }
        }
    }

    pub fn listen_keys_get_directions(&mut self, breaks: &mut ShareVec) {
        // Make sure thsis just a copy
        let input_thread = thread::spawn(|| {
            enable_raw_mode();

            let stdin = io::stdin();
            let mut stdout = io::stdout();
            stdout.flush().unwrap();

            // Create a single Bytes iterator, so it isn't moved multiple times
            let mut bytes = stdin.lock().bytes();

            while let Some(Ok(b)) = bytes.next() {
                if b == b'q' {
                    break;
                }

                if b == 27 {
                    let next1 = bytes.next().unwrap().unwrap();
                    let next2 = bytes.next().unwrap().unwrap();

                    match (next1, next2) {
                        (91, 65) => {
                            // new break point is created  if current direction of the head is not like the direction told here!!
                            let requested_direction = Direction::Up;

                            self.head.direction = requested_direction;
                        }
                        (91, 66) => println!("Down"),
                        (91, 67) => println!("Right"),
                        (91, 68) => println!("Left"),
                        _ => println!("Unknown escape: {} {} {}", b, next1, next2),
                    }
                } else {
                    println!("Pressed: {}", b as char);
                }
            }
            disable_raw_mode();
        });
    }
}
