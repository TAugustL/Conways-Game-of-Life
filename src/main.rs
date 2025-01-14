use std::io::Write;

use conway::{Renderer, GRID_SIZE, WINDOW_SIZE};
use sdl2::{self, event::Event, keyboard::Keycode, mouse::MouseButton};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Controls:");
    println!("Mouse Left  => create cell");
    println!("Mouse Right => delete cell");
    println!("Arrow Right => next step");
    println!("Arrow Left  => previous step");
    println!("G           => toggle grid");
    println!("Space       => toggle autoplay");
    println!("Enter       => reset");
    println!("Escape      => exit");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Conway's Game of Life", WINDOW_SIZE.0, WINDOW_SIZE.1)
        .position_centered()
        .vulkan()
        .build()?;

    let mut renderer = Renderer::new(window)?;

    let refresh_rate: u32 = 60;

    let mut event_pump = sdl_context.event_pump()?;

    let mut grid: Vec<Vec<bool>> = vec![
        vec![false; (WINDOW_SIZE.0 / GRID_SIZE) as usize];
        (WINDOW_SIZE.1 / GRID_SIZE) as usize
    ];

    // use this for going back
    let mut old_grids: Vec<Vec<Vec<bool>>> = Vec::new();

    let mut show_grid: bool = true;
    let mut autoplay: bool = false;
    let mut drawing: bool = false;
    let mut erasing: bool = false;

    let mut generation: u32 = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::G),
                    ..
                } => {
                    show_grid = !show_grid;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::SPACE),
                    ..
                } => {
                    autoplay = !autoplay;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::RETURN),
                    ..
                } => {
                    grid = vec![
                        vec![false; (WINDOW_SIZE.0 / GRID_SIZE) as usize];
                        (WINDOW_SIZE.1 / GRID_SIZE) as usize
                    ];
                    renderer.grid_state = grid.clone();
                    old_grids.clear();
                    generation = 0;
                    print!("Generation 0       \r");
                    std::io::stdout().flush()?;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    drawing = true;
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    drawing = false;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Right,
                    ..
                } => {
                    erasing = true;
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Right,
                    ..
                } => {
                    erasing = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::RIGHT),
                    ..
                } => {
                    old_grids.push(grid.clone());
                    grid = conway::step_forward(grid);
                    renderer.grid_state = grid.clone();
                    generation += 1;
                    print!("Generation {generation}\r");
                    std::io::stdout().flush()?;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::LEFT),
                    ..
                } => {
                    if let Some(old_grid) = old_grids.pop() {
                        grid = old_grid;
                        renderer.grid_state = grid.clone();
                        generation -= 1;
                        print!("Generation {generation}     \r");
                        std::io::stdout().flush()?;
                    } else {
                        println!("Already at oldest change!");
                    }
                }
                _ => (),
            }
        }

        if drawing {
            let x = event_pump.mouse_state().x();
            let y = event_pump.mouse_state().y();
            if (0..WINDOW_SIZE.0 as i32).contains(&x) && (0..(WINDOW_SIZE.1) as i32).contains(&y) {
                let gridy = (y / GRID_SIZE as i32) as usize;
                let gridx = (x / GRID_SIZE as i32) as usize;
                grid[gridy][gridx] = true;
                renderer.grid_state = grid.clone();
            }
        } else if erasing {
            let x = event_pump.mouse_state().x();
            let y = event_pump.mouse_state().y();
            if (0..WINDOW_SIZE.0 as i32).contains(&x) && (0..(WINDOW_SIZE.1) as i32).contains(&y) {
                let gridy = (y / GRID_SIZE as i32) as usize;
                let gridx = (x / GRID_SIZE as i32) as usize;
                grid[gridy][gridx] = false;
                renderer.grid_state = grid.clone();
            }
        }

        if autoplay {
            old_grids.push(grid.clone());
            grid = conway::step_forward(grid);
            renderer.grid_state = grid.clone();
            generation += 1;
            print!("Generation {generation}\r");
            std::io::stdout().flush()?;
        }

        renderer.draw(show_grid)?;
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / refresh_rate));
    }

    Ok(())
}
