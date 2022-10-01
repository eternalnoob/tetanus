use std::{thread, time};

const GRID_SIZE: usize = 10;
const NUM_ITER: i32 = 10;
const DESIRED_FRAMES: u64 = 5;
const FRAME_DELAY: time::Duration = time::Duration::from_millis(1000 / DESIRED_FRAMES);

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

type Grid = [i32; GRID_SIZE*GRID_SIZE];

fn get_at(g: Grid, i: usize, j: usize) -> i32 {
    // row major
    g[i*GRID_SIZE + j]
}
fn set_at(g: &mut Grid, i: usize, j: usize, val: i32){
    // row major
    g[i*GRID_SIZE + j] = val;
}

// row, column
fn to_coord(i: usize) -> (usize, usize) {
    (i / GRID_SIZE, i % GRID_SIZE)
}

fn main() {
    let mut life_even = [0; GRID_SIZE*GRID_SIZE];

    // simple blinker
    set_at(&mut life_even, 1,1, 1);
    set_at(&mut life_even, 1,2, 1);
    set_at(&mut life_even, 1,3, 1);
    set_at(&mut life_even, 1,4, 1);
    set_at(&mut life_even, 1,5, 1);

    let mut life_odd = [0; GRID_SIZE*GRID_SIZE];

    for i in 0..NUM_ITER {
        let now = time::Instant::now();
        println!("\niteration {}", i);
        match i % 2 == 0 {
            true => {
                print_arr(life_even);
                run_step(life_even, &mut life_odd)
            },
            false => { 
                print_arr(life_odd);
                run_step(life_odd, &mut life_even)
            }
        }
        let passed = now.elapsed(); 
        match FRAME_DELAY.checked_sub(passed) {
            Some(d) => thread::sleep(d),
            None => {},
        }
    }
}

fn lives(src: Grid, i: usize, j: usize) -> bool {
    let neigh_alive: i32 = OFFSETS.map( |(i_o, j_o)|
        {
            let i_coord = get_coord(i, i_o);
            let j_coord = get_coord(j, j_o);
            match (i_coord, j_coord) {
                (Some(i_d), Some(j_d)) => {
                    if i_d >= GRID_SIZE || j_d >= GRID_SIZE {
                        0
                    } else {
                        get_at(src, i_d, j_d)
                    }
                },
                (_, _) => 0,
            }
        }
    ).into_iter().sum();

    match get_at(src, i, j) == 1 {
        true => {
            // we alive now
            neigh_alive == 2 || neigh_alive == 3 
        },
        false => {
            neigh_alive == 3
        },
    }
}

fn run_step(from: Grid, to: &mut Grid) {
    for i in 0..from.len(){
        match from.get(i) {
            Some(_) => {
                let (i, j) = to_coord(i);
                let res = match lives(from, i, j) {
                    true => 1,
                    false => 0,
                };
                set_at(to, i, j, res);
            },
            None => todo!(),
        }
    }
}

fn print_arr(life_grid: Grid) {
    for i in 0..life_grid.len(){
        if i % GRID_SIZE == 0 {
            println!("");
        } 
        match life_grid.get(i) {
            Some(life_row) => {
                match life_row {
                    0 => print!("\u{25FB} "),
                    1 => print!("\u{25FC} "),
                    _ => panic!("lol impossible"),
                }
            },
            None => panic!("my code never has bugs"),
        }
    }
}
