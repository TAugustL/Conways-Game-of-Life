use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::WindowCanvas,
    video::Window,
};

pub const WINDOW_SIZE: (u32, u32) = (1200, 800);
pub const GRID_SIZE: u32 = 20;

pub struct Renderer {
    canvas: WindowCanvas,
    pub grid_state: Vec<Vec<bool>>,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Self, Box<dyn std::error::Error>> {
        let canvas = window.into_canvas().build()?;
        let grid_state = vec![
            vec![false; (WINDOW_SIZE.0 / GRID_SIZE) as usize];
            (WINDOW_SIZE.1 / GRID_SIZE) as usize
        ];

        Ok(Renderer { canvas, grid_state })
    }

    pub fn draw(&mut self, show_grid: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut grid: Vec<Rect> = Vec::new();

        for y in 0..WINDOW_SIZE.1 / GRID_SIZE {
            for x in 0..WINDOW_SIZE.0 / GRID_SIZE {
                if self.grid_state[y as usize][x as usize] {
                    grid.push(Rect::new(
                        (x * GRID_SIZE) as i32,
                        (y * GRID_SIZE) as i32,
                        GRID_SIZE,
                        GRID_SIZE,
                    ));
                }
            }
        }

        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.fill_rects(&grid)?;
        if show_grid {
            self.draw_grid()?;
        }
        self.canvas.present();
        Ok(())
    }

    fn draw_grid(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.canvas.set_draw_color(Color::GREY);
        for x in 0..WINDOW_SIZE.0 / GRID_SIZE {
            self.canvas.draw_line(
                Point::new((x * GRID_SIZE) as i32, 0),
                Point::new((x * GRID_SIZE) as i32, WINDOW_SIZE.1 as i32),
            )?;
        }
        for y in 0..WINDOW_SIZE.1 / GRID_SIZE {
            self.canvas.draw_line(
                Point::new(0, (y * GRID_SIZE) as i32),
                Point::new(WINDOW_SIZE.0 as i32, (y * GRID_SIZE) as i32),
            )?;
        }

        Ok(())
    }
}

pub fn step_forward(grid: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_grid: Vec<Vec<bool>> = vec![
        vec![false; (WINDOW_SIZE.0 / GRID_SIZE) as usize];
        (WINDOW_SIZE.1 / GRID_SIZE) as usize
    ];

    let x_size = WINDOW_SIZE.0 / GRID_SIZE;
    let y_size = WINDOW_SIZE.1 / GRID_SIZE;

    for y in 0..y_size {
        for x in 0..x_size {
            let mut alive_neighbors: u8 = 0;

            let (x, y) = (x as i32, y as i32);

            // get alive neighbor count
            if grid[y as usize][(x - 1).rem_euclid(x_size as i32) as usize] {
                alive_neighbors += 1;
            }
            if grid[y as usize][(x + 1).rem_euclid(x_size as i32) as usize] {
                alive_neighbors += 1;
            }
            for xi in 0..3 {
                if grid[(y - 1).rem_euclid(y_size as i32) as usize]
                    [(x - 1 + xi).rem_euclid(x_size as i32) as usize]
                {
                    alive_neighbors += 1;
                }
            }
            for xi in 0..3 {
                if grid[(y + 1).rem_euclid(y_size as i32) as usize]
                    [(x - 1 + xi).rem_euclid(x_size as i32) as usize]
                {
                    alive_neighbors += 1;
                }
            }

            if !(2..=3).contains(&alive_neighbors) {
                new_grid[y as usize][x as usize] = false;
            }
            if (2..=3).contains(&alive_neighbors) && grid[y as usize][x as usize] {
                new_grid[y as usize][x as usize] = true;
            }
            if alive_neighbors == 3 && !grid[y as usize][x as usize] {
                new_grid[y as usize][x as usize] = true;
            }
        }
    }

    new_grid
}
