use std::{isize, time, usize};

use itertools::Itertools;

#[derive(Debug)]
struct Board {
    bounds: Point,
    robots: Vec<Robot>,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Vector {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Robot {
    position: Point,
    velocity: Vector,
}

impl Board {
    fn parse(contents: &str, bounds: Point) -> Self {
        Self {
            bounds,
            robots: contents.trim().lines().map(|x| Robot::parse(x)).collect(),
        }
    }

    fn safety_factor(&self) -> usize {
        let middle = Point::new(self.bounds.x / 2, self.bounds.y / 2);

        let mut counts = (0, 0, 0, 0);
        self.robots
            .iter()
            .filter(|x| x.position.x != middle.x && x.position.y != middle.y)
            .for_each(
                |x| match (x.position.x > middle.x, x.position.y > middle.y) {
                    (true, true) => counts.3 += 1,
                    (true, false) => counts.2 += 1,
                    (false, true) => counts.1 += 1,
                    (false, false) => counts.0 += 1,
                },
            );

        counts.0 * counts.1 * counts.2 * counts.3
    }

    fn move_n_times(&mut self, n: usize) {
        for x in self.robots.iter_mut() {
            x.move_n_times(n, &self.bounds);
        }
    }
}

impl Point {
    fn new(i: usize, j: usize) -> Self {
        Self { x: i, y: j }
    }

    fn parse(string: &str) -> Self {
        let digits = string.trim().chars().skip(2).collect::<String>();
        let digits = digits.split(',').collect::<Vec<_>>();

        assert!(digits.len() == 2);

        let x = digits[1].parse();
        let y = digits[0].parse();

        assert!(x.is_ok());
        assert!(y.is_ok());

        Self::new(x.unwrap(), y.unwrap())
    }

    fn move_by(&self, movement: &Vector, bounds: &Point) -> Self {
        let x = self.x as isize + movement.x;
        let y = self.y as isize + movement.y;

        let x = if x.is_negative() {
            bounds.x as isize + x
        } else if x as usize >= bounds.x {
            x - bounds.x as isize
        } else {
            x
        };

        let y = if y.is_negative() {
            bounds.y as isize + y
        } else if y as usize >= bounds.y {
            y - bounds.y as isize
        } else {
            y
        };

        Self {
            x: x as usize,
            y: y as usize,
        }
    }
}

impl Vector {
    fn new(i: isize, j: isize) -> Self {
        Self { x: i, y: j }
    }

    fn parse(string: &str) -> Self {
        let string = string.trim().chars().skip(2).collect::<String>();
        let digits = string.split(',').collect::<Vec<_>>();

        assert!(digits.len() == 2);

        let x = digits[1].parse();
        let y = digits[0].parse();

        assert!(x.is_ok());
        assert!(y.is_ok());

        Self::new(x.unwrap(), y.unwrap())
    }
}

impl Robot {
    fn parse(line: &str) -> Self {
        let mut robots = line
            .trim()
            .split_whitespace()
            .tuples()
            .map(|x| Self::parse_tuple(x));

        let robot = robots.next();

        assert!(robot.is_some());
        assert!(robots.count() == 0);

        robot.unwrap()
    }

    fn parse_tuple(tuple: (&str, &str)) -> Self {
        Self {
            position: Point::parse(tuple.0),
            velocity: Vector::parse(tuple.1),
        }
    }

    fn move_n_times(&mut self, count: usize, bounds: &Point) {
        for _ in 0..count {
            self.position = self.position.move_by(&self.velocity, &bounds);
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let contents = std::fs::read_to_string("src/inputs/day14.txt")?;

    let bounds = Point::new(103, 101);
    let mut board = Board::parse(&contents, bounds);

    // Part 1
    let start = time::Instant::now();
    board.move_n_times(100);
    println!(
        "Part 1: safety = {}, completed in = {:?}\n",
        board.safety_factor(),
        start.elapsed()
    );

    Ok(())
}
