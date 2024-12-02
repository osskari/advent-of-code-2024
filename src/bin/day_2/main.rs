use std::{fs, io, usize};

#[derive(Debug)]
struct Row(Vec<u32>);

impl Row {
    pub fn parse_lines(contents: String) -> Vec<Self> {
        return contents.trim().lines().map(|x| Row::parse_row(x)).collect();
    }

    fn parse_row(row: &str) -> Self {
        let items = row
            .trim()
            .split(' ')
            .map(|x| {
                let temp = x.parse();
                assert!(temp.is_ok());
                return temp.unwrap();
            })
            .collect();

        return Self(items);
    }

    pub fn is_safe(&self) -> bool {
        assert!(self.0.len() >= 2);
        let direction = Direction::parse(self.0[0], self.0[1]);

        for x in self.0.windows(2) {
            assert_eq!(x.len(), 2);

            let diff = x[0] as i32 - x[1] as i32;
            let local_direction = Direction::new(diff);

            if !direction.matches(&local_direction) {
                return false;
            }

            let diff = diff.abs();

            if diff < 1 || diff > 3 {
                return false;
            }
        }

        return true;
    }

    pub fn is_safe_dampened(&self) -> bool {
        assert!(self.0.len() >= 2);

        if self.is_safe() {
            return true;
        }

        for index in 0..self.0.len() {
            let one_removed: Vec<_> = self
                .0
                .iter()
                .enumerate()
                .filter(|&(i, _)| i as usize != index)
                .map(|(_, x)| x.clone())
                .collect();

            if Self(one_removed).is_safe() {
                return true;
            }
        }

        return false;
    }
}

#[derive(Debug)]
enum Direction {
    Ascending,
    Descending,
    Neutral,
}

impl Direction {
    pub fn new(value: i32) -> Self {
        match value {
            b if b > 0 => Self::Descending,
            b if b < 0 => Self::Ascending,
            _ => Self::Neutral,
        }
    }

    pub fn parse(lhs: u32, rhs: u32) -> Self {
        let diff = lhs as i32 - rhs as i32;
        return Self::new(diff);
    }

    pub fn matches(&self, other: &Self) -> bool {
        return matches!(
            (self, other),
            (Self::Ascending, Self::Ascending)
                | (Self::Descending, Self::Descending)
                | (Self::Neutral, Self::Neutral)
        );
    }
}

fn main() -> Result<(), io::Error> {
    let contents = fs::read_to_string("src/inputs/day2.txt")?;

    let rows = Row::parse_lines(contents);

    // Part 1
    let safe_count = rows.iter().filter(|x| x.is_safe()).count();
    println!("Part 1:\nsafe count = {}\n", safe_count);

    // Part 2
    let safe_count = rows.iter().filter(|x| x.is_safe_dampened()).count();
    println!("Part 2:\ndampened safe count = {}", safe_count);

    return Ok(());
}
