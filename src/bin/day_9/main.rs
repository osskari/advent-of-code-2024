use core::panic;
use std::{time, usize};

#[derive(Debug, Clone)]
struct DiscMap {
    map: Vec<Space>,
    count: usize,
}

#[derive(Debug, Clone, Copy)]
enum Space {
    File(usize, usize),
    Space(usize),
}

impl DiscMap {
    fn from(contents: &str) -> Self {
        let mut map = vec![];
        let mut id = 0;
        for (i, x) in contents.trim().char_indices() {
            assert!(x.is_numeric());
            let x = x as usize - '0' as usize;

            let item = if i % 2 == 0 {
                let t = Space::File(id, x);
                id += 1;
                t
            } else {
                Space::Space(x)
            };

            map.push(item);
        }

        let count = map.len();

        Self { map, count }
    }

    fn flatten(&self) -> Self {
        let flattened: Vec<_> = self
            .map
            .iter()
            .flat_map(|x| match x {
                Space::File(id, count) => {
                    (0..*count).map(|_| Space::File(*id, 1)).collect::<Vec<_>>()
                }
                Space::Space(count) => (0..*count).map(|_| Space::Space(1)).collect::<Vec<_>>(),
            })
            .collect();

        let count = flattened.len();

        Self {
            map: flattened,
            count,
        }
    }

    fn frag(&mut self) {
        let mut slot = 0;
        let mut num = self.count;

        loop {
            slot = match self.find_slot(slot, 1) {
                Some(s) => s,
                None => return,
            };

            num = match self.find_num(num) {
                Some(n) => n,
                None => return,
            };

            if num < slot {
                return;
            }

            self.map.swap(slot, num);
        }
    }

    fn defrag(&mut self) {
        let mut num_i = self.map.len();

        loop {
            num_i = match self.find_num(num_i) {
                Some(i) => i,
                None => return,
            };
            let num_count = self.map[num_i].get_count();

            let slot_i = match self.find_slot(0, num_count) {
                Some(i) => {
                    if i > num_i {
                        continue;
                    }
                    i
                }
                None => continue,
            };
            let slot_count = self.map[slot_i].get_count();

            self.map[slot_i] = match self.map[slot_i] {
                Space::File(_, _) => panic!("SHOULD BE SPACE"),
                Space::Space(_) => Space::Space(num_count),
            };

            self.map.swap(slot_i, num_i);

            self.combine_slots(num_i);

            self.map
                .insert(slot_i + 1, Space::Space(slot_count - num_count));
        }
    }

    fn combine_slots(&mut self, pivot: usize) {
        let current = match self.map[pivot] {
            Space::File(_, _) => panic!("PIVOT IS FILE"),
            Space::Space(c) => c,
        };

        let after = if pivot + 1 < self.map.len() {
            match self.map[pivot + 1] {
                Space::File(_, _) => None,
                Space::Space(c) => {
                    self.map.remove(pivot + 1);
                    Some(c)
                }
            }
        } else {
            None
        };

        let before = if pivot >= 1 {
            match self.map[pivot - 1] {
                Space::File(_, _) => None,
                Space::Space(c) => {
                    self.map.remove(pivot - 1);
                    Some(c)
                }
            }
        } else {
            None
        };

        let count = match (before, after) {
            (None, None) => current,
            (None, Some(a)) => current + a,
            (Some(b), None) => b + current,
            (Some(b), Some(a)) => b + current + a,
        };

        let new_index = if before.is_some() { pivot - 1 } else { pivot };

        self.map[new_index] = Space::Space(count);
    }

    fn find_slot(&self, start: usize, num_size: usize) -> Option<usize> {
        for i in start..self.map.len() {
            match self.map[i] {
                Space::File(_, _) => {}
                Space::Space(c) => {
                    if c >= num_size {
                        return Some(i);
                    }
                }
            }
        }
        None
    }

    fn find_num(&self, start: usize) -> Option<usize> {
        for i in (0..start).rev() {
            match self.map[i] {
                Space::File(_, _) => {
                    return Some(i);
                }
                Space::Space(_) => {}
            }
        }
        None
    }

    fn checksum(&self) -> usize {
        let mut real_id = 0;
        self.map
            .iter()
            .enumerate()
            .flat_map(|(_, x)| match x {
                Space::File(id, count) => (0..*count)
                    .map(|_| {
                        let res = id * real_id;
                        real_id += 1;
                        res
                    })
                    .collect::<Vec<_>>(),
                Space::Space(count) => {
                    real_id += count;
                    vec![]
                }
            })
            .sum()
    }
}

impl Space {
    fn get_count(&self) -> usize {
        match self {
            Space::File(_, count) => *count,
            Space::Space(count) => *count,
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let contents = std::fs::read_to_string("src/inputs/day9.txt")?;
    let mut map = DiscMap::from(&contents);

    // Part 1
    let start = time::Instant::now();
    let mut flat = map.clone().flatten();
    flat.frag();
    println!(
        "Part 1: checksum = {}, competed in = {:?}\n",
        flat.checksum(),
        start.elapsed()
    );

    // Part 2
    let start = time::Instant::now();
    map.defrag();
    println!(
        "Part 2: checksum = {}, competed in = {:?}\n",
        map.checksum(),
        start.elapsed()
    );

    Ok(())
}
