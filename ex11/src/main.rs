use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should be able to read input");

    let rocks: Vec<u64> = contents.split_whitespace()
        .map(|s| s.parse::<u64>().expect("Should be a number")).collect();

    println!("Number of rocks: {}", rocks.len());

    let mut memoizer = Memoizer::new();
    let acc25: usize = rocks.iter().map(|e| memoizer.blink(*e, 25)).sum();
    println!("Number of stones after 25 blinks: {}", acc25);

    let acc75: usize = rocks.iter().map(|e| memoizer.blink(*e, 75)).sum();
    println!("Number of stones after 75 blinks: {}", acc75);
}

struct Memoizer {
    memory: HashMap<(u64, usize), usize>
}

impl Memoizer {
    fn new() -> Memoizer {
        Memoizer{memory: HashMap::new()}
    }

    fn blink(&mut self, engraved: u64, n_blinks: usize) -> usize {
        if n_blinks == 0 {
            return 1;
        }
        if let Some(value) = self.memory.get(&(engraved, n_blinks)) {
            return *value;
        }
        if engraved == 0 {
            let result = self.blink(1, n_blinks - 1);
            self.memory.insert((engraved, n_blinks), result);
            return result;
        }
        let s = engraved.to_string();
        if s.len() % 2 == 0 {
            let (s1, s2) = s.split_at(s.len() / 2);
            let result = self.blink(s1.parse().expect("should be a number"), n_blinks - 1)
                + self.blink(s2.parse().expect("should be a number"), n_blinks - 1);
            self.memory.insert((engraved, n_blinks), result);
            return result;
        }
        let result = self.blink(engraved * 2024, n_blinks - 1);
        self.memory.insert((engraved, n_blinks), result);
        result
    }
}
