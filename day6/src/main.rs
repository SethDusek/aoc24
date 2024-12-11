use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

use itertools::Itertools;
const TEST: &str = include_str!("../test.txt");
const DATA: &str = include_str!("../data.txt");

fn parse(input: &str) -> HashMap<(i64, i64), char> {
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
fn part1(input: &str) {
    let parsed = parse(input);
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let chars = ['>', 'v', '<', '^'];
    let (start, mut dir, v) = parsed
        .iter()
        .filter_map(|(k, v)| chars.iter().position(|&c| c == *v).map(|dir| (k, dir, v)))
        .next()
        .unwrap();
    let mut pos = *start;
    let mut unique = HashSet::new();
    while let Some(&point) = parsed.get(&pos) {
        unique.insert(pos);
        let mut next_pos = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);
        while parsed.get(&next_pos) == Some(&'#') {
            dir = (dir + 1) % dirs.len();
            next_pos = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);
        }
        pos = next_pos;
    }
    println!("{}", unique.len());
}

fn escapes(grid: &HashMap<(i64, i64), char>, mut pos: (i64, i64), dir: (i64, i64)) -> bool {
    loop {
        match grid.get(&pos) {
            Some('#') => break false,
            None => break true,
            _ => {}
        }
        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
}

fn part2(input: &str) {
    let parsed = parse(input);
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let chars = ['>', 'v', '<', '^'];
    let (start, mut dir, v) = parsed
        .iter()
        .filter_map(|(k, v)| chars.iter().position(|&c| c == *v).map(|dir| (k, dir, v)))
        .next()
        .unwrap();
    let mut pos = *start;
    let mut visited: HashMap<(i64, i64), HashSet<usize>> = HashMap::new();
    while let Some(&_) = parsed.get(&pos) {
        visited
            .entry(pos)
            .and_modify(|hs| {
                hs.insert(dir);
            })
            .or_insert(HashSet::from_iter(once(dir)));
        let mut next_pos = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);
        while parsed.get(&next_pos) == Some(&'#') {
            dir = (dir + 1) % dirs.len();
            next_pos = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);
        }
        pos = next_pos;
    }
    for (pos, dirs) in &visited {
        for dir in dirs {}
    }
}
fn part2_real_final_fr_fr(input: &str) {
    let parsed = parse(input);
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let chars = ['>', 'v', '<', '^'];
    let (start, mut dir, v) = parsed
        .iter()
        .filter_map(|(k, v)| chars.iter().position(|&c| c == *v).map(|dir| (k, dir, v)))
        .next()
        .unwrap();
    let pos = *start;
    let mut count = 0;
    for x in 0..=parsed.keys().max_by_key(|(y, x)| x).unwrap().1 {
        for y in 0..=parsed.keys().max_by_key(|(y, x)| y).unwrap().0 {
            let mut cloned = parsed.clone();
            if (y, x) == pos {
                continue;
            }
            cloned.insert((y, x), '#');
            if traverse(&cloned, pos, dir, dirs) {
                count += 1;
            }
        }
    }
    println!("{count}");
}

fn traverse(
    parsed: &HashMap<(i64, i64), char>,
    mut pos: (i64, i64),
    mut dir: usize,
    dirs: [(i64, i64); 4],
) -> bool {
    let mut unique = HashSet::new();
    while let Some(&point) = parsed.get(&pos) {
        if unique.contains(&(pos, dir)) {
            return true;
        }
        unique.insert((pos, dir));
        let mut next_pos = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);
        while parsed.get(&next_pos) == Some(&'#') {
            dir = (dir + 1) % dirs.len();
            next_pos = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);
        }
        pos = next_pos;
    }
    return false;
}
fn main() {
    part1(TEST);
    part1(DATA);
    part2_real_final_fr_fr(TEST);
    part2_real_final_fr_fr(DATA);
}
