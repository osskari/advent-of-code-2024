use core::panic;
use std::{
    sync::{atomic::AtomicUsize, Arc},
    time, usize,
};

use ahash::AHashSet;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

struct Grid {
    grid: Vec<Vec<Tile>>,
    bounds: Point,
}

#[derive(Clone, Copy)]
enum Tile {
    Obstacle,
    Nothing,
}

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
struct Point(usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct State(Point, Direction);

impl Grid {
    pub fn from(contents: &str) -> Self {
        let grid: Vec<Vec<Tile>> = contents
            .trim()
            .lines()
            .map(|x| x.trim().chars().map(move |x| Tile::from(x)).collect())
            .collect();

        let bounds = Point(grid.len(), grid[0].len());
        Self { grid, bounds }
    }

    fn with(&self, obstacle: Point) -> Self {
        let mut g = self.grid.clone();
        g[obstacle.0][obstacle.1] = Tile::Obstacle;
        Self {
            grid: g,
            bounds: self.bounds,
        }
    }

    pub fn move_until_oob<F>(&self, state: State, mut func: F)
    where
        F: FnMut(&State) -> bool,
    {
        let mut direction = state.1;
        let mut state = state;

        if func(&state) {
            return;
        }

        loop {
            let next = match state.get_next(self.bounds, direction) {
                Some(s) => s,
                None => {
                    return;
                }
            };

            if self.is_tile_obstacle(next.0 .0, next.0 .1) {
                direction = next.rotate();
                continue;
            } else {
                state = next;
                if func(&state) {
                    return;
                }
            }
        }
    }

    fn is_tile_obstacle(&self, i: usize, j: usize) -> bool {
        match self.grid[i][j] {
            Tile::Obstacle => true,
            Tile::Nothing => false,
        }
    }

    fn count_touched_tiles(&self, state: State) -> usize {
        let mut touched: AHashSet<Point> = AHashSet::new();

        self.move_until_oob(state, |s| {
            touched.insert(s.0);
            false
        });

        return touched.len();
    }

    fn count_all_loops(&self, state: State) -> usize {
        let loops = Arc::new(AtomicUsize::new(0));

        let placements: Vec<_> = (0..self.bounds.0)
            .cartesian_product(0..self.bounds.1)
            .filter(|(i, j)| *i != state.0 .0 && *j != state.0 .1 || !self.is_tile_obstacle(*i, *j))
            .collect();

        placements.into_par_iter().for_each(|(i, j)| {
            let point = Point(i, j);
            let grid = self.with(point);

            let mut touched: AHashSet<State> = AHashSet::with_capacity(3000);

            grid.move_until_oob(state, |s| {
                if !touched.insert(s.clone()) {
                    loops.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    return true;
                }
                return false;
            });
        });

        return loops.load(std::sync::atomic::Ordering::SeqCst);
    }
}

impl Tile {
    pub fn from(c: char) -> Self {
        if c == '#' {
            Self::Obstacle
        } else {
            Self::Nothing
        }
    }
}

impl State {
    pub fn from(contents: &str) -> Self {
        for (i, x) in contents.trim().lines().enumerate() {
            for (j, x) in x.char_indices() {
                let direction = match x {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => continue,
                };
                return State(Point(i, j), direction);
            }
        }
        panic!("No guard in input");
    }

    pub fn get_next(&self, bounds: Point, direction: Direction) -> Option<Self> {
        let next = match direction {
            Direction::Up => (self.0 .0.wrapping_sub(1), self.0 .1),
            Direction::Down => (self.0 .0.wrapping_add(1), self.0 .1),
            Direction::Left => (self.0 .0, self.0 .1.wrapping_sub(1)),
            Direction::Right => (self.0 .0, self.0 .1.wrapping_add(1)),
        };

        if next.0 == usize::MAX || next.0 == bounds.0 || next.1 == usize::MAX || next.1 == bounds.1
        {
            return None;
        }

        return Some(State(Point(next.0, next.1), direction));
    }

    fn rotate(&self) -> Direction {
        match self.1 {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let contents = std::fs::read_to_string("src/inputs/day6.txt")?;
    let state = State::from(&contents);
    let grid = Grid::from(&contents);

    let start = time::Instant::now();
    let count = grid.count_touched_tiles(state);
    println!(
        "Part 1: count = {}, completed in: {:?}\n",
        count,
        start.elapsed()
    );

    let start = time::Instant::now();
    let count = grid.count_all_loops(state);
    println!(
        "Part 2: count = {}, completed in: {:?}\n",
        count,
        start.elapsed()
    );

    Ok(())
}
