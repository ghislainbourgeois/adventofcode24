use std::{collections::{HashMap, HashSet}, fs};

mod vec2;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should be able to read input");

    let mut boundary = vec2::Vec2 {
        a: 0,
        b: 0
    };

    let mut antennas: HashMap<char, Vec<vec2::Vec2>> = HashMap::new();
    for (a, row) in contents.lines().enumerate() {
        boundary.a = a.try_into().unwrap();
        for (b, char) in row.chars().enumerate() {
            boundary.b = b.try_into().unwrap();
            match char {
                '.' => {},
                _ => {
                    let v = vec2::Vec2{
                        a: a.try_into().unwrap(),
                        b: b.try_into().unwrap(),
                    };
                    if let Some(freq_antennas) = antennas.get_mut(&char) {
                        freq_antennas.push(v);
                    } else {
                        antennas.insert(char, vec![v]);
                    }
                },
            }
        }
    }

    boundary.a += 1;
    boundary.b += 1;

    let mut antinodes: HashSet<vec2::Vec2> = HashSet::new();
    for (_, freq_antennas) in antennas.iter() {
        for (i, a1) in freq_antennas.iter().enumerate() {
            for a2 in &freq_antennas[i..] {
                if a1 == a2 {
                    continue;
                }
                let antinode_distance = (a2 - a1) * 2;
                let antinode = &antinode_distance + a1;
                if antinode.is_inside(&boundary) {
                    antinodes.insert(antinode);
                }
                let antinode = a2 - &antinode_distance;
                if antinode.is_inside(&boundary) {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    println!("Number of unique antinodes: {}", antinodes.len());

    let mut antinodes: HashSet<vec2::Vec2> = HashSet::new();
    for (_, freq_antennas) in antennas.iter() {
        for (i, a1) in freq_antennas.iter().enumerate() {
            for a2 in &freq_antennas[i..] {
                if a1 == a2 {
                    continue;
                }
                antinodes.insert(vec2::Vec2{a: a1.a, b: a1.b});
                antinodes.insert(vec2::Vec2{a: a2.a, b: a2.b});
                for i in 2.. {
                    let antinode_distance = (a2 - a1) * i;

                    let antinode = &antinode_distance + a1;
                    let antinode1_inside = antinode.is_inside(&boundary);
                    if antinode1_inside {
                        antinodes.insert(antinode);
                    }

                    let antinode = a2 - &antinode_distance;
                    let antinode2_inside = antinode.is_inside(&boundary);
                    if antinode2_inside {
                        antinodes.insert(antinode);
                    }

                    if !antinode1_inside && !antinode2_inside {
                        break;
                    }
                }
            }
        }
    }

    println!("Number of unique antinodes: {}", antinodes.len());
}
