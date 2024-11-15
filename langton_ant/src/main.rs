use minifb::{Key, Window, WindowOptions};

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
        LangtonsAnt {
            grid: vec![vec![false; grid_size]; grid_size],
            x: grid_size / 2,
            y: grid_size / 2,
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

    // Limit the update rate to ~60 FPS
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut ant = LangtonsAnt::new(GRID_SIZE);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Perform one step of the simulation
        ant.step();

        // Draw the grid
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let color = if ant.grid[y][x] { 0xFFFFFF } else { 0x000000 };
                for cy in 0..CELL_SIZE {
                    for cx in 0..CELL_SIZE {
                        let px = x * CELL_SIZE + cx;
                        let py = y * CELL_SIZE + cy;
                        buffer[py * WIDTH + px] = color;
                    }
                }
            }
        }

        // Draw the ant
        for cy in 0..CELL_SIZE {
            for cx in 0..CELL_SIZE {
                let px = ant.x * CELL_SIZE + cx;
                let py = ant.y * CELL_SIZE + cy;
                buffer[py * WIDTH + px] = 0xFF0000; // Red color for the ant
            }
        }

        // Update the window with the buffer
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
