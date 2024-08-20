use csv_export::file_to_csv;
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
mod csv_export;

fn main() {
    let search_algs = generate_all_algs(3, true);
    let table = generate_table(8, true);
    // let mut cube = cube::from("R U R2 F R'");

    let now = Instant::now();
    file_to_csv("scrambles.txt", search_algs, table);
    println!("{:?}", now.elapsed());
}

#[allow(dead_code)]
fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    return input.trim().to_string();
}
