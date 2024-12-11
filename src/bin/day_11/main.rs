use std::{time, usize};

use ahash::AHashMap;

#[derive(Clone)]
struct Stones {
    stones: AHashMap<String, usize>,
}

impl Stones {
    fn parse(contents: &str) -> Self {
        let mut stones = AHashMap::new();

        contents.trim().split_whitespace().for_each(|x| {
            stones
                .entry(x.to_string())
                .and_modify(|v| *v += 1)
                .or_insert(1);
        });

        Self { stones }
    }

    fn count_at_iteration(&mut self, iteration: usize) -> usize {
        let mut cache = AHashMap::new();
        for _ in 0..iteration {
            let mut it_keys = AHashMap::new();

            for i in self.stones.keys() {
                match cache.get(i) {
                    Some(value) => {
                        Self::update_count(&mut it_keys, &value, self.stones[i]);
                    }
                    None => {
                        let value = Self::get_next_value(&i);
                        Self::update_count(&mut it_keys, &value, self.stones[i]);
                        cache.insert(i.to_string(), value);
                    }
                }
            }
            self.stones = it_keys;
        }

        self.stones.values().sum()
    }

    fn update_count(map: &mut AHashMap<String, usize>, keys: &(String, Option<String>), i: usize) {
        map.entry(keys.0.to_string())
            .and_modify(|x| *x += i)
            .or_insert(i);

        match &keys.1 {
            Some(k) => {
                map.entry(k.to_string())
                    .and_modify(|x| *x += i)
                    .or_insert(i);
            }
            None => {}
        };
    }

    fn get_next_value(key: &String) -> (String, Option<String>) {
        if key == "0" {
            (String::from("1"), None)
        } else if key.len() % 2 == 0 {
            let (v0, v1) = key.split_at(key.len() / 2);

            let v0 = v0.parse::<usize>();
            assert!(v0.is_ok());
            let v1 = v1.parse::<usize>();
            assert!(v1.is_ok());

            (v0.unwrap().to_string(), Some(v1.unwrap().to_string()))
        } else {
            let key = key.parse::<usize>();
            assert!(key.is_ok());

            (format!("{}", key.unwrap() * 2024), None)
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let contents = std::fs::read_to_string("src/inputs/day11.txt")?;

    let mut stones = Stones::parse(&contents);

    // Part 1
    let mut stones_1 = stones.clone();
    let start = time::Instant::now();
    println!(
        "Part 1: count = {}, completed in = {:?}\n",
        stones_1.count_at_iteration(25),
        start.elapsed()
    );

    // Part 2
    let start = time::Instant::now();
    println!(
        "Part 2: count = {}, completed in = {:?}\n",
        stones.count_at_iteration(75),
        start.elapsed()
    );

    Ok(())
}
