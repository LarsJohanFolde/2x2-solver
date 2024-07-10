#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use std::{collections::HashMap, fs}; 
use std::{env, io, time::Instant};
mod alg_index;
mod cube;
use std::fs::File;
use std::io::Write;


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

    // Solve by args
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    //
    // let mut scramble = String::new();
    // for (i, arg) in args.iter().enumerate() {
    //     if i == 0 {
    //         continue;
    //     }
    //     scramble.push_str(arg);
    //     scramble.push_str(" ");
    // }
    // scramble = scramble.trim().to_string();

    // let now = Instant::now();
    let data_file = fs::read_to_string("scrambles.txt").expect("Failed to read scrambles");
    let output_file = File::options()
        .read(true)
        .write(true)
        .open("output.csv")
        .expect("Failed to open file");
    let column_header: String = format!(
        "{},{},{},{},{},{},{},{},{},{}",
        "Scramble",
        "Solution",
        "Move Count",
        "Competition Name",
        "Competition ID",
        "Round Type ID",
        "Group ID",
        "Is Extra",
        "Scramble Num",
        "Country ID"
    );
    writeln!(&output_file, "{}", column_header).expect("Failed to write header");
    // println!("Reading file: {:?}", now.elapsed());
    
    // println!("{:?}", get_cube_state("F U F", &move_map));


    // let mut puzzle: [u8; 16] = solved_state;
    // let t_perm: [u8; 16] = [0, 2, 1, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0];

    let search_algs: Vec<String> = cube::generate_all_algs(3, false);
    let table: HashMap<u32, String> = cube::generate_table(9, true);
    // let now = Instant::now();
    // let cube = get_cube_state("F' U2 R U R' U2 R U' R' U' R", &move_map);
    // let now = Instant::now();
    // let solution = cube::solve(cube::get_cube_state(&scramble, &move_map), &search_algs, &table);
    // let moves: Vec<&str> = solution.split(" ").collect();
    // println!("{} ({})", solution, moves.len());
    // println!("{:?}", now.elapsed());
    

    let now = Instant::now();
    // CSV Dump
    for line in data_file.lines() {
        let line_content = line.to_string().replace("\t", "");
        let data: Vec<&str> = line_content.split(";").collect();
        let scramble: &str = data[0];
        let cube = cube::get_cube_state(scramble, &move_map);
        let competition_id: &str = data[2];
        let solution: String = cube::solve(cube, &search_algs, &table);
        let solution_vec: Vec<&str> = solution.split(" ").collect();
        let move_count: usize = solution_vec.len();
        let competition_name: &str = data[1];
        let round_type_id: char = data[3].parse::<char>().expect("round_type_id is not char");
        let group_id: &str = data[4];
        let is_extra: char = data[5].parse::<char>().expect("is_extra is not char");
        let scramble_num: u8 = data[6].parse::<u8>().expect("scramble_num is not u8");
        let country_id: &str = data[7];

        let csv_line: String = format!(
            "{},{},{},{},{},{},{},{},{},{}",
            scramble,
            solution,
            move_count,
            competition_name,
            competition_id,
            round_type_id,
            group_id,
            is_extra,
            scramble_num,
            country_id
        );

        writeln!(&output_file, "{}", csv_line).expect("Failed to write data");
        
    }
    
    
    
    // println!("{} ({}), in {:?}", solution, moves.len(), now.elapsed());

    // for line in data_file.lines() {
    //     let cube: [u8; 16] = cube::get_cube_state(line, &move_map);
    //     cube::solve(cube, &search_algs, &table);
        // let solution: String = cube::solve(cube, &search_algs, &table);
        // let solution_array: Vec<&str> = solution.split(" ").collect();
        // println!("{} ({})", solution, solution_array.len());
    // }
    println!("{:?}", now.elapsed());
}

#[allow(dead_code)]
fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    return input.trim().to_string()
}
