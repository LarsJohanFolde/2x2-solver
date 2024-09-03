use crate::cube;
use std::{collections::HashMap, fs, fs::File, io::Write};

pub fn file_to_csv(scramble_file: &str, search_algs: Vec<String>, table: HashMap<u32, String>) -> () {
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
    let mut cube = cube::Cube::new();

    // CSV Dump
    for line in data_file.lines() {
        let line_content = line.to_string().replace("\t", "");
        let data: Vec<&str> = line_content.split(";").collect();
        let scramble: &str = data[0];
        cube.state = cube::Cube::from(scramble).state;
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
