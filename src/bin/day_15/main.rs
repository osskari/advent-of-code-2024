use core::panic;
use std::time;

#[derive(Debug)]
struct State {
    robot: Point,
    grid: Vec<Vec<char>>,
    instructions: Vec<Direction>,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Alignment {
    Vertical,
    Horizontal,
}

impl State {
    fn parse(contents: &str) -> Self {
        let mut lines = contents.trim().lines();
        let mut robot = None;

        let grid = lines
            .by_ref()
            .take_while(|x| !x.is_empty())
            .enumerate()
            .map(|(i, x)| {
                x.char_indices()
                    .inspect(|(j, x)| {
                        if *x == '@' {
                            assert!(robot.is_none());
                            robot = Some(Point::new(i, *j));
                        }
                    })
                    .map(|(_, x)| x)
                    .collect()
            })
            .collect();

        let instructions = lines
            .skip_while(|x| x.is_empty())
            .flat_map(|x| x.chars().map(|x| Direction::parse(x)).collect::<Vec<_>>())
            .collect();

        assert!(robot.is_some());

        Self {
            robot: robot.unwrap(),
            grid,
            instructions,
        }
    }

    fn run_instructions(&mut self) {
        for i in 0..self.instructions.len() {
            self.do_move_at(i);
        }
    }

    fn do_move_at(&mut self, index: usize) {
        let instruction = &self.instructions[index];

        let range: Vec<_> = match instruction {
            Direction::Up => (0..self.robot.x).rev().collect(),
            Direction::Down => (self.robot.x + 1..self.grid.len()).collect(),
            Direction::Left => (0..self.robot.y).rev().collect(),
            Direction::Right => (self.robot.y + 1..self.grid[self.robot.y].len()).collect(),
        };

        let mut dot = None;

        for i in 0..range.len() {
            let next = self.robot.nth_in_range(i, &range, instruction);
            if self.grid[next.x][next.y] == '#' {
                break;
            } else if self.grid[next.x][next.y] == '.' {
                dot = Some(i);
                break;
            }
        }

        let dot = match dot {
            Some(dot) => dot,
            None => return,
        };

        for i in 1..=dot {
            let replace = self.robot.nth_in_range(i, &range, instruction);
            self.grid[replace.x][replace.y] = 'O';
        }

        self.grid[self.robot.x][self.robot.y] = '.';

        self.robot.do_move(instruction);
        self.grid[self.robot.x][self.robot.y] = '@';
    }

    fn gps_sum(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, x)| {
                x.iter()
                    .enumerate()
                    .filter(|(_, x)| **x == 'O')
                    .map(|(j, _)| (100 * i) + j)
                    .sum::<usize>()
            })
            .sum()
    }
}

impl Point {
    fn new(i: usize, j: usize) -> Self {
        Self { x: i, y: j }
    }

    fn do_move(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                assert!(self.x != 0);
                self.x -= 1;
            }
            Direction::Down => {
                self.x += 1;
            }
            Direction::Left => {
                assert!(self.y != 0);
                self.y -= 1;
            }
            Direction::Right => {
                self.y += 1;
            }
        }
    }

    fn nth_in_range(&self, n: usize, range: &Vec<usize>, direction: &Direction) -> Self {
        match direction.alignment() {
            Alignment::Vertical => Self::new(range[n], self.y),
            Alignment::Horizontal => Self::new(self.x, range[n]),
        }
    }
}

impl Direction {
    fn parse(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("BAD DIRECTION"),
        }
    }

    fn alignment(&self) -> Alignment {
        match self {
            Direction::Up | Direction::Down => Alignment::Vertical,
            Direction::Left | Direction::Right => Alignment::Horizontal,
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let contents = std::fs::read_to_string("src/inputs/day15.txt")?;

    let mut state = State::parse(&contents);

    let start = time::Instant::now();
    state.run_instructions();
    println!(
        "Part 1: sum = {}, completed in = {:?}\n",
        state.gps_sum(),
        start.elapsed()
    );

    Ok(())
}
