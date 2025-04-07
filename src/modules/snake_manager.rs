use super::{
    container::{self, Container},
    particle::Particle,
    types::{self, Break_effect, Direction, Position, ShareVec},
};
use crate::utils::{
    self,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{
    clone,
    io::{self, Read, Write},
    sync::{Arc, Mutex},
    thread::JoinHandle,
};
use std::{collections::HashMap, thread};
use std::{process::Command, thread::Thread, time::Duration};

#[derive(Clone)]
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

    pub fn update_container(&self, grid: &mut Container) {
        let rows = grid.matrix.len();
        let cols = grid.matrix[0].len();

        for i in 1..rows - 1 {
            for j in 1..cols - 1 {
                grid.matrix[i][j] = " ".to_string();
            }
        }
        self.instances.iter().for_each(|e| {
            grid.matrix[e.pos.x as usize][e.pos.y as usize] = "=".to_string();
        });
    }

    // Modified to not return a JoinHandle, directly processes the breaks
    pub fn break_exist(&mut self, breaks: &mut Vec<Break_effect>) {
        breaks.iter_mut().for_each(|brk| {
            let index = brk.current_index;

            if let Some(ins) = self.instances.get_mut(index) {
                match brk.goto {
                    Direction::Up => {
                        if ins.pos.x > 1 {
                            ins.pos.x -= 1;
                        }
                    }
                    Direction::Down => {
                        if ins.pos.x < ins.height as usize {
                            // ====> width and height are allowed to include (real -1)
                            ins.pos.x += 1;
                        }
                    }
                    Direction::Right => {
                        if ins.pos.y < ins.width as usize {
                            ins.pos.y += 1;
                        }
                    }
                    Direction::Left => {
                        if ins.pos.y > 1 {
                            ins.pos.y -= 1;
                        }
                    }
                }

                brk.current_index += 1;
            } else {
                println!("The index of particle that break expects is not quite correct!!!");
            }
        });
    }

    // Modified to take a reference to Snake and return a JoinHandle
    pub fn listen_keys_get_directions(self, breaks: ShareVec) -> JoinHandle<()> {
        thread::spawn(move || {
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
                            let break_effect = Break_effect {
                                goto: Direction::Up,
                                current_index: 0, // head of the snake
                            };

                            breaks.lock().unwrap().push(break_effect);
                        }
                        (91, 66) => {
                            let break_effect = Break_effect {
                                goto: Direction::Down,
                                current_index: 0, // head of the snake
                            };
                            breaks.lock().unwrap().push(break_effect);
                        }
                        (91, 67) => {
                            let break_effect = Break_effect {
                                goto: Direction::Right,
                                current_index: 0, // head of the snake
                            };
                            breaks.lock().unwrap().push(break_effect);
                        }
                        (91, 68) => {
                            let break_effect = Break_effect {
                                goto: Direction::Left,
                                current_index: 0, // head of the snake
                            };
                            breaks.lock().unwrap().push(break_effect);
                        }
                        _ => println!("Unknown escape: {} {} {}", b, next1, next2),
                    }
                } else {
                    println!("Pressed: {}", b as char);
                }
            }
        })
    }
}
