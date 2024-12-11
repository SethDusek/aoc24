use itertools::Itertools;
use regex::Regex;
use std::collections::*;
const DATA: &str = include_str!("../data.txt");
const TEST: &str = include_str!("../test.txt");

enum Op {
    Enable,
    Disable,
    Mul(u64, u64),
}
fn parse(input: &str) -> Vec<Op> {
    let mut output = vec![];
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
    for line in re.captures_iter(input) {
        if line.name("do").is_some() {
            output.push(Op::Enable);
        } else if line.name("dont").is_some() {
            output.push(Op::Disable);
        } else {
            let mut iter = line
                .iter()
                .flatten()
                .skip(1)
                .map(|v| v.as_str().parse::<u64>().unwrap());
            output.push(Op::Mul(iter.next().unwrap(), iter.next().unwrap()));
        }
    }
    output
}

fn part1(input: &str) {
    let matches = parse(input);
    let mut sum = 0;
    for (a, b) in matches.iter().filter_map(|m| match m {
        Op::Mul(a, b) => Some((a, b)),
        _ => None,
    }) {
        sum += a * b;
    }
    dbg!(sum);
}

fn part2(input: &str) {
    let matches = parse(input);
    let mut sum = 0;
    let mut enabled = true;
    for matched in matches {
        match matched {
            Op::Enable => enabled = true,
            Op::Disable => enabled = false,
            Op::Mul(a, b) if enabled => sum += a * b,
            _ => {}
        }
    }
    dbg!(sum);
}

fn main() {
    part1(TEST);
    part1(DATA);
    part2(TEST);
    part2(DATA);
}
