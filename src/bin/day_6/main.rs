use core::panic;
use std::{
    collections::{HashSet, VecDeque},
    fs, isize, usize,
};

#[derive(Debug, Clone)]
struct State {
    grid: Vec<Vec<Tile>>,
    grid_bounds: Point,
    guard: Guard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Obstacle,
    Up,
    Down,
    Left,
    Right,
    Nothing,
}

#[derive(Debug, Clone)]
struct Guard(Point, Direction);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(isize, isize);

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl State {
    pub fn from(contents: &str) -> Self {
        let mut guard = None;
        let grid: Vec<Vec<Tile>> = contents
            .trim()
            .lines()
            .enumerate()
            .map(|(i, x)| {
                x.trim()
                    .char_indices()
                    .map(|(j, x)| {
                        if guard.is_none() && Direction::is_direction(x) {
                            guard = Some(Guard::from(x, i, j));
                        }

                        Tile::from(x)
                    })
                    .collect()
            })
            .collect();

        assert!(guard.is_some());

        let bounds = Point::from(grid.len(), grid[0].len());

        Self {
            grid,
            grid_bounds: bounds,
            guard: guard.unwrap(),
        }
    }

    fn count_touched(&self) -> usize {
        let mut copy = self.clone();
        let mut touched = HashSet::new();

        touched.insert((copy.guard.0 .0, copy.guard.0 .1));
        while copy.move_guard() {
            touched.insert((copy.guard.0 .0, copy.guard.0 .1));
        }

        touched.len()
    }

    fn count_loops(&self) -> usize {
        let mut count = 0;
        for (i, x) in self.grid.iter().enumerate() {
            for j in 0..x.len() {
                let mut copy = self.clone();
                if copy.detect_loop(Point::from(i, j)) {
                    count += 1;
                }
            }
        }

        count
    }

    fn detect_loop(&mut self, extra_obstruction: Point) -> bool {
        if extra_obstruction == self.guard.0 {
            return false;
        }

        let eo = extra_obstruction.unsigned();

        if matches!(self.grid[eo.0][eo.1], Tile::Obstacle) {
            return false;
        }

        self.grid[eo.0][eo.1] = Tile::Obstacle;

        let mut triples: HashSet<Point> = HashSet::new();
        let mut buffer = VecDeque::with_capacity(2);

        while self.move_guard() {
            let current = self.guard.0.unsigned();

            if buffer.len() == 2 && buffer[0] == self.guard.0 && buffer[1] == self.guard.0 {
                if !triples.insert(self.guard.0.clone()) {
                    return true;
                }
            }

            let tile_value = match self.guard.1 {
                Direction::Up => Tile::Up,
                Direction::Down => Tile::Down,
                Direction::Left => Tile::Left,
                Direction::Right => Tile::Right,
            };

            if self.grid[current.0][current.1] == tile_value {
                return true;
            }

            self.grid[current.0][current.1] = tile_value.clone();

            buffer.push_back(self.guard.0);
        }

        return false;
    }

    fn move_guard(&mut self) -> bool {
        let mut next_pos = self.guard.get_next_pos();
        let is_valid = next_pos.inbounds(&self.grid_bounds);

        if !is_valid {
            return false;
        }

        let mut rotate_count = 0;
        while self.has_obstacle(&next_pos) {
            self.guard.rotate();

            if rotate_count >= 3 {
                return true;
            }
            rotate_count += 1;

            next_pos = self.guard.get_next_pos();
        }

        self.guard.0 = next_pos;

        return true;
    }

    fn has_obstacle(&self, next_pos: &Point) -> bool {
        let next_pos = next_pos.unsigned();
        matches!(self.grid[next_pos.0][next_pos.1], Tile::Obstacle)
    }
}

impl Tile {
    pub fn from(c: char) -> Self {
        if c == '#' {
            return Self::Obstacle;
        }
        return Self::Nothing;
    }
}

impl Guard {
    fn from(c: char, i: usize, j: usize) -> Self {
        Self(Point::from(i, j), Direction::from(c))
    }

    fn get_next_pos(&self) -> Point {
        match self.1 {
            Direction::Up => Point(self.0 .0 - 1, self.0 .1),
            Direction::Down => Point(self.0 .0 + 1, self.0 .1),
            Direction::Left => Point(self.0 .0, self.0 .1 - 1),
            Direction::Right => Point(self.0 .0, self.0 .1 + 1),
        }
    }

    fn rotate(&mut self) {
        self.1 = match self.1 {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
    }
}

impl Point {
    fn from(i: usize, j: usize) -> Self {
        let i = i.try_into();
        assert!(i.is_ok());

        let j = j.try_into();
        assert!(j.is_ok());

        Self(i.unwrap(), j.unwrap())
    }

    fn inbounds(&self, bounds: &Point) -> bool {
        self.0 >= 0 && self.1 >= 0 && self.0 < bounds.0 && self.1 < bounds.1
    }

    fn unsigned(&self) -> (usize, usize) {
        assert!(self.0 >= 0);
        assert!(self.1 >= 0);

        (self.0.try_into().unwrap(), self.1.try_into().unwrap())
    }
}

impl Direction {
    fn from(c: char) -> Self {
        assert!(Self::is_direction(c));
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!(""),
        }
    }

    fn is_direction(c: char) -> bool {
        match c {
            '^' | 'v' | '<' | '>' => true,
            _ => false,
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("src/inputs/day6.txt")?;
    let state = State::from(&contents);

    // Part 1
    let count = state.count_touched();
    println!("Part 1: touched count = {}", count);

    // Part 2
    let loop_count = state.count_loops();
    println!("Part 2: loop count = {}", loop_count);

    Ok(())
}
