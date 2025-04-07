mod modules;
mod utils;
use modules::types::{Break_effect, Data};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
fn main() {
    utils::terminal::enable_raw_mode();
    let mut container = modules::container::Container::new();
    let head = Data {
        direction: modules::types::Direction::Left,
        position: modules::types::Position {
            x: 1,
            y: (container.matrix[0].len() - 3) as usize,
        },
    };
    let tail = Data {
        direction: modules::types::Direction::Left,
        position: modules::types::Position {
            x: 1,
            y: container.matrix[0].len() - 2,
        },
    };
    let length = 2;
    let height = (container.matrix.len() - 2) as i32;
    let width = (container.matrix[0].len() - 2) as i32;

    // Create a shared snake instance using Arc<Mutex<>>
    let snake = Arc::new(Mutex::new(modules::snake_manager::Snake::spawn(
        head, tail, length, height, width,
    )));

    let data: Vec<Break_effect> = Vec::new();
    let breaks = Arc::new(Mutex::new(data));

    // Clone the snake for input thread
    let snake_for_input = snake.clone();
    let breaks1 = breaks.clone();
    let th1 = thread::spawn(move || {
        // Get the snake from the mutex to pass to listen_keys
        let snake_instance = snake_for_input.lock().unwrap().clone();
        snake_instance.listen_keys_get_directions(breaks1.clone())
    })
    .join()
    .unwrap();

    // Main game loop
    let th3 = thread::spawn(move || loop {
        {
            // Lock the snake to apply changes and render
            let mut snake_guard = snake.lock().unwrap();

            // Process any breaks
            {
                let mut breaks_guard = breaks.lock().unwrap();
                if !breaks_guard.is_empty() {
                    snake_guard.break_exist(&mut *breaks_guard);
                }

                // Clean up breaks that have been processed
                breaks_guard.retain(|e| e.current_index <= snake_guard.length - 1);
            }

            // Update and render
            snake_guard.update_container(&mut container);
            container.clear_screen();
            container.render();
        } // snake_guard dropped here

        // Wait before next update
        thread::sleep(Duration::from_millis(500));
    });

    th3.join().unwrap();
    utils::terminal::disable_raw_mode();
}
