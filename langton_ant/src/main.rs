use std::time::Duration;

use minifb::{Key, Window, WindowOptions};
use rand::Rng;
const GRID_SIZE: usize = 101; // Grid dimensions (101x101)
const CELL_SIZE: usize = 5;  // Pixel size of each cell
const WIDTH: usize = GRID_SIZE * CELL_SIZE; // Window width
const HEIGHT: usize = GRID_SIZE * CELL_SIZE; // Window height

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct LangtonsAnt {
    grid: Vec<Vec<bool>>, // true = white, false = black
    x: usize,
    y: usize,
    direction: Direction,
}

impl LangtonsAnt {
    fn new(grid_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..grid_size);
        let y = rng.gen_range(0..grid_size);
        Self {
            grid: vec![vec![false; grid_size]; grid_size],
            x,
            y,
            direction: Direction::Up,
        }
    }

    fn step(&mut self) {
        // Toggle the current cell color
        self.grid[self.y][self.x] = !self.grid[self.y][self.x];

        // Change direction based on current cell color
        self.direction = match (self.direction, self.grid[self.y][self.x]) {
            (Direction::Up, true) => Direction::Right,
            (Direction::Up, false) => Direction::Left,
            (Direction::Right, true) => Direction::Down,
            (Direction::Right, false) => Direction::Up,
            (Direction::Down, true) => Direction::Left,
            (Direction::Down, false) => Direction::Right,
            (Direction::Left, true) => Direction::Up,
            (Direction::Left, false) => Direction::Down,
        };

        // Move the ant
        match self.direction {
            Direction::Up => {
                if self.y > 0 {
                    self.y -= 1;
                }
            }
            Direction::Right => {
                if self.x < self.grid[0].len() - 1 {
                    self.x += 1;
                }
            }
            Direction::Down => {
                if self.y < self.grid.len() - 1 {
                    self.y += 1;
                }
            }
            Direction::Left => {
                if self.x > 0 {
                    self.x -= 1;
                }
            }
        }
    }
}
fn draw_grid(buffer: &mut [u32], ants: &[LangtonsAnt], zoom_level: usize) {
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let mut color = 0x000000;
            for ant in ants {
                if ant.grid[y][x] {
                    color = 0xFFFFFF;
                    break;
                }
            }
            for cy in 0..zoom_level {
                for cx in 0..zoom_level {
                    let buffer_x = x * zoom_level + cx;
                    let buffer_y = y * zoom_level + cy;
                    if buffer_x < WIDTH && buffer_y < HEIGHT {
                        buffer[buffer_y * WIDTH + buffer_x] = color;
                    }
                }
            }
        }
    }
}

fn initialize_ants(num_ants: usize, grid_size: usize) -> Vec<LangtonsAnt> {
    (0..num_ants).map(|_| LangtonsAnt::new(grid_size)).collect()
}

fn perform_steps(ants: &mut [LangtonsAnt]) {
    for ant in ants {
        ant.step();
    }
}

fn main() {
    let mut window = Window::new(
        "Langton's Ant - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Failed to create window: {}", e);
    });

    window.limit_update_rate(Some(Duration::from_micros(16600)));

    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut ants = initialize_ants(10, GRID_SIZE); // Change the number of ants here

    let mut zoom_level = 1;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Equal) {
            zoom_level += 1;
        }
        if window.is_key_down(Key::Minus) {
            if zoom_level > 1 {
                zoom_level -= 1;
            }
        }

        perform_steps(&mut ants);
        draw_grid(&mut buffer, &ants, zoom_level);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}