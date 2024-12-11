use std::collections::{BTreeSet, HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");
const TEST: &str = include_str!("../test.txt");

type Parsed = HashMap<(i64, i64), char>;

fn parse(input: &str) -> Parsed {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((y as i64, x as i64), c))
        })
        .collect()
}
fn nodes(parsed: &Parsed) -> HashMap<char, Vec<(i64, i64)>> {
    parsed
        .iter()
        .filter(|((_, _), c)| c.is_lowercase() || c.is_uppercase() || c.is_digit(10))
        .fold(HashMap::new(), |mut hm, ((y, x), c)| {
            hm.entry(*c)
                .and_modify(|v| v.push((*y, *x)))
                .or_insert(vec![(*y, *x)]);
            hm
        })
}
fn mhd((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    (x2.abs_diff(x1).pow(2) + y2.abs_diff(y1).pow(2)) as i64
}
fn part1(input: &str) {
    let parsed = parse(input);
    let nodes = nodes(&parsed);
    let mut count = 0;
    for y in 0..=parsed.keys().map(|(y, _)| y).max().copied().unwrap() {
        'outer: for x in 0..=parsed.keys().map(|(_, x)| x).max().copied().unwrap() {
            for (frequency, points) in &nodes {
                for i in 0..points.len() {
                    let diff = (points[i].0 - y, points[i].1 - x);
                    if diff == (0, 0) {
                        continue;
                    }
                    if parsed.get(&(y + diff.0 * 2, x + diff.1 * 2)) == Some(frequency) {
                        count += 1;
                        continue 'outer;
                    }
                }
            }
        }
    }
    println!("{count}");
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    if b == 0 {
        panic!();
    }
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}
fn part2(input: &str) {
    let parsed = parse(input);
    let nodes = nodes(&parsed);
    let mut unique: BTreeSet<(i64, i64)> = BTreeSet::new();
    for (frequency, points) in &nodes {
        for i in 0..points.len() - 1 {
            for p2 in &points[i + 1..] {
                let mut diff = (points[i].0 - p2.0 as i64, points[i].1 - p2.1 as i64);
                diff = (diff.0 / gcd(diff.0, diff.1), diff.1 / gcd(diff.0, diff.1));
                println!("diff = x = {}, y = {}", diff.1, diff.0);
                println!("start = x = {}, y = {}", points[i].1, points[i].0);
                // println!(
                //     "p1 = x = {} y = {} p2 = x = {} y = {} diff = x = {}, y = {}",
                //     points[i].1, points[i].0, p2.1, p2.0, diff.1, diff.0
                // );
                for step in 0.. {
                    let point = (points[i].0 + diff.0 * step, points[i].1 + diff.1 * step);
                    let point2 = (points[i].0 + diff.0 * -step, points[i].1 + diff.1 * -step);
                    println!("point = x = {}, y = {}", point.1, point.0);
                    println!("point = x = {}, y = {}", point2.1, point2.0);
                    if parsed.get(&point).is_some() {
                        unique.insert(point);
                    }
                    if parsed.get(&point2).is_some() {
                        unique.insert(point2);
                    }
                    if parsed.get(&point).is_none() && parsed.get(&point2).is_none() {
                        break;
                    }
                }
            }
        }
    }
    println!("{}", unique.len());
}
fn main() {
    part1(TEST);
    part1(INPUT);
    part2(TEST);
    part2(INPUT);
}
