#[allow(unused_imports)]
use cube::{generate_all_algs, generate_table, get_state_from};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{collections::HashMap, fs};
#[allow(unused_imports)]
use std::{env, io, time::Instant};
mod alg_index;
mod cube;
use std::fs::File;
use std::io::Write;

fn main() {
    let search_algs = generate_all_algs(3, true);
    let table = generate_table(8, true);
    let mut cube = cube::from("R U R2 F R'");

    println!("{}", cube.find_solution(&search_algs, &table));
}

#[allow(dead_code)]
fn file_to_csv(scramble_file: &str, search_algs: Vec<String>, table: HashMap<u32, String>) -> () {
    let data_file = fs::read_to_string(scramble_file).expect("Failed to read scrambles");
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
    let mut cube = cube::new();

    let move_map = HashMap::from([
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

    // CSV Dump
    for line in data_file.lines() {
        let line_content = line.to_string().replace("\t", "");
        let data: Vec<&str> = line_content.split(";").collect();
        let scramble: &str = data[0];
        cube.state = cube::get_state_from(scramble, &move_map);
        let competition_id: &str = data[2];
        let solution: String = cube.find_solution(&search_algs, &table);
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
}

#[allow(dead_code)]
fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    return input.trim().to_string();
}
