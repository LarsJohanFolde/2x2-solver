pub struct AlgIndex {
    depth: usize,
    alg: Vec<u8>,
}

pub fn assign_alg_index(depth: usize) -> AlgIndex {
    let pattern = [0, 3];
    let repeats = (depth / 2) + 1;
    let mut result = Vec::with_capacity(depth);
    for _ in 0..repeats {
        result.extend_from_slice(&pattern);
    }
    result.truncate(depth);
    AlgIndex {
        depth,
        alg: result,
    }
}

impl AlgIndex {
    pub fn inc(&mut self) {
        for i in 0..self.alg.len() {
            self.alg[i] += 1;
            self.alg[i] %= 9;
            if self.alg[i] != 0 {
                break;
            }
        }
    }
    
    pub fn increment(&mut self) {
        self.inc();
        while !self.is_valid() {
            self.inc();
        }
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..self.depth-1 {
            if self.alg[i] / 3 == self.alg[i+1] / 3 {
                return false
            }
        }
        return true
    }

    pub fn to_string(&self) -> String {
        const MOVES: [&str; 9] = ["U", "U2", "U'", "R", "R2", "R'", "F", "F2", "F'"];
        let mut alg_string = String::new();
        for value in self.alg.iter() {
            alg_string.push_str(MOVES[*value as usize]);
            alg_string.push_str(" ");
        }
        alg_string.trim().to_string()
    }
}
