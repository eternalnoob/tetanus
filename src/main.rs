extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::EventLoop;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::{cmp, usize};
use std::{thread, time};
use std::io::{self, Write};
use rand::distributions::{Distribution, Uniform};



use::rayon::prelude::*;

const GRID_SIZE: usize = 600;
const NUM_ITER: i32 = 100;
const DESIRED_FRAMES: u64 = 5;
const FRAME_DELAY: time::Duration = time::Duration::from_millis(1000 / DESIRED_FRAMES);
const PRINT_OUTPUT: bool = false;


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    state: Vec<i8>,
    rendered: bool,
}
impl App {
    fn render(&mut self, args: &RenderArgs, status: &Vec<i8>) {
        self.state = status.clone();
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square_size: f64 = (cmp::min(
            (args.window_size[0] * 0.75) as usize,
            (args.window_size[1] * 0.75) as usize) / GRID_SIZE) as f64;
            self.gl.draw(args.viewport(), |c, gl| {
                // Clear the screen.
                clear(BLACK, gl);
            });

            for i in  0..(GRID_SIZE*GRID_SIZE) {
                let (row, col) = to_coord(i);
                let square = rectangle::square((square_size + 1.5) * col as f64,
                (square_size + 1.5) * row as f64,
                square_size);
                // let rotation = self.rotation;
                let (x, y) = (
                    (square_size + 1.5) * col as f64,
                    (square_size + 1.5) * row as f64
                );

                self.gl.draw(args.viewport(), |c, gl| {
                    let transform = c
                        .transform
                        .trans(0.0, 0.0);

                    let color = match self.state[i] {
                        0 => BLACK,
                        _ => WHITE,
                    };
                    rectangle(color, square, transform, gl);
                });
            }
            self.rendered = true;
    }

    fn update(&mut self, args: &UpdateArgs, status: &Vec<i8>) {
        self.state = status.clone();
    }
}


enum Offset {
    Neg(usize),
    Pos(usize),
}

fn get_coord(i: usize, offset: Offset) -> Option<usize> {
    match offset {
        Offset::Pos(offset) => i.checked_add(offset),
        Offset::Neg(offset) => i.checked_sub(offset)
    }
} 

const OFFSETS: [(Offset, Offset); 8] = [
    (Offset::Neg(1), Offset::Neg(1)), (Offset::Neg(1), Offset::Pos(0)), (Offset::Neg(1), Offset::Pos(1)),
    (Offset::Pos(0), Offset::Neg(1)),                                   (Offset::Pos(0), Offset::Pos(1)),
    (Offset::Pos(1), Offset::Neg(1)), (Offset::Pos(1), Offset::Pos(0)), (Offset::Pos(1), Offset::Pos(1)),
];

fn vec_at(g: &Vec<i8>, i: usize, j: usize) -> i8 {
    // row major
    g[i*GRID_SIZE + j]
}
fn set_vec(g: &mut Vec<i8>, i: usize, j: usize, val: i8){
    // row major
    g[i*GRID_SIZE + j] = val
}

// row, column
fn to_coord(i: usize) -> (usize, usize) {
    (i / GRID_SIZE, i % GRID_SIZE)
}

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(20).build_global().unwrap();
    //let mut life_even = [0; GRID_SIZE*GRID_SIZE];

    // simple blinker
    let mut rng = rand::thread_rng();
    let die: Uniform<i8>= Uniform::from(0..2);

    let mut life_vec: Vec<i8> = (0..GRID_SIZE*GRID_SIZE).map( |_|
        die.sample(&mut rng)
    ).collect();


    set_vec(&mut life_vec, 1,1, 1);
    set_vec(&mut life_vec, 1,2, 1);
    set_vec(&mut life_vec, 1,3, 1);
    set_vec(&mut life_vec, 1,4, 1);
    set_vec(&mut life_vec, 1,5, 1);

    //let mut life_odd = [0; GRID_SIZE*GRID_SIZE];

    let mut settings = EventSettings::new();
    settings.set_max_fps(1);
    settings.set_ups(0);

    let mut total_lag = time::Duration::from_millis(0);

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [1920, 1200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        state: life_vec.clone(),
        rendered: false,
    };

    let mut events = Events::new(settings);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            life_vec = run_vec_step(&life_vec);
            app.render(&args, &life_vec);
            print!("getting rendered?");
        }
    }

}

fn run_vec_step(from: &Vec<i8>) -> Vec<i8>  {
    (0..from.len()).into_par_iter().map( |idx|
        vec_lives(&from, idx)
    ).collect()
}


fn vec_lives(src: &Vec<i8>, idx:usize) -> i8 {
    let (i, j) = to_coord(idx); 
    let neigh_alive: i8 = OFFSETS.map( |(i_o, j_o)|
        {
            let i_coord = get_coord(i, i_o);
            let j_coord = get_coord(j, j_o);
            match (i_coord, j_coord) {
                (Some(i_d), Some(j_d)) => {
                    if i_d >= GRID_SIZE || j_d >= GRID_SIZE {
                        0
                    } else {
                       vec_at(&src, i_d, j_d)
                    }
                },
                (_, _) => 0,
            }
        }
    ).into_iter().sum();

    match vec_at(&src, i, j) == 1 {
        true => {
            // we alive now
            (neigh_alive == 2 || neigh_alive == 3).into()
        },
        false => {
            (neigh_alive == 3).into()
        },
    }
}

fn print_vec(life_grid: &Vec<i8>) {
    print!("{}", (0..life_grid.len()).into_par_iter().map( |idx|
        match (idx%GRID_SIZE, &life_grid[idx]) {
            (0, 0) => "\n\u{25FB} ".to_string(),
            (0, 1) => "\n\u{25FC} ".to_string(),
            (_, 0) => "\u{25FB} ".to_string(),
            (_, 1) => "\u{25FC} ".to_string(),
            (_, _) => "".to_string(),
        }
    ).collect::<String>().to_string());
}
