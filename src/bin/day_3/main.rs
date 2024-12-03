use core::str;
use std::{fs, io, usize};

struct TokenList(Vec<Token>);

enum RunOptions {
    Default,
    DoDont,
}

impl TokenList {
    pub fn parse_string(contents: &[u8]) -> Self {
        let mut i = 0;
        let mut ret = vec![];
        while i < contents.len() {
            let token = Token::parse(&contents[i..]);

            if token.0.is_none() {
                i += token.1;
                continue;
            }

            ret.push(token.0.unwrap());

            i += 1;
        }
        return Self(ret);
    }

    pub fn get_multsum(&self, opts: RunOptions) -> u32 {
        let mut is_on = true;
        let mut mults = vec![];

        for x in self.0.iter() {
            match x {
                Token::Mul(pair) => {
                    if is_on {
                        mults.push(pair.mult());
                    }
                }
                Token::Do => {
                    if matches!(opts, RunOptions::DoDont) {
                        is_on = true;
                    }
                }
                Token::Dont => {
                    if matches!(opts, RunOptions::DoDont) {
                        is_on = false;
                    }
                }
            }
        }

        return mults.iter().sum();
    }
}

#[derive(Debug)]
enum Token {
    Mul(Pair),
    Do,
    Dont,
}

impl Token {
    fn parse(contents: &[u8]) -> (Option<Self>, usize) {
        if &contents[0..contents.len().min(4)] == b"do()" {
            return (Some(Self::Do), 5);
        } else if &contents[0..contents.len().min(7)] == b"don't()" {
            return (Some(Self::Dont), 8);
        } else if &contents[0..contents.len().min(4)] == b"mul(" {
            let pair = Pair::parse(&contents[4..]);

            if pair.is_none() {
                return (None, 1);
            }

            let pair = pair.unwrap();
            let i = pair.2;

            return (Some(Self::Mul(pair)), i);
        }
        return (None, 1);
    }
}

#[derive(Debug)]
struct Pair(u32, u32, usize);

impl Pair {
    pub fn parse(contents: &[u8]) -> Option<Pair> {
        let lhs = Self::parse_num(&contents, b',');
        if lhs.0.is_none() {
            return None;
        }

        let rhs = Self::parse_num(&contents[lhs.1..], b')');
        if rhs.0.is_none() {
            return None;
        }

        return Some(Pair(lhs.0?, rhs.0?, 4 + lhs.1 + rhs.1));
    }

    fn parse_num(contents: &[u8], end: u8) -> (Option<u32>, usize) {
        let mut i = 0;
        while i < contents.len() {
            if contents[i] == end {
                break;
            } else if !(contents[i] as char).is_numeric() {
                return (None, i);
            }
            i += 1;
        }

        let num = str::from_utf8(&contents[0..i]);
        if num.is_err() {
            return (None, i);
        }

        let num = num.unwrap().parse();
        if num.is_err() {
            return (None, i);
        }

        return (Some(num.unwrap()), i + 1);
    }

    pub fn mult(&self) -> u32 {
        return self.0 * self.1;
    }
}

fn main() -> Result<(), io::Error> {
    let contents = fs::read_to_string("src/inputs/day3.txt")?;

    let nums = TokenList::parse_string(contents.as_bytes());

    // Part 1
    println!(
        "Part 1:\nmult sum = {}\n",
        nums.get_multsum(RunOptions::Default)
    );

    // Part 2
    println!(
        "Part 2:\nmult sum = {}\n",
        nums.get_multsum(RunOptions::DoDont)
    );

    Ok(())
}
