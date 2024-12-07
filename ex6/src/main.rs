use std::{collections::HashSet, fs};

mod map;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should be able to read input");
    
    let map = map::LabMap::from_string(contents);
    match map {
        Ok(mut m) => {
            let unique_positions: HashSet<map::Guard> = m.guard_patrol().collect();
            println!("Unique positions: {}", unique_positions.len());
            let possible_obstacles = m.count_possible_obstacles();
            println!("Possible obstacles: {}", possible_obstacles);
        },
        Err(e) => { println!("{e}"); },
    }
}
