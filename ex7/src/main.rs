use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should be able to read input");

    let equations = contents.lines().map(|l| Equation::from_str(l)).collect::<Result<Vec<Equation>,_>>();

    match equations {
        Ok(equations) => {
            let solver = Solver{concat_enabled: false};
            let sum: usize = equations.iter().map(|e| solver.solve(e)).sum();
            println!("Solvable with (+, *): {sum}");
            let solver = Solver{concat_enabled: true};
            let sum: usize = equations.iter().map(|e| solver.solve(e)).sum();
            println!("Solvable with (+, *, ||): {sum}");
        },
        Err(e) => { println!("Error: {e}"); },
    }
}

struct Equation {
    result: usize,
    numbers: Vec<usize>,
}

impl Equation {
    fn from_str(line: &str) -> Result<Equation, String> {
        let fields: Vec<&str> = line.splitn(2, ":").collect();
        let Some(result) = fields.get(0) else {
            return Err("Missing test value".to_string());
        };
        let Ok(result) = result.parse() else {
            return Err("Missing test value".to_string());
        };
        let Some(numbers) = fields.get(1) else {
            return Err("Missing numbers".to_string());
        };
        let numbers: Vec<&str> = numbers.split(" ").collect();
        let numbers = numbers.iter().skip(1).map(|n| n.parse()).collect::<Result<Vec<usize>,_>>();
        match numbers {
            Err(e) => Err(e.to_string()),
            Ok(numbers) => Ok(Equation{
                result,
                numbers,
            })
        }
    }
}

struct Solver {
    concat_enabled: bool
}

impl Solver {
    fn solve(&self, equation: &Equation) -> usize {
        self.solve_part(equation.numbers[0], &equation.numbers[1..].to_vec(), equation.result)
    }

    fn solve_part(&self, accumulator: usize, rest: &Vec<usize>, expected: usize) -> usize {
        if accumulator > expected {
            return 0;
        }
        if accumulator == expected && rest.is_empty() {
            return expected;
        }
        if rest.is_empty() {
            return 0;
        }
        usize::max(
            usize::max(
                self.solve_part(accumulator + rest[0], &rest[1..].to_vec(), expected),
                self.solve_part(accumulator * rest[0], &rest[1..].to_vec(), expected)
            ),
            self.solve_part(self.concat_operator(accumulator, rest[0]), &rest[1..].to_vec(), expected)
        )
    }

    fn concat_operator(&self, a: usize, b: usize) -> usize {
        if !self.concat_enabled {
            return 0;
        }
        let result = a.to_string() + &b.to_string();
        result.parse().unwrap()
    }
}
