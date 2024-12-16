use std::fs;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should be able to read input");
    
    let mut button_a: Button = Button{x: 0, y: 0};
    let mut button_b: Button = Button{x: 0, y: 0};
    let mut prizes: Vec<Prize> = Vec::new();

    for line in contents.lines() {
        match line.split(":").next() {
            Some("Button A") => { button_a = Button::from_str(line); },
            Some("Button B") => { button_b = Button::from_str(line); },
            Some("Prize") => {
                prizes.push(Prize::from_str_and_buttons(line, button_a.clone(), button_b.clone()));
            },
            _ => { }
        }
    }

    let minimum_tokens: usize = prizes.iter().map(|p| p.part_one()).sum();
    println!("Minimum tokens to win all possible prizes: {minimum_tokens}");
    let minimum_tokens: usize = prizes.iter().map(|p| p.part_two()).sum();
    println!("Minimum tokens to win all possible prizes: {minimum_tokens}");
}

#[derive(Clone, Debug)]
struct Button {
    x: usize,
    y: usize,
}

impl Button {
    fn from_str(line: &str) -> Self {
        let fields: Vec<&str> = line.split("+").collect();
        if fields.len() != 3 {
            panic!("Wrong button format");
        }
        let x: usize = fields[1].split(",").next().expect("Should have a comma").parse().expect("Should be a number");
        let y: usize = fields[2].parse().expect("Should be a number");
        Button{x, y}
    }
}

#[derive(Debug)]
struct Prize {
    x: usize,
    y: usize,
    button_a: Button,
    button_b: Button
}

impl Prize {
    fn from_str_and_buttons(line: &str, button_a: Button, button_b: Button) -> Self {
        let fields: Vec<&str> = line.split("=").collect();
        if fields.len() != 3 {
            panic!("Wrong prize format");
        }
        let x: usize = fields[1].split(",").next().expect("Should have a comma").parse().expect("Should be a number");
        let y: usize = fields[2].parse().expect("Should be a number");
        Prize{x, y, button_a, button_b}
    }

    fn part_one(&self) -> usize {
        let line1: ((f64, f64), (f64, f64)) = (
            (0.0, (self.x as f64 / self.button_b.x as f64)),
            ((self.x as f64 / self.button_a.x as f64), 0.0)
        );
        let line2: ((f64, f64), (f64, f64)) = (
            (0.0, (self.y as f64 / self.button_b.y as f64)),
            ((self.y as f64 / self.button_a.y as f64), 0.0)
        );

        let (times_a, times_b) = self.intersection(line1, line2);

        if times_a > 100 || times_b > 100 {
            return 0;
        }
        let new_x = times_a * self.button_a.x + times_b * self.button_b.x;
        let new_y = times_a * self.button_a.y + times_b * self.button_b.y;
        if new_x != self.x || new_y != self.y {
            return 0;
        }
        times_b + 3 * times_a
    }

    fn intersection(&self, line1: ((f64, f64), (f64, f64)), line2: ((f64, f64), (f64, f64))) -> (usize, usize) {
        let xdiff: (f64, f64) = (
            (line1.0.0 - line1.1.0),
            (line2.0.0 - line2.1.0)
        );
        let ydiff: (f64, f64) = (
            (line1.0.1 - line1.1.1),
            (line2.0.1 - line2.1.1)
        );

        let div = Self::det(xdiff, ydiff);
        if div == 0.0 {
            return (0, 0);
        }

        let d = (Self::det(line1.0, line1.1), Self::det(line2.0, line2.1));
        
        let times_a: usize = (Self::det(d, xdiff) / div).round() as usize;
        let times_b: usize = (Self::det(d, ydiff) / div).round() as usize;
        (times_a, times_b)
    }

    fn det(a: (f64, f64), b: (f64, f64)) -> f64 {
        a.0 * b.1 - a.1 * b.0
    }

    fn x(&self) -> usize {
        self.x + 10000000000000
    }

    fn y(&self) -> usize {
        self.y + 10000000000000
    }

    fn part_two(&self) -> usize {
        let line1: ((f64, f64), (f64, f64)) = (
            (0.0, (self.x() as f64 / self.button_b.x as f64)),
            ((self.x() as f64 / self.button_a.x as f64), 0.0)
        );
        let line2: ((f64, f64), (f64, f64)) = (
            (0.0, (self.y() as f64 / self.button_b.y as f64)),
            ((self.y() as f64 / self.button_a.y as f64), 0.0)
        );

        let (times_a, times_b) = self.intersection(line1, line2);

        let new_x = times_a * self.button_a.x + times_b * self.button_b.x;
        let new_y = times_a * self.button_a.y + times_b * self.button_b.y;
        if new_x != self.x() || new_y != self.y() {
            return 0;
        }
        times_b + 3 * times_a
    }
}
