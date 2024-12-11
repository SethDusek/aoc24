use std::cmp::Ordering;

const INPUT: &str = include_str!("../input.txt");
const TEST: &str = include_str!("../test.txt");

fn part1(input: &str) {
    let (rules, input) = input.split_once("\n\n").unwrap();
    let rules: Vec<(u64, u64)> = rules
        .lines()
        .map(|l| l.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .collect();
    let lines = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|c| c.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut valid = 0;
    'outer: for line in lines {
        for (i, num) in line.iter().copied().enumerate() {
            for rule in rules.iter().filter(|(_, b)| *b == num) {
                if line[..]
                    .iter()
                    .position(|prev| *prev == rule.0)
                    .unwrap_or(0)
                    > i
                {
                    continue 'outer;
                }
            }
        }
        valid += line[line.len() / 2];
    }
    println!("{valid:?}");
}
fn part2(input: &str) {
    let (rules, input) = input.split_once("\n\n").unwrap();
    let rules: Vec<(u64, u64)> = rules
        .lines()
        .map(|l| l.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .collect();
    let lines = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|c| c.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut valid = 0;
    for line in lines {
        let mut new_line = line.clone();
        new_line.sort_by(|a, b| {
            if rules.iter().any(|(a1, b1)| a1 == b && b1 == a) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        if new_line != line {
            valid += new_line[new_line.len() / 2];
        }
    }
    println!("{valid:?}");
}

fn main() {
    part1(TEST);
    part1(INPUT);
    part2(TEST);
    part2(INPUT);
}
