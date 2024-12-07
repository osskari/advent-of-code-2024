use std::{fs, time, usize};

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

struct Equation {
    solution: usize,
    numbers: Vec<usize>,
}

#[derive(Clone, Copy)]
enum Operator {
    Plus,
    Mult,
    Concat,
}

impl Operator {
    pub fn calculate(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Operator::Plus => lhs + rhs,
            Operator::Mult => lhs * rhs,
            Operator::Concat => {
                let concated = format!("{}{}", lhs, rhs).parse();
                assert!(concated.is_ok());
                concated.unwrap()
            }
        }
    }
}

impl Equation {
    pub fn parse(contents: &str) -> Vec<Self> {
        contents.trim().lines().map(|x| Equation::from(x)).collect()
    }

    fn from(line: &str) -> Self {
        let mut split = line.trim().split(':');

        Self {
            solution: Self::parse_solution(split.next()),
            numbers: Self::parse_numbers(split.collect()),
        }
    }

    fn parse_solution(split: Option<&str>) -> usize {
        assert!(split.is_some());
        let solution = split.unwrap().parse();
        assert!(solution.is_ok());
        solution.unwrap()
    }

    fn parse_numbers(string: String) -> Vec<usize> {
        string
            .trim()
            .split_whitespace()
            .map(|x| {
                let temp = x.parse();
                assert!(temp.is_ok());
                temp.unwrap()
            })
            .collect()
    }

    fn find_solution(&self, operators: &Vec<Operator>) -> usize {
        let windows: Vec<_> = self.numbers.windows(2).collect();

        let combinations: Vec<Vec<_>> = (0..windows.len())
            .map(|_| operators.iter().cloned().collect::<Vec<_>>())
            .multi_cartesian_product()
            .collect();

        for combo in combinations {
            let mut sum = windows[0][0];
            for (i, x) in combo.iter().enumerate() {
                sum = x.calculate(sum, windows[i][1]);
            }

            if sum == self.solution {
                return sum;
            }
        }

        0
    }
}

fn total_calibration_result(equations: &Vec<Equation>) -> usize {
    let ops = vec![Operator::Plus, Operator::Mult];
    equations
        .into_par_iter()
        .map(|x| x.find_solution(&ops))
        .sum()
}

fn total_calibration_result2(equations: &Vec<Equation>) -> usize {
    let ops = vec![Operator::Plus, Operator::Mult, Operator::Concat];
    equations
        .into_par_iter()
        .map(|x| x.find_solution(&ops))
        .sum()
}

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("src/inputs/day7.txt")?;
    let equations = Equation::parse(&contents);

    let start = time::Instant::now();
    let sum: usize = total_calibration_result(&equations);
    println!(
        "Part 1: count = {}, completed in: {:?}\n",
        sum,
        start.elapsed()
    );

    let start = time::Instant::now();
    let sum: usize = total_calibration_result2(&equations);
    println!(
        "Part 2: count = {}, completed in: {:?}\n",
        sum,
        start.elapsed()
    );

    Ok(())
}
