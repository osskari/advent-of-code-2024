use std::{char, fs, io, ops::Range};

struct Bounds {
    up: usize,
    down: usize,
    left: usize,
    right: usize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

struct Grid(Vec<Vec<char>>);

struct Pivot {
    before: String,
    middle: char,
    after: String,
}

impl Bounds {
    fn parse(grid: &Grid, i: usize, j: usize) -> Self {
        Self {
            up: i,
            down: grid.len() - 1 - i,
            left: j,
            right: grid.at(i).len() - 1 - j,
        }
    }

    fn does_fit(&self, direction: Direction, len: usize) -> bool {
        match direction {
            Direction::Up => self.up >= len,
            Direction::Down => self.down >= len,
            Direction::Left => self.left >= len,
            Direction::Right => self.right >= len,
            Direction::UpLeft => self.up >= len && self.left >= len,
            Direction::UpRight => self.up >= len && self.right >= len,
            Direction::DownLeft => self.down >= len && self.left >= len,
            Direction::DownRight => self.down >= len && self.right >= len,
        }
    }
}

impl Grid {
    fn parse(contents: &str) -> Self {
        let grid: Vec<Vec<_>> = contents
            .trim()
            .lines()
            .map(|x| x.trim().chars().collect())
            .collect();

        assert!(grid.len() > 1);

        return Self(grid);
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn at(&self, i: usize) -> &Vec<char> {
        &self.0[i]
    }

    fn range(&self, range: Range<usize>) -> &[Vec<char>] {
        &self.0[range]
    }

    fn enumerate_grid<F>(&self, pivot: char, func: F) -> usize
    where
        F: Fn(usize, usize) -> usize,
    {
        let mut count = 0;
        for i in 0..self.len() {
            for j in 0..self.at(i).len() {
                if self.at(i)[j] == pivot {
                    count += func(i, j);
                }
            }
        }
        return count;
    }
}

impl Pivot {
    fn parse(string: &str) -> Self {
        assert!(string.len() > 0);
        assert!(string.len() % 2 == 1);

        let middle = string.chars().count() / 2;
        let mut chars = string.chars();

        let before = chars.by_ref().take(middle).collect();
        let middle = chars.next().unwrap();
        let after = chars.collect();

        Self {
            before,
            middle,
            after,
        }
    }

    fn len(&self) -> usize {
        assert_eq!(self.before.len(), self.after.len());

        return self.before.len();
    }

    fn matches_bidirectional(&self, other: &str) -> bool {
        let combined = format!("{}{}{}", self.before, self.middle, self.after);

        other == combined || other == combined.chars().rev().collect::<String>()
    }
}

fn xmas_scan(grid: &Grid) -> usize {
    let pivot = Pivot::parse("SAMXMAS");

    return grid.enumerate_grid(pivot.middle, |i, j| search_around(&grid, &pivot, i, j));
}

fn crossmas_scan(grid: &Grid) -> usize {
    let pivot = Pivot::parse("MAS");

    return grid.enumerate_grid(pivot.middle, |i, j| cross_search(&grid, &pivot, i, j));
}

fn cross_search(grid: &Grid, pivot: &Pivot, i: usize, j: usize) -> usize {
    assert!(grid.at(i)[j] == pivot.middle);

    let mut count = 0;
    let bounds = Bounds::parse(grid, i, j);

    if bounds.does_fit(Direction::UpLeft, pivot.len())
        && bounds.does_fit(Direction::DownRight, pivot.len())
    {
        let line = grid
            .range(i - pivot.len()..i + pivot.len() + 1)
            .iter()
            .enumerate()
            .map(|(ii, x)| x[j - pivot.len() + ii])
            .collect::<String>();

        if pivot.matches_bidirectional(&line) {
            count += 1;
        }
    }

    if bounds.does_fit(Direction::DownLeft, pivot.len())
        && bounds.does_fit(Direction::UpRight, pivot.len())
    {
        let line = grid
            .range(i - pivot.len()..i + pivot.len() + 1)
            .iter()
            .enumerate()
            .map(|(ii, x)| x[j + pivot.len() - ii])
            .collect::<String>();

        if pivot.matches_bidirectional(&line) {
            count += 1;
        }
    }

    if count >= 2 {
        1
    } else {
        0
    }
}

fn search_around(grid: &Grid, pivot: &Pivot, i: usize, j: usize) -> usize {
    assert!(grid.at(i)[j] == pivot.middle);

    let mut count = 0;
    let bounds = Bounds::parse(grid, i, j);

    if bounds.does_fit(Direction::Up, pivot.len()) {
        let rest: String = grid
            .range(i - pivot.len()..i)
            .iter()
            .map(|x| x[j])
            .collect();
        if rest == pivot.before {
            count += 1;
        }
    }

    if bounds.does_fit(Direction::Down, pivot.len()) {
        let rest: String = grid
            .range(i + 1..i + pivot.len() + 1)
            .iter()
            .map(|x| x[j])
            .collect();
        if rest == pivot.after {
            count += 1;
        }
    }

    if bounds.does_fit(Direction::Left, pivot.len()) {
        let rest: String = grid.at(i)[j - pivot.len()..j].iter().collect();
        if rest == pivot.before {
            count += 1;
        }
    }

    if bounds.does_fit(Direction::Right, pivot.len()) {
        let rest: String = grid.at(i)[j + 1..j + pivot.len() + 1].iter().collect();
        if rest == pivot.after {
            count += 1;
        }
    }

    if bounds.does_fit(Direction::UpLeft, pivot.len()) {
        let rest: String = grid
            .range(i - pivot.len()..i)
            .iter()
            .enumerate()
            .map(|(ii, x)| x[j - pivot.len() + ii])
            .collect();
        if rest == pivot.before {
            count += 1;
        }
    }

    if bounds.does_fit(Direction::UpRight, pivot.len()) {
        let rest: String = grid
            .range(i - pivot.len()..i)
            .iter()
            .enumerate()
            .map(|(ii, x)| x[j + pivot.len() - ii])
            .collect();
        if rest == pivot.before {
            count += 1;
        }
    }

    if bounds.does_fit(Direction::DownLeft, pivot.len()) {
        let rest: String = grid
            .range(i + 1..i + pivot.len() + 1)
            .iter()
            .enumerate()
            .map(|(ii, x)| x[j - 1 - ii])
            .collect();
        if rest == pivot.after {
            count += 1;
        }
    }

    if bounds.does_fit(Direction::DownRight, pivot.len()) {
        let rest: String = grid
            .range(i + 1..i + pivot.len() + 1)
            .iter()
            .enumerate()
            .map(|(ii, x)| x[j + ii + 1])
            .collect();
        if rest == pivot.after {
            count += 1;
        }
    }

    return count;
}

fn main() -> Result<(), io::Error> {
    let contents = fs::read_to_string("src/inputs/day4.txt")?;
    let grid = Grid::parse(&contents);

    // Part 1
    let xmas_count = xmas_scan(&grid);
    println!("Part 1: count = {}\n", xmas_count);

    let crossmas_count = crossmas_scan(&grid);
    println!("Part 2: count = {}\n", crossmas_count);

    Ok(())
}
