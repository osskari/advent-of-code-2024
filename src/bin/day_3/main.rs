use core::str;
use std::{fs, io, usize};

fn parse(contents: &[u8], on_off: bool) -> Vec<(u32, u32)> {
    let mut ret = vec![];
    let mut i = 0;
    let mut is_on = true;
    while i < contents.len() {
        if on_off && &contents[i..contents.len().min(i + 4)] == b"do()" {
            is_on = true;
            i += 4;
        }

        if on_off && &contents[i..contents.len().min(i + 7)] == b"don't()" {
            is_on = false;
            i += 7;
        }

        let upper_bound = contents.len().min(i + 4);

        if !is_on || &contents[i..upper_bound] != b"mul(" {
            i += 1;
            continue;
        }

        i += 4;

        if i >= contents.len() {
            break;
        }

        let lhs = parse_number(&contents[i..], b',');
        i += lhs.1;

        if lhs.0.is_none() {
            continue;
        }

        i += 1;

        if i >= contents.len() {
            break;
        }

        let rhs = parse_number(&contents[i..], b')');
        i += rhs.1;

        if rhs.0.is_none() {
            continue;
        }

        ret.push((lhs.0.unwrap(), rhs.0.unwrap()));

        i += 1;
    }

    return ret;
}

fn parse_number(contents: &[u8], end_byte: u8) -> (Option<u32>, usize) {
    let mut i = 0;

    while i < contents.len() && contents[i] != end_byte {
        let curr = contents[i] as char;
        if !curr.is_numeric() {
            return (None, i);
        }

        i += 1;
    }

    let number = str::from_utf8(&contents[0..i]);
    if number.is_err() {
        return (None, i);
    }

    let number = number.unwrap().parse();
    if number.is_err() {
        return (None, i);
    }

    return (Some(number.unwrap()), i);
}

fn main() -> Result<(), io::Error> {
    let contents = fs::read_to_string("src/inputs/day3.txt")?;

    // Part 1
    let nums = parse(contents.as_bytes(), false);
    let mul_sum: u32 = nums.iter().map(|x| x.0 * x.1).sum();
    println!("Part 1:\nmult sum = {}\n", mul_sum);

    // Part 2
    let nums = parse(contents.as_bytes(), true);
    let mul_sum: u32 = nums.iter().map(|x| x.0 * x.1).sum();
    println!("Part 2:\nmult sum = {}\n", mul_sum);

    Ok(())
}
