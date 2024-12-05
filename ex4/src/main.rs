use std::{char, fs};

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Could not read input.txt file");

    let word_search = parse_input(contents);

    println!("Word count: {}", count_xmas(&word_search));

    println!("X-Word count: {}", count_x_mas(&word_search));
}

fn parse_input(contents: String) -> Vec<Vec<char>> {
    let mut word_search: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        let mut columns: Vec<char> = Vec::new();
        for char in line.chars() {
            columns.push(char);
        }
        word_search.push(columns);
    }
    word_search
}

fn count_xmas(word_search: &Vec<Vec<char>>) -> usize {
    let mut wc: usize = 0;
    for i in 0..(word_search.len()) {
        for j in 0..(word_search[i].len()) {
            if word_search[i][j] != 'X' {
                continue;
            }
            if i >= 3 {
                // Can search up
                if word_search[i-1][j] == 'M' && word_search[i-2][j] == 'A' && word_search[i-3][j] == 'S' {
                    wc += 1;
                }

                if j >= 3 {
                    // Can search up-left
                    if word_search[i-1][j-1] == 'M' && word_search[i-2][j-2] == 'A' && word_search[i-3][j-3] == 'S' {
                        wc += 1;
                    }
                }

                if j <= word_search[i].len() - 4 {
                    // Can search up-right
                    if word_search[i-1][j+1] == 'M' && word_search[i-2][j+2] == 'A' && word_search[i-3][j+3] == 'S' {
                        wc += 1;
                    }
                }

            }

            if j >= 3 {
                // Can search left
                if word_search[i][j-1] == 'M' && word_search[i][j-2] == 'A' && word_search[i][j-3] == 'S' {
                    wc += 1;
                }
            }

            if j <= word_search[i].len() - 4 {
                // Can search right
                if word_search[i][j+1] == 'M' && word_search[i][j+2] == 'A' && word_search[i][j+3] == 'S' {
                    wc += 1;
                }
            }

            if i <= word_search.len() - 4 {
                // Can search down
                if word_search[i+1][j] == 'M' && word_search[i+2][j] == 'A' && word_search[i+3][j] == 'S' {
                    wc += 1;
                }

                if j >= 3 {
                    // Can search down-left
                    if word_search[i+1][j-1] == 'M' && word_search[i+2][j-2] == 'A' && word_search[i+3][j-3] == 'S' {
                        wc += 1;
                    }
                }

                if j <= word_search[i].len() - 4 {
                    // Can search down-right
                    if word_search[i+1][j+1] == 'M' && word_search[i+2][j+2] == 'A' && word_search[i+3][j+3] == 'S' {
                        wc += 1;
                    }
                }

            }
        }
    }
    wc
}

fn count_x_mas(word_search: &Vec<Vec<char>>) -> usize {
    let mut xc: usize = 0;
    for i in 0..(word_search.len()) {
        for j in 0..(word_search[i].len()) {
            if word_search[i][j] != 'M' {
                continue;
            }
            if i >= 2 && j <= word_search[i].len() - 3 {
                // Can search up
                if word_search[i-1][j+1] == 'A' && word_search[i][j+2] == 'M' && word_search[i-2][j] == 'S' && word_search[i-2][j+2] == 'S' {
                    xc += 1;
                }
            }

            if i <= word_search.len() - 3 {
                // Can search down
                if j >= 2 {
                    // 2-4
                    if word_search[i+1][j-1] == 'A' && word_search[i+2][j] == 'M' && word_search[i][j-2] == 'S' && word_search[i+2][j-2] == 'S' {
                        xc += 1;
                    }
                }

                if j <= word_search[i].len() - 3 {
                    // 1-2 or 1-3
                    if word_search[i+1][j+1] == 'A' && word_search[i][j+2] == 'M' && word_search[i+2][j] == 'S' && word_search[i+2][j+2] == 'S' {
                        xc += 1;
                    }

                    if word_search[i+1][j+1] == 'A' && word_search[i+2][j] == 'M' && word_search[i][j+2] == 'S' && word_search[i+2][j+2] == 'S' {
                        xc += 1;
                    }
                }
            }
        }
    }
    xc
}
