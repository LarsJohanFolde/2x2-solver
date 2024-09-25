#![allow(unused)]
use crate::alg_index;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub struct Cube {
    pub state: [u8; 16],
    move_map: HashMap<&'static str, [u8; 16]>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_alg() {
        let mut cube = Cube {
            state: [6, 2, 3, 0, 5, 4, 1, 7, 2, 0, 0, 0, 0, 2, 2, 0],
            move_map: HashMap::from([
                ("R", [0, 2, 5, 3, 4, 6, 1, 7, 0, 1, 2, 0, 0, 1, 2, 0]),
                ("U", [3, 0, 1, 2, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0]),
                ("F", [0, 1, 3, 4, 5, 2, 6, 7, 0, 0, 1, 2, 1, 2, 0, 0]),
                ("R'", [0, 6, 1, 3, 4, 2, 5, 7, 0, 1, 2, 0, 0, 1, 2, 0]),
                ("U'", [1, 2, 3, 0, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0]),
                ("F'", [0, 1, 5, 2, 3, 4, 6, 7, 0, 0, 1, 2, 1, 2, 0, 0]),
                ("R2", [0, 5, 6, 3, 4, 1, 2, 7, 0, 0, 0, 0, 0, 0, 0, 0]),
                ("U2", [2, 3, 0, 1, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0]),
                ("F2", [0, 1, 4, 5, 2, 3, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0]),
            ])
        };

        cube.apply_alg("U2 F U' R'".to_string());

        assert_eq!(cube.state, [0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            state: [0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0],
            move_map: HashMap::from([
                ("R", [0, 2, 5, 3, 4, 6, 1, 7, 0, 1, 2, 0, 0, 1, 2, 0]),
                ("U", [3, 0, 1, 2, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0]),
                ("F", [0, 1, 3, 4, 5, 2, 6, 7, 0, 0, 1, 2, 1, 2, 0, 0]),
                ("R'", [0, 6, 1, 3, 4, 2, 5, 7, 0, 1, 2, 0, 0, 1, 2, 0]),
                ("U'", [1, 2, 3, 0, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0]),
                ("F'", [0, 1, 5, 2, 3, 4, 6, 7, 0, 0, 1, 2, 1, 2, 0, 0]),
                ("R2", [0, 5, 6, 3, 4, 1, 2, 7, 0, 0, 0, 0, 0, 0, 0, 0]),
                ("U2", [2, 3, 0, 1, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0]),
                ("F2", [0, 1, 4, 5, 2, 3, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0]),
            ])
        }
    }

    pub fn from(scramble: &str) -> Cube {
        let mut cube: Cube = Cube::new();
        if scramble == "" {
            return cube
        } else {
            cube.apply_alg(scramble.to_string());
            return cube
        }
    }

    fn apply_move(&mut self, move_array: [u8; 16]) {
        // Save the initial state of the array representation of the Cube
        let initial_state: [u8; 16] = self.state;

        // Permutation
        for i in 0..7 {
            self.state[i] = initial_state[move_array[i] as usize];
        }

        // Orientation
        for i in 8..15 {
            self.state[i] = (initial_state[(move_array[i - 8] + 8) as usize] + move_array[i]) % 3;
        }
    }

    pub fn apply_alg(&mut self, alg: String) {
        let moves: Vec<&str> = alg.split(" ").collect();
        for current_move in moves.iter() {
            self.apply_move(self.move_map[current_move]);
        }
    }

    fn get_id(&self) -> u32 {
        let mut id_0: u32 = 0;
        for i in 0..7 {
            id_0 += self.state[i] as u32 * 7_u32.pow(i as u32);
        }

        let mut id_1: u32 = 0;
        for i in 0..7 {
            id_1 += self.state[i + 8] as u32 * 3_u32.pow(i as u32);
        }

        id_0 * 3_u32.pow(6) + id_1
    }

    pub fn find_solution(
        &self,
        search_algs: &Vec<String>,
        table: &HashMap<u32, String>,
    ) -> String {
        let mut cube = self.clone();
        if self.state == [0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0] {
            return "".to_string()
        }

        if let Some(solution) = table.get(&cube.get_id()) {
            return solution.to_string()
        }

        for alg in search_algs.iter() {
            cube.apply_alg(alg.to_string());
            if let Some(solution) = table.get(&cube.get_id()) {
                return format!("{} {}", alg, solution.to_string())
            }
            cube.apply_alg(inverse_solution(alg));
        }
        "No solution found".to_string()
    }

}

fn inverse_solution(solution: &str) -> String {
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
    output.trim().to_string()
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
    let algs: Vec<String> = generate_all_algs(depth, print_progress);
    let mut table: HashMap<u32, String> = HashMap::new();
    for alg in algs.iter() {
        let cube = Cube::from(alg);
        let id = cube.get_id();
        if !table.contains_key(&id) {
            table.insert(id, inverse_solution(alg));
        }
    }
    table
}
