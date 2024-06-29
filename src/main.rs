#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use std::{collections::HashMap, fs}; 
use std::{env, io, time::Instant};
mod alg_index;
mod cube;


fn main() {
    let move_map: HashMap<&str, [u8; 16]> = HashMap::from([
        ("R", [0, 2, 5, 3, 4, 6, 1, 7, 0, 1, 2, 0, 0, 1, 2, 0]),
        ("U", [3, 0, 1, 2, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0]),
        ("F", [0, 1, 3, 4, 5, 2, 6, 7, 0, 0, 1, 2, 1, 2, 0, 0]),
        ("R'", [0, 6, 1, 3, 4, 2, 5, 7, 0, 1, 2, 0, 0, 1, 2, 0]),
        ("U'", [1, 2, 3, 0, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0]),
        ("F'", [0, 1, 5, 2, 3, 4, 6, 7, 0, 0, 1, 2, 1, 2, 0, 0]),
        ("R2", [0, 5, 6, 3, 4, 1, 2, 7, 0, 0, 0, 0, 0, 0, 0, 0]),
        ("U2", [2, 3, 0, 1, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0]),
        ("F2", [0, 1, 4, 5, 2, 3, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0]),
    ]);

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut scramble = String::new();
    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }
        scramble.push_str(arg);
        scramble.push_str(" ");
    }
    scramble = scramble.trim().to_string();

    // let now = Instant::now();
    // let data_file = fs::read_to_string("scrambles.txt").expect("Failed to read scrambles");
    // println!("Reading file: {:?}", now.elapsed());


    // let mut puzzle: [u8; 16] = solved_state;
    // let t_perm: [u8; 16] = [0, 2, 1, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0];

    let search_algs: Vec<String> = cube::generate_all_algs(3);
    let table: HashMap<u32, String> = cube::generate_table(7);
    // let now = Instant::now();
    // let cube = get_cube_state("F' U2 R U R' U2 R U' R' U' R", &move_map);
    let now = Instant::now();
    let solution = cube::solve(cube::get_cube_state(&scramble, &move_map), &search_algs, &table);
    println!("Time elapsed: {:?}", now.elapsed());
    let moves: Vec<&str> = solution.split(" ").collect();
    // println!("{} ({})", solution, moves.len());
    // println!("{:?}", now.elapsed());
    
    
    
    println!("\nSolution:");
    println!("{} ({})", solution, moves.len());

    // let now = Instant::now();
    // for line in data_file.lines() {
    //     let cube: [u8; 16] = get_cube_state(line, &move_map);
    //     solve(cube, &search_algs, &table);
    //     let solution: String = solve(cube, &search_algs, &table);
    //     let solution_array: Vec<&str> = solution.split(" ").collect();
    //     println!("{} ({})", solution, solution_array.len());
    // }
}

#[allow(dead_code)]
fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    return input.trim().to_string()
}
