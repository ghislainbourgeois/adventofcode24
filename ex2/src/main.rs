use std::fs;

fn main() {
    let mut reports: Vec<Report> = Vec::new();

    let contents = fs::read_to_string("input.txt")
        .expect("Could not read input.txt file");
    
    let mut safe_reports: usize = 0;
    let mut safe_reports_pd: usize = 0;
    for raw_report in contents.lines() {
        let mut report = Report::new();

        for numbers in raw_report.split_ascii_whitespace() {
            let level: isize = numbers.parse().expect("Should be a number");
            report.push(level);
        }

        if report.is_safe() {
            safe_reports += 1;
        }
        if report.is_safe_with_problem_dampener() {
            safe_reports_pd += 1;
        }
        reports.push(report);
    }

    println!("Number of safe reports: {}", safe_reports);
    println!("Number of safe reports with problem dampener: {}", safe_reports_pd);
}

struct Report {
    levels: Vec<isize>,
    differences: Vec<isize>
}

impl Report {
    pub fn new() -> Report {
        Report {
            levels: Vec::new(),
            differences: Vec::new(),
        }
    }

    pub fn push(&mut self, n: isize) {
        let _ = &self.levels.push(n);
        if &self.levels.len() > &1 {
            let _ = &self.differences.push(n - &self.levels[&self.levels.len() - 2]);
        }
    }

    pub fn is_safe(&self) -> bool {
        self.levels_increasing() || self.levels_decreasing()
    }

    fn levels_increasing(&self) -> bool {
        let c = &self.differences.iter().filter(|n| !n.is_positive() || **n > 3).count();
        *c == 0
    }

    fn levels_decreasing(&self) -> bool {
        let c = &self.differences.iter().filter(|n| !n.is_negative() || **n < -3).count();
        *c == 0
    }

    pub fn is_safe_with_problem_dampener(&self) -> bool {
        self.levels_increasing_pd() || self.levels_decreasing_pd()
    }

    fn levels_increasing_pd(&self) -> bool {
        let c = &self.differences.iter().filter(|n| !n.is_positive() || **n > 3).count();
        match *c {
            0 => true,
            1 => {
                for (i, d) in self.differences.iter().enumerate() {
                    if !d.is_positive() || *d > 3 {
                        if i == 0 || i == self.differences.len() - 1 {
                            return true
                        }
                        let n = d + self.differences[i - 1];
                        if 0 < n && n <= 3 {
                            return true
                        }
                        let n = d + self.differences[i + 1];
                        if 0 < n && n <= 3 {
                            return true
                        }
                    }
                }
                false
            },
            2 => {
                for (i, d) in self.differences.iter().enumerate() {
                    if !d.is_positive() || *d > 3 {
                        if i == self.differences.len() - 1 {
                            return false;
                        }
                        let next = self.differences[i + 1];
                        if 0 < next && next <= 3 {
                            return false;
                        }
                        let n = d + next;
                        if 0 < n && n <= 3 {
                            return true;
                        }
                    }
                }
                false
            },
            _ => false,
        }
    }

    fn levels_decreasing_pd(&self) -> bool {
        let c = &self.differences.iter().filter(|n| !n.is_negative() || **n < -3).count();
        match *c {
            0 => true,
            1 => {
                for (i, d) in self.differences.iter().enumerate() {
                    if !d.is_negative() || *d < -3 {
                        if i == 0 || i == self.differences.len() - 1 {
                            return true
                        }
                        let n = d + self.differences[i - 1];
                        if 0 > n && n >= -3 {
                            return true
                        }
                        let n = d + self.differences[i + 1];
                        if 0 > n && n >= -3 {
                            return true
                        }
                    }
                }
                false
            },
            2 => {
                for (i, d) in self.differences.iter().enumerate() {
                    if !d.is_negative() || *d < -3 {
                        if i == self.differences.len() - 1 {
                            return false;
                        }
                        let next = self.differences[i + 1];
                        if 0 > next && next >= -3 {
                            return false;
                        }
                        let n = d + next;
                        if 0 > n && n >= -3 {
                            return true;
                        }
                    }
                }
                false
            },
            _ => false,
        }
    }
}
