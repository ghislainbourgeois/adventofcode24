use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should be able to read input");

    let mut map: Vec<Vec<u32>> = Vec::new();
    for line in contents.lines() {
        map.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let trailheads = find_trailheads(&map);

    let ratings_of_trailheads: Vec<_> = trailheads.iter().map(|t| t.score(&map)).collect();


    let scores: usize = ratings_of_trailheads.iter().map(|trailhead| trailhead.len()).sum();

    println!("Trail heads scores: {scores}");

    let ratings: usize = ratings_of_trailheads.iter().map(|trailhead| trailhead.values().sum::<usize>()).sum();

    println!("Trail heads ratings: {ratings}");
}

fn find_trailheads(map: &Vec<Vec<u32>>) -> Vec<TrailHead> {
    let mut trailheads = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 0 {
                trailheads.push(TrailHead::new(i, j));
            }
        }
    }
    trailheads
}

#[derive(Debug)]
struct TrailHead {
    coordinates: (usize, usize)
}

impl TrailHead {
    fn new(i: usize, j: usize) -> TrailHead {
        TrailHead { coordinates: (i, j) }
    }

    fn score(&self, map: &Vec<Vec<u32>>) -> HashMap<(usize, usize), usize> {
        find_top(self.coordinates.0, self.coordinates.1, 0, map)
    }
}

fn find_top(i: usize, j: usize, next_val: u32, map: &Vec<Vec<u32>>) -> HashMap<(usize, usize), usize> {
    if map[i][j] != next_val {
        return HashMap::new();
    }
    if next_val == 9 {
        let mut top = HashMap::new();
        top.insert((i, j), 1);
        return top;
    }
    let up = match i == 0 {
        false => find_top(i - 1, j, next_val + 1, map),
        true => HashMap::new()
    };
    let down = match i == map.len() - 1 {
        false => find_top(i + 1, j, next_val + 1, map),
        true => HashMap::new()
    };
    let left = match j == 0 {
        false => find_top(i, j - 1, next_val + 1, map),
        true => HashMap::new()
    };
    let right = match j == map[0].len() - 1 {
        false => find_top(i, j + 1, next_val + 1, map),
        true => HashMap::new()
    };
    let mut tops = HashMap::new();
    for (k, v) in up.iter() {
        tops.insert(*k, *v);
    }
    for (k, v) in down.iter() {
        if let Some(orig) = tops.get_mut(k) {
            *orig = *orig + v;
        } else {
            tops.insert(*k, *v);
        }
    }
    for (k, v) in left.iter() {
        if let Some(orig) = tops.get_mut(k) {
            *orig = *orig + v;
        } else {
            tops.insert(*k, *v);
        }
    }
    for (k, v) in right.iter() {
        if let Some(orig) = tops.get_mut(k) {
            *orig = *orig + v;
        } else {
            tops.insert(*k, *v);
        }
    }
    tops
}
