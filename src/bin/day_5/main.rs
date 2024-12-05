use std::{fs, io};

struct Run(Vec<Rule>, Vec<Update>);
struct Rule(u32, u32);
#[derive(Clone)]
struct Update(Vec<u32>);

impl Run {
    pub fn parse(contents: &str) -> Self {
        let mut lines = contents.trim().lines();

        let rules: Vec<_> = lines.by_ref().take_while(|x| !x.is_empty()).collect();
        let rules = rules.iter().map(|x| Rule::parse(x)).collect();

        let updates: Vec<_> = lines.collect();
        let updates = updates.iter().map(|x| Update::parse(x)).collect();

        Self(rules, updates)
    }

    pub fn correct_middle_sum(&self) -> u32 {
        self.1
            .iter()
            .filter(|x| x.validate_update(&self.0))
            .map(|x| x.middle_page())
            .sum()
    }

    fn incorrect_corrected_middle_sum(&mut self) -> u32 {
        self.1
            .clone()
            .iter_mut()
            .filter(|x| !x.validate_update(&self.0))
            .map(|x| x.reorder(&self.0).middle_page())
            .sum()
    }
}

impl Rule {
    pub fn parse(rule: &str) -> Self {
        let parts: Vec<_> = rule.trim().split('|').collect();
        assert!(parts.len() == 2);

        let first = parts[0].parse();
        assert!(first.is_ok());

        let second = parts[1].parse();
        assert!(second.is_ok());

        Self(first.unwrap(), second.unwrap())
    }
}

impl Update {
    pub fn parse(update: &str) -> Self {
        let values: Vec<_> = update
            .trim()
            .split(',')
            .map(|x| {
                let x = x.parse();
                assert!(x.is_ok());
                x.unwrap()
            })
            .collect();

        assert!(values.len() > 0);

        Self(values)
    }

    pub fn validate_update(&self, rules: &Vec<Rule>) -> bool {
        for i in 0..self.0.len() - 1 {
            for j in i + 1..self.0.len() {
                let failed_rules: Vec<_> = rules
                    .iter()
                    .filter(|x| self.0[i] == x.1 && self.0[j] == x.0)
                    .collect();

                if failed_rules.len() > 0 {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn reorder(&mut self, rules: &Vec<Rule>) -> Self {
        let mut ordered = self.0.clone();

        for i in 0..ordered.len() {
            for j in 0..ordered[0..ordered.len() - i].len() - 1 {
                let failed_rules: Vec<_> = rules
                    .iter()
                    .filter(|y| ordered[j] == y.1 && ordered[j + 1] == y.0)
                    .collect();

                if failed_rules.len() > 0 {
                    ordered.swap(j, j + 1);
                }
            }
        }

        Self(ordered)
    }

    pub fn middle_page(&self) -> u32 {
        assert!(self.0.len() > 0);
        assert!(self.0.len() % 2 == 1);
        let value = self.0.iter().nth(self.0.len() / 2);
        assert!(value.is_some());
        value.unwrap().to_owned()
    }
}

fn main() -> Result<(), io::Error> {
    let contents = fs::read_to_string("src/inputs/day5.txt")?;

    let mut parsed = Run::parse(&contents);

    // Part 1
    let middles = parsed.correct_middle_sum();
    println!("Part 1: middle sum = {}", middles);

    // Part 2
    let middles = parsed.incorrect_corrected_middle_sum();
    println!("Part 2: middle sum = {}", middles);

    Ok(())
}
