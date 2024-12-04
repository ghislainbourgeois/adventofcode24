use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Could not read input.txt file");

    let mut stack: String = String::new();
    let mut num1: usize = 0;
    let mut num2: usize;
    let mut program: Vec<Box<dyn Instruction>> = Vec::new();

    for char in contents.chars() {
        match char {
            'd' => {
                stack.truncate(0);
                stack.push(char);
            },
            'o' => {
                if stack == "d".to_string() {
                    stack.push(char);
                } else {
                    stack.truncate(0);
                }
            },
            'n' => {
                if stack == "do".to_string() {
                    stack.push(char);
                } else {
                    stack.truncate(0);
                }
            },
            '\'' => {
                if stack == "don".to_string() {
                    stack.push(char);
                } else {
                    stack.truncate(0);
                }
            },
            't' => {
                if stack == "don'".to_string() {
                    stack.push(char);
                } else {
                    stack.truncate(0);
                }
            },
            'm' => {
                stack.truncate(0);
                stack.push(char);
            },
            'u' => {
                if stack == "m".to_string() {
                    stack.push(char);
                } else {
                    stack.truncate(0);
                }
            },
            'l' => {
                if stack == "mu".to_string() {
                    stack.push(char);
                } else {
                    stack.truncate(0);
                }
            },
            '(' => {
                if stack == "mul".to_string() || stack == "do".to_string() || stack == "don't".to_string() {
                    stack.push(char);
                } else {
                    stack.truncate(0);
                }
            },
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                if !stack.is_empty() && "0123456789(,".contains(stack.chars().nth_back(0).as_slice()) {
                    stack.push(char);
                } else {
                    stack.truncate(0);
                }
            },
            ',' => {
                if "0123456789".contains(stack.chars().nth_back(0).as_slice())
                    && !stack.contains(",") {
                    num1 = stack.split_once('(').unwrap().1.parse().expect("Should be the first number");
                    if num1 < 1000 {
                        stack.push(char);
                    } else {
                        stack.truncate(0);
                    }
                } else {
                    stack.truncate(0);
                }
            },
            ')' => {
                if !stack.is_empty() && "0123456789".contains(stack.chars().nth_back(0).as_slice()) && stack.contains(",") {
                    num2 = stack.split_once(',').unwrap().1.parse().expect("Should be the second number");
                    if num2 < 1000 {
                        program.push(Box::new(Mul::new(num1, num2)));
                    }
                } else {
                    match stack.as_str() {
                        "do(" => {
                            program.push(Box::new(Do::new()));
                        },
                        "don't(" => {
                            program.push(Box::new(Dont::new()));
                        },
                        _ => {},
                    }
                }
                stack.truncate(0);
            },
            _ => {
                stack.truncate(0);
            },
        }
    }

    let first_interpreter = Interpreter::new(false);
    println!("Sum of multiplications: {}", first_interpreter.run(&program));
    let second_interpreter = Interpreter::new(true);
    println!("Sum of multiplications: {}", second_interpreter.run(&program));
}

pub struct Interpreter {
    accumulator: usize,
    mul_enabled: bool,
    dont_enabled: bool
}

impl Interpreter {
    pub fn new(dont_enabled: bool) -> Interpreter {
        Interpreter{
            accumulator: 0,
            mul_enabled: true,
            dont_enabled
        }
    }

    pub fn run(mut self, program: &Vec<Box<dyn Instruction>>) -> usize {
        for instruction in program.iter() {
            instruction.eval(&mut self);
        }
        
        self.accumulator
    }
}

pub trait Instruction {
    fn eval(&self, interpreter: &mut Interpreter);
}

struct Mul {
    a: usize,
    b: usize
}

impl Mul {
    pub fn new(a: usize, b: usize) -> Mul {
        Mul{a, b}
    }
}

impl Instruction for Mul {
    fn eval(&self, interpreter: &mut Interpreter) {
        if interpreter.mul_enabled {
            interpreter.accumulator += self.a * self.b;
        }
    }
}

struct Do {}

impl Do {
    pub fn new() -> Do {
        Do{}
    }
}

impl Instruction for Do {
    fn eval(&self, interpreter: &mut Interpreter) {
        interpreter.mul_enabled = true;
    }
}

struct Dont {}

impl Dont {
    pub fn new() -> Dont {
        Dont{}
    }
}

impl Instruction for Dont {
    fn eval(&self, interpreter: &mut Interpreter) {
        if interpreter.dont_enabled {
            interpreter.mul_enabled = false;
        }
    }
}
