const INPUT: &'static str = include_str!("../data.txt");
const TEST: &'static str = include_str!("../test.txt");

fn parse(input: &str) -> Vec<Vec<u64>> {
    let out: Vec<_> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();
    out
}

fn part1(input: &str) {
    let out = parse(input);
    dbg!(out
        .into_iter()
        .filter(|level| {
            (level.is_sorted_by(|a, b| a > b) || level.is_sorted())
                && level
                    .windows(2)
                    .all(|arr| arr[0].abs_diff(arr[1]) >= 1 && arr[0].abs_diff(arr[1]) <= 3)
        })
        .count());
}
fn part2(input: &str) {
    let out = parse(input);
    dbg!(out
        .into_iter()
        .filter(|level| {
            let mut any = false;
            for to_remove in 0..=level.len() {
                let mut level = level.clone();
                if to_remove < level.len() {
                    level.remove(to_remove);
                }
                any |= (level.is_sorted_by(|a, b| a > b) || level.is_sorted())
                    && level
                        .windows(2)
                        .all(|arr| arr[0].abs_diff(arr[1]) >= 1 && arr[0].abs_diff(arr[1]) <= 3)
            }
            any
        })
        .count());
}
fn main() {
    // part1(TEST);
    // part1(INPUT);
    part2(TEST);
    part2(INPUT);
}
