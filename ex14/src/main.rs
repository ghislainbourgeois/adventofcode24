extern crate bmp;
use std::{cmp, collections::HashMap, fs};
use bmp::{Image, Pixel};

const MAX_X: isize = 101;
const MAX_Y: isize = 103;

fn main() {
    let robots: Vec<Robot> = fs::read_to_string("input.txt").expect("Should be able to read input")
    .lines().map(|l| Robot::from_str(l)).collect();

    let mut easter_egg_robots = robots.clone();
    let mut quadrants: HashMap<Quadrant, usize> = HashMap::new();
    for mut robot in robots {
        robot.move_multiple(100);
        let q = robot.quadrant();
        if let Some(entry) = quadrants.get_mut(&q) {
            *entry += 1;
        } else {
            quadrants.insert(q, 1);
        }
    }

    let factor = quadrants.get(&Quadrant::UL).unwrap_or(&0) *
                quadrants.get(&Quadrant::UR).unwrap_or(&0) *
                quadrants.get(&Quadrant::LR).unwrap_or(&0) *
                quadrants.get(&Quadrant::LL).unwrap_or(&0);
    println!("Safety factor: {}", factor);

    for seconds in 1..10000 {
        let mut img = Image::new(MAX_X as u32, MAX_Y as u32);
        for r in easter_egg_robots.iter_mut() {
            r.move_once();
            img.set_pixel(r.position.0 as u32, r.position.1 as u32, Pixel::new(0, 255, 0));
        }
        let _ = img.save(format!("{}.bmp", seconds));
    }
}

#[derive(Clone)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize)
}

impl Robot {
    fn from_str(s: &str) -> Self {
        match s.split_once(" ") {
            Some((p, v)) => {
                let (x, y) = p.split_once("=").expect("Should have a position")
                    .1.split_once(",").expect("Should have coordinates");
                let (vx, vy) = v.split_once("=").expect("Should have a position")
                    .1.split_once(",").expect("Should have coordinates");
                let x: isize = x.parse().expect("Should be a number");
                let y: isize = y.parse().expect("Should be a number");
                let vx: isize = vx.parse().expect("Should be a number");
                let vy: isize = vy.parse().expect("Should be a number");
                Self {
                    position: (x, y),
                    velocity: (vx, vy)
                }
            },
            None => { panic!("Bad robot!"); }
        }
    }

    fn move_multiple(&mut self, times: usize) {
        for _ in 0..times {
            self.move_once();
        }
    }

    fn move_once(&mut self) {
        let mut new_position = (self.position.0 + self.velocity.0, self.position.1 + self.velocity.1);
        if new_position.0 < 0 {
            new_position.0 = MAX_X + new_position.0;
        } else if new_position.0 >= MAX_X {
            new_position.0 = new_position.0 - MAX_X;
        }
        if new_position.1 < 0 {
            new_position.1 = MAX_Y + new_position.1;
        } else if new_position.1 >= MAX_Y {
            new_position.1 = new_position.1 - MAX_Y;
        }
        self.position = new_position;
    }

    fn quadrant(&self) -> Quadrant {
        match self.position.1.cmp(&(MAX_Y / 2)) {
            cmp::Ordering::Less => {
                match self.position.0.cmp(&(MAX_X / 2)) {
                    cmp::Ordering::Equal => Quadrant::NA,
                    cmp::Ordering::Less => Quadrant::UL,
                    cmp::Ordering::Greater => Quadrant::UR
                }
            },
            cmp::Ordering::Greater => {
                match self.position.0.cmp(&(MAX_X / 2)) {
                    cmp::Ordering::Equal => Quadrant::NA,
                    cmp::Ordering::Less => Quadrant::LL,
                    cmp::Ordering::Greater => Quadrant::LR
                }
            },
            cmp::Ordering::Equal => Quadrant::NA
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
enum Quadrant {
    UL,
    UR,
    LR,
    LL,
    NA
}
