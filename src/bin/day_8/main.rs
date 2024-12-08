use std::{fs, time, usize};

use ahash::{AHashMap, AHashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point(isize, isize);
#[derive(Debug)]
struct Antenae(AHashMap<char, Vec<Point>>);

impl Antenae {
    fn from(contents: &str) -> Self {
        let mut map = AHashMap::new();
        for (i, x) in contents.trim().lines().enumerate() {
            for (j, x) in x.trim().char_indices() {
                if x == '.' {
                    continue;
                }
                map.entry(x)
                    .and_modify(|c: &mut Vec<Point>| c.push(Point(i as isize, j as isize)))
                    .or_insert(vec![Point(i as isize, j as isize)]);
            }
        }

        Self(map)
    }
}

impl Point {
    fn find_bounds(contents: &str) -> Point {
        let mut lines = contents.trim().lines();
        let columns = lines.next().unwrap().len();
        let rows = lines.count() + 1;

        Point(rows as isize, columns as isize)
    }

    fn sub(&self, other: &Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }

    fn add(&self, other: &Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }

    fn inbounds(&self, bounds: &Point) -> bool {
        self.0 >= 0 && self.0 < bounds.0 && self.1 >= 0 && self.1 < bounds.1
    }

    fn find_surrounding_antinodes(&self, other: &Point, bounds: &Point) -> Vec<Point> {
        let step = other.sub(self);
        let mut points = vec![];

        let first = self.sub(&step);
        if first.inbounds(&bounds) {
            points.push(first);
        };

        let second = other.add(&step);
        if second.inbounds(&bounds) {
            points.push(second);
        };

        points
    }

    fn find_all_inline_antinodes(&self, other: &Point, bounds: &Point) -> Vec<Point> {
        let step = other.sub(self);

        let mut points = vec![self.clone(), other.clone()];

        let mut first = self.sub(&step);
        while first.inbounds(&bounds) {
            points.push(first);
            first = first.sub(&step);
        }

        let mut first = other.add(&step);
        while first.inbounds(&bounds) {
            points.push(first);
            first = first.add(&step);
        }

        points
    }
}

fn get_point_pairs(points: &Vec<Point>) -> Vec<(Point, Point)> {
    points
        .iter()
        .flat_map(|&x| points.iter().map(move |&y| (x, y)))
        .filter(|(a, b)| a.1 < b.1 || (a.1 == b.1 && a.0 < b.0))
        .collect()
}

fn iterate_all_pairs<F>(antenae: &Antenae, func: F) -> usize
where
    F: Fn(&Point, &Point) -> Vec<Point>,
{
    let mut points = AHashSet::new();

    for key in antenae.0.keys() {
        let items = antenae.0.get(key).unwrap();

        for (first, second) in get_point_pairs(items) {
            for point in func(&first, &second) {
                points.insert(point.clone());
            }
        }
    }

    points.len()
}

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("src/inputs/day8.txt")?;

    let antenae = Antenae::from(&contents);
    let bounds = Point::find_bounds(&contents);

    // Part 1
    let start = time::Instant::now();
    let count = iterate_all_pairs(&antenae, |first, second| {
        first.find_surrounding_antinodes(second, &bounds)
    });

    println!(
        "Part 1: count = {}, competed in = {:?}\n",
        count,
        start.elapsed()
    );

    // Part 2
    let start = time::Instant::now();
    let count = iterate_all_pairs(&antenae, |first, second| {
        first.find_all_inline_antinodes(&second, &bounds)
    });

    println!(
        "Part 2: count = {}, competed in = {:?}\n",
        count,
        start.elapsed()
    );

    Ok(())
}
