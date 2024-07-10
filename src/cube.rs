use crate::alg_index;
use std::collections::HashMap;

pub fn apply_move(mut cube: [u8; 16], move_array: [u8; 16]) -> [u8; 16] {
    let initial_state: [u8; 16] = cube.clone();

    // Permutation
    for i in 0..7 {
        cube[i] = initial_state[move_array[i] as usize];
    }

    // Orientation
    for i in 8..15 {
        cube[i] = (initial_state[(move_array[i - 8] + 8) as usize] + move_array[i]) % 3;
    }

    return cube;
}

pub fn apply_alg(mut cube: [u8; 16], alg: String, move_map: &HashMap<&str, [u8; 16]>) -> [u8; 16] {
    let moves: Vec<&str> = alg.split(" ").collect();
    for current_move in moves.iter() {
        cube = apply_move(cube, move_map[current_move]);
    }
    return cube;
}

pub fn get_cube_state(scramble: &str, move_map: &HashMap<&str, [u8; 16]>) -> [u8; 16] {
    let mut cube: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0];
    if scramble == "" {
        return cube;
    } else {
        cube = apply_alg(cube, scramble.to_string(), move_map);
        return cube;
    }
}

pub fn inverse_solution(solution: &str) -> String {
    let mut output: String = String::new();
    let moves: Vec<&str> = solution.split(" ").collect();
    let reversed_moves: Vec<&str> = moves.iter().copied().rev().collect();
    for current_move in reversed_moves.iter() {
        match current_move.chars().last().unwrap() {
            '\'' => output.push_str(&current_move.replace("'", "")),
            '2' => output.push_str(&current_move),
            _ => {
                output.push_str(&current_move);
                output.push_str("'");
            }
        }
        output.push_str(" ");
    }
    return output.trim().to_string();
}

pub fn get_id_from_state(cube: [u8; 16]) -> u32 {
    let mut id_0: u32 = 0;
    for i in 0..7 {
        id_0 += cube[i] as u32 * 7_u32.pow(i as u32);
    }

    let mut id_1: u32 = 0;
    for i in 0..7 {
        id_1 += cube[i + 8] as u32 * 3_u32.pow(i as u32);
    }

    return id_0 * 3_u32.pow(6) + id_1;
}

pub fn generate_all_algs(depth: u8, print_progress: bool) -> Vec<String> {
    let mut all_algs = Vec::new();
    for i in 1..=depth {
        if print_progress {
            println!("Generating table of depth {}", i);
        }
        let mut alg_index: alg_index::AlgIndex = alg_index::assign_alg_index(i as usize);
        let start_alg: String = alg_index.to_string();
        all_algs.push(alg_index.to_string());
        alg_index.increment();
        while alg_index.to_string() != start_alg {
            all_algs.push(alg_index.to_string());
            alg_index.increment();
        }
    }
    all_algs
}

pub fn generate_table(depth: u8, print_progress: bool) -> HashMap<u32, String> {
    if print_progress {
        println!("Generating table...");
    }
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
    let algs: Vec<String> = generate_all_algs(depth, print_progress);
    let mut table: HashMap<u32, String> = HashMap::new();
    for alg in algs.iter() {
        let cube = get_cube_state(alg, &move_map);
        let id = get_id_from_state(cube);
        if !table.contains_key(&id) {
            table.insert(id, inverse_solution(alg));
        }
    }
    table
}

pub fn solve(
    mut cube: [u8; 16],
    search_algs: &Vec<String>,
    table: &HashMap<u32, String>,
) -> String {
    if cube == [0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0] {
        return "".to_string();
    }

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
    let id: u32 = get_id_from_state(cube);
    if let Some(solution) = table.get(&id) {
        return solution.to_string();
    }

    for alg in search_algs.iter() {
        cube = apply_alg(cube, alg.to_string(), &move_map);
        let id = get_id_from_state(cube);
        if let Some(solution) = table.get(&id) {
            return format!("{} {}", alg, solution.to_string());
        }
        cube = apply_alg(cube, inverse_solution(alg), &move_map);
    }
    return "No solution found".to_string();
}
