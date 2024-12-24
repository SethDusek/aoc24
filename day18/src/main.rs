use std::collections::{HashMap, HashSet, VecDeque};

const TEST: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");
fn parse_grid(input: &str) -> HashMap<(i64, i64), char> {
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

fn part1(input: &str, num: usize, size: i64) -> bool {
    let steps = input
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .collect::<Vec<(i64, i64)>>();
    let mut grid = HashMap::new();
    for y in 0..=size {
        for x in 0..=size {
            grid.insert((y, x), '.');
        }
    }
    for (x, y) in steps.iter().take(num) {
        grid.insert((*y, *x), '#');
    }
    let start = (0, 0);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0));
    let mut found = false;
    while let Some(next) = queue.pop_front() {
        if next.0 == (size, size) {
            found = true;
            // println!("{}", next.1);
            break;
        }
        if grid.get(&next.0) == Some(&'#') {
            continue;
        }

        if visited.contains(&next.0) {
            continue;
        }
        visited.insert(next.0);
        let adj = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for adj in adj {
            let next_pos = (next.0 .0 + adj.0, next.0 .1 + adj.1);
            if visited.contains(&next_pos) || grid.get(&next_pos).is_none() {
                continue;
            }
            queue.push_back((next_pos, next.1 + 1));
        }
    }
    if !found {
        println!("No solution at {:?}", steps[num - 1]);
    }
    found
}
fn main() {
    for i in 0..TEST.lines().count() {
        if !part1(TEST, i, 6) {
            break;
        }
    }
    for i in 0..INPUT.lines().count() {
        if !part1(INPUT, i, 70) {
            break;
        }
    }
}
