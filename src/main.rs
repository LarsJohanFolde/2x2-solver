use csv_export::file_to_csv;
#[allow(unused_imports)]
use cube::{Cube, generate_all_algs, generate_table};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{collections::HashMap, fs};
#[allow(unused_imports)]
use std::{env, io, time::Instant};
mod alg_index;
mod cube;
mod csv_export;

fn main() {
    let search_algs = generate_all_algs(3, true);
    let table = generate_table(8, true);
    // let mut cube = cube::from("R U R2 F R'");
    let cube = Cube::new();
    let cube2 = Cube::from("R U R' F2"); 

    println!("{}", cube2.find_solution(&search_algs, &table));

    println!("{:?}", cube.state);

    // let now = Instant::now();
    // file_to_csv("scrambles.txt", search_algs, table);
    // println!("{:?}", now.elapsed());
}

#[allow(dead_code)]
fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    return input.trim().to_string();
}
