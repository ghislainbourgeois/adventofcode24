use std::collections::HashSet;

pub struct LabMap {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
    guard: Guard,
    candidates: HashSet<Position>,
    current_candidate: Option<Position>
}

impl LabMap {
    pub fn from_string(input: String) -> Result<LabMap, String> {
        let data: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let height = data.len();
        let width = data[0].len();
        let Some((row, _)) = data.iter().enumerate().filter(|(_, row)| row.contains(&'^')).next() else {
            return Err("Could not find guard's row".to_string());
        };
        let Some((column, _)) = data[row].iter().enumerate().filter(|(_, c)| **c == '^').next() else {
            return Err("Could not find guard's column".to_string());
        };
        
        Ok(LabMap{
            data,
            height,
            width,
            guard: Guard{
                position: Position{row, column},
                direction: Direction::UP
            },
            candidates: HashSet::new(),
            current_candidate: None
        })
    }

    pub fn guard_patrol(&mut self) -> GuardPatrolIterator {
        return GuardPatrolIterator::new(self.guard.clone(), self);
    }

    pub fn count_possible_obstacles(&mut self) -> usize {
        let mut count: usize = 0;
        for candidate in &self.candidates.clone() {
            self.current_candidate = Some(candidate.clone());
            let mut guards: HashSet<Guard> = HashSet::new();
            for guard in self.guard_patrol() {
                if !guards.insert(guard) {
                    count += 1;
                    break;
                }
            }
        }
        count
    }

    fn will_bump_obstacle(&mut self, guard: &Guard) -> bool {
        if self.will_be_outside(guard) {
            return false;
        }
        let new_position = guard.position.move_in_direction(&guard.direction);

        match &self.current_candidate {
            Some(candidate) => {
                if *candidate == new_position {
                    return true
                }

            },
            None => {
                self.candidates.insert(new_position.clone());
            }
        }

        match self.data.get(new_position.row) {
            Some(row) => {
                match row.get(new_position.column) {
                    Some(&'#') => true,
                    _ => false,
                }
            },
            _ => false
        }
    }

    fn will_be_outside(&self, guard: &Guard) -> bool {
        match guard.direction {
            Direction::UP if guard.position.row == 0 => true,
            Direction::RIGHT if guard.position.column == self.width - 1 => true,
            Direction::DOWN if guard.position.row == self.height - 1 => true,
            Direction::LEFT if guard.position.column == 0 => true,
            _ => false,
        }
    }
}

pub struct GuardPatrolIterator<'a> {
    guard: Guard,
    map: &'a mut LabMap,
    init: bool,
}

impl<'a> GuardPatrolIterator<'a> {
    fn new(guard: Guard, map: &'a mut LabMap) -> GuardPatrolIterator<'a> {
        GuardPatrolIterator{guard, map, init: true}
    }
}

impl<'a> Iterator for GuardPatrolIterator<'a> {
    type Item = Guard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.init {
            self.init = false;
            return Some(self.guard.clone());
        } else {
            loop {
                match self.map.will_be_outside(&self.guard) {
                    true => return None,
                    false => match self.map.will_bump_obstacle(&self.guard) {
                        true => {
                            self.guard.turn();
                            continue;
                        },
                        false => {
                            let new_position = self.guard.position.move_in_direction(&self.guard.direction);
                            self.guard.position = new_position.clone();
                            return Some(self.guard.clone());
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Position {
    row: usize,
    column: usize,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Guard {
    position: Position,
    direction: Direction
}

impl Guard {
    fn turn(&mut self) {
        match self.direction {
            Direction::UP => { self.direction = Direction::RIGHT; },
            Direction::RIGHT => { self.direction = Direction::DOWN; },
            Direction::DOWN => { self.direction = Direction::LEFT; },
            Direction::LEFT => { self.direction = Direction::UP; },
        }
    }
}

impl Position {
    fn move_in_direction(&self, direction: &Direction) -> Position {
        match direction {
            Direction::UP => Position{row: self.row - 1, column: self.column}, 
            Direction::RIGHT => Position{row: self.row, column: self.column + 1}, 
            Direction::DOWN => Position{row: self.row + 1, column: self.column}, 
            Direction::LEFT => Position{row: self.row, column: self.column - 1}, 
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}
