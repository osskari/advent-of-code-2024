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

    pub fn new(left: Vec<u32>, right: Vec<u32>) -> Self {
        assert_eq!(left.len(), right.len());
        Self(left, right)
    }

    pub fn get_pairs(mut self) -> Vec<Pair> {
        assert_eq!(self.0.len(), self.1.len());
        self.0.sort();
        self.1.sort();

        return self
            .0
            .iter()
            .zip(self.1)
            .map(|(left, right)| Pair::new(left.clone(), right))
            .collect();
    }
}

fn main() -> Result<(), io::Error> {
    let contents = fs::read_to_string("src/inputs/day1.txt")?;

    let pairs = SortedInput::parse(contents).get_pairs();
    let sum = Pair::total_distance(pairs);

    println!("total distance: {}", sum);

    return Ok(());
}
