use std::{thread, time};

const GRID_SIZE: usize = 40;
const NUM_ITER: i32 = 1000000;
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

type Grid = [[i32; GRID_SIZE]; GRID_SIZE];

fn main() {
    let mut life_even = [[0; GRID_SIZE] ; GRID_SIZE];

    // simple blinker
    life_even[1][1] = 1;
    life_even[1][2] = 1;
    life_even[1][3] = 1;
    life_even[1][4] = 1;
    life_even[1][5] = 1;

    let mut life_odd = [[0; GRID_SIZE] ; GRID_SIZE];

    for i in 0..NUM_ITER {
        let now = time::Instant::now();
        println!("iteration {}", i);
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
        println!("\n");
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
                        src[i_d][j_d]
                    }
                },
                (_, _) => 0,
            }
        }
    ).into_iter().sum();

    match src[i][j] == 1 {
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
            Some(from_row) => {
                for j in 0..from_row.len(){
                    to[i][j] = match lives(from, i, j) {
                        true => 1,
                        false => 0,
                    };
                }
            },
            None => todo!(),
        }
    }
}

fn print_arr(life_grid: Grid) {
    for i in 0..life_grid.len(){
        match life_grid.get(i) {
            Some(life_row) => {
                for j in 0..life_row.len(){
                    match life_row.get(j) {
                        Some(0) => print!("\u{25FB} "),
                        Some(1) => print!("\u{25FC} "),
                        Some(_) => panic!("lol impossible"),
                        None => panic!(),
                    }
                }
                println!("")
            },
            None => panic!("my code never has bugs"),
        }
    }
}
