use std::{fs, io};

#[derive(Debug)]
struct Pair(u32, u32);

impl Pair {
    pub fn new(left: u32, right: u32) -> Self {
        Self(left, right)
    }

    fn distance(&self) -> u32 {
        return (self.0 as i32 - self.1 as i32).abs() as u32;
    }

    fn total_distance(pairs: Vec<Self>) -> u32 {
        return pairs.iter().map(|x| x.distance()).sum();
    }
}

#[derive(Debug)]
struct SortedInput(Vec<u32>, Vec<u32>);

impl SortedInput {
    pub fn parse(contents: String) -> Self {
        let contents = contents.trim().lines();

        let mut left: Vec<u32> = vec![];
        let mut right: Vec<u32> = vec![];
        for line in contents {
            let items: Vec<_> = line.split(' ').filter(|&x| !x.is_empty()).collect();
            assert_eq!(items.len(), 2);

            let temp = items[0].parse();
            assert!(temp.is_ok());
            left.push(temp.unwrap());

            let temp = items[1].parse();
            assert!(temp.is_ok());
            right.push(temp.unwrap());
        }

        return Self::new(left, right);
    }

    pub fn new(mut left: Vec<u32>, mut right: Vec<u32>) -> Self {
        assert_eq!(left.len(), right.len());

        left.sort();
        right.sort();

        Self(left, right)
    }

    pub fn get_pairs(&self) -> Vec<Pair> {
        assert_eq!(self.0.len(), self.1.len());

        return self
            .0
            .iter()
            .zip(&self.1)
            .map(|(left, right)| Pair::new(left.clone(), right.clone()))
            .collect();
    }

    pub fn similarity_score(&self) -> u32 {
        let mut score_total = 0;
        let mut right_i = 0;

        for x in self.0.iter() {
            let (score, i) = Self::score_item(x, &self.1[right_i..]);

            score_total += x * score;
            right_i += i as usize;
        }

        return score_total;
    }

    fn score_item(value: &u32, right: &[u32]) -> (u32, u32) {
        let mut counter = 0;
        let mut i = 0;

        for x in right.iter() {
            if x > value {
                return (counter, i);
            } else if x == value {
                counter += 1;
            }

            i += 1;
        }

        return (counter, i);
    }
}

fn main() -> Result<(), io::Error> {
    let contents = fs::read_to_string("src/inputs/day1.txt")?;

    let sorted = SortedInput::parse(contents);

    // Part 1
    let pairs = sorted.get_pairs();
    let sum = Pair::total_distance(pairs);

    println!("Part 1:\ntotal distance = {}\n", sum);

    // Part 2
    let similarity_score = sorted.similarity_score();

    println!("Part 2:\nsimilarity score = {}", similarity_score);

    return Ok(());
}
