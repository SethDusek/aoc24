use itertools::Itertools;
use std::collections::*;
const INPUT: &str = include_str!("../input.txt");
const TEST: &str = include_str!("../test.txt");

fn parse(input: &str) -> HashMap<(i64, i64), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i64, j as i64), c))
        })
        .collect()
}

fn search(
    map: &HashMap<(i64, i64), char>,
    start: (i64, i64),
    rem: &[char],
    dir: (i64, i64),
) -> u16 {
    if rem.is_empty() {
        return 1;
    }
    if let Some(c) = map.get(&start) {
        if *c == rem[0] {
            if rem.len() > 1 {
                return search(map, (start.0 + dir.0, start.1 + dir.1), &rem[1..], dir);
            } else {
                return 1;
            }
        } else {
            return 0;
        }
    }
    return 0;
}
fn part1(input: &str) {
    let parsed = parse(input);
    let y_max = parsed.keys().max_by_key(|(y, _)| y).unwrap().0;
    let y_min = parsed.keys().min_by_key(|(y, _)| y).unwrap().0;
    let x_max = parsed.keys().max_by_key(|(_, x)| x).unwrap().1;
    let x_min = parsed.keys().min_by_key(|(_, x)| x).unwrap().1;
    let mut count = 0;
    let arr = ['X', 'M', 'A', 'S'];
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            // count += search(&parsed, (y, x), &arr[..]);
            let res: u16 = (-1..=1)
                .cartesian_product(-1..=1)
                .filter(|&dir| dir != (0, 0))
                .map(|dir| search(&parsed, (y, x), &arr, dir))
                .sum();
            count += res;
        }
    }
    println!("{count}");
}

fn part2(input: &str) {
    let map = parse(input);
    let y_max = map.keys().max_by_key(|(y, _)| y).unwrap().0;
    let x_max = map.keys().max_by_key(|(_, x)| x).unwrap().1;
    let pat = ['M', 'A', 'S'];
    let pat2 = ['S', 'A', 'M'];
    let mut count = 0;
    for y in 0..=y_max {
        for x in 0..=x_max {
            for (pat, pat2) in [pat, pat2].iter().cartesian_product([pat, pat2].iter()) {
                count +=
                    search(&map, (y, x), pat, (1, 1)) & search(&map, (y + 2, x), pat2, (-1, 1));
            }
        }
    }
    println!("{count}");
}
fn main() {
    part1(TEST);
    part1(INPUT);
    part2(TEST);
    part2(INPUT);
}
