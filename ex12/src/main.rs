use std::{collections::HashSet, fs};

fn main() {
    let map: Vec<Vec<char>> = fs::read_to_string("input.txt").expect("Should be able to read input")
        .lines().map(|l| l.chars().collect()).collect();

    let mut regions: Vec<Region> = Vec::new();

    let mut visited_plots: Vec<(usize, usize)> = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, _c) in row.iter().enumerate() {
            if visited_plots.contains(&(i, j)) {
                continue;
            }
            visited_plots.push((i, j));
            let region = Region::from_plot_and_map(i, j, &map);
            visited_plots.append(&mut region.plots.clone());
            regions.push(region);
        }
    }

    let total_price: usize = regions.iter().map(|r| r.price()).sum();
    println!("Total price: {total_price}");

    let discounted_price: usize = regions.iter().map(|r| r.discounted_price()).sum();
    println!("Discounted price: {discounted_price}");
}

#[derive(Debug)]
struct Region {
    plots: Vec<(usize, usize)>,
    perimeter: usize
}

impl Region {
    fn from_plot_and_map(i: usize, j: usize, map: &Vec<Vec<char>>) -> Self {
        let visited_plots: Vec<(usize, usize)> = Vec::new();
        let mut region = Self{plots: visited_plots, perimeter: 0};
        region.find_plots_in_region(i, j, map);
        region
    }

    fn find_plots_in_region(&mut self, i: usize, j: usize, map: &Vec<Vec<char>>) {
        self.plots.push((i, j));
        if i == 0 {
            self.perimeter += 1;
        } else {
            if map[i - 1][j] == map[i][j] {
                if !self.plots.contains(&(i - 1, j)) {
                    self.find_plots_in_region(i - 1, j, map);
                }
            } else {
                self.perimeter += 1;
            }
        }
        if j == 0 {
            self.perimeter += 1;
        } else {
            if map[i][j - 1] == map[i][j] {
                if !self.plots.contains(&(i, j - 1)) {
                    self.find_plots_in_region(i, j - 1, map);
                }
            } else {
                self.perimeter += 1;
            }
        }
        if i == map.len() - 1 {
            self.perimeter += 1;
        } else {
            if map[i + 1][j] == map[i][j] {
                if !self.plots.contains(&(i + 1, j)) {
                    self.find_plots_in_region(i + 1, j, map);
                }
            } else {
                self.perimeter += 1;
            }
        }
        if j == map[0].len() - 1 {
            self.perimeter += 1;
        } else {
            if map[i][j + 1] == map[i][j] {
                if !self.plots.contains(&(i, j + 1)) {
                    self.find_plots_in_region(i, j + 1, map);
                }
            } else {
                self.perimeter += 1;
            }
        }
    }

    fn price(&self) -> usize {
        self.perimeter * self.plots.len()
    }

    fn discounted_price(&self) -> usize {
        self.count_sides() * self.plots.len()
    }

    fn count_sides(&self) -> usize {
        let mut sides: usize = 0;
        let rows: HashSet<usize> = self.plots.iter().map(|p| p.0).collect();
        let columns : HashSet<usize> = self.plots.iter().map(|p| p.1).collect();
        for row in &rows {
            let mut north_sides: Vec<usize> = self.plots.iter().filter(|p| p.0 == *row).map(|p| p.1).collect();
            north_sides.sort();
            let mut south_sides = north_sides.clone();
            if *row != 0 {
                let above: Vec<usize> = self.plots.iter().filter(|p| p.0 == *row - 1).map(|p| p.1).collect();
                north_sides = north_sides.into_iter().filter(|p| !above.contains(p)).collect();
            }
            if *row != *rows.iter().max().unwrap() {
                let below: HashSet<usize> = self.plots.iter().filter(|p| p.0 == *row + 1).map(|p| p.1).collect();
                south_sides = south_sides.into_iter().filter(|p| !below.contains(p)).collect();
            }
            sides += Self::count_split_sides(&north_sides);
            sides += Self::count_split_sides(&south_sides);
        }
        for col in &columns {
            let mut west_sides: Vec<usize> = self.plots.iter().filter(|p| p.1 == *col).map(|p| p.0).collect();
            west_sides.sort();
            let mut east_sides = west_sides.clone();
            if *col != 0 {
                let left: Vec<usize> = self.plots.iter().filter(|p| p.1 == *col - 1).map(|p| p.0).collect();
                west_sides = west_sides.into_iter().filter(|p| !left.contains(p)).collect();
            }
            if *col != *columns.iter().max().unwrap() {
                let right: HashSet<usize> = self.plots.iter().filter(|p| p.1 == *col + 1).map(|p| p.0).collect();
                east_sides = east_sides.into_iter().filter(|p| !right.contains(p)).collect();
            }
            sides += Self::count_split_sides(&west_sides);
            sides += Self::count_split_sides(&east_sides);
        }
        sides
    }

    fn count_split_sides(indices: &Vec<usize>) -> usize {
        match indices.len() {
            0 => 0,
            1 => 1,
            _ => {
                let mut sides: usize = 1;
                for i in 1..indices.len() {
                    if indices[i] - indices[i - 1] > 1 {
                        sides += 1;
                    }
                }
                sides
            }
        }
    }
}
