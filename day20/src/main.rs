use std::collections::{HashMap, HashSet, VecDeque};

use arrayvec::ArrayVec;

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

#[derive(PartialOrd, PartialEq, Clone, Hash, Eq, Debug)]
struct Node {
    pos: (i64, i64),
    steps: i64,
}

fn unique_pos(positions: &[(i64, i64)]) -> ArrayVec<(i64, i64), 2> {
    let mut v = ArrayVec::new();
    if let Some(start) = positions.first() {
        v.push(*start);
    }
    if let Some(end) = positions.last() {
        v.push(*end);
    }
    v
}
fn bfs(grid: &HashMap<(i64, i64), char>, start: (i64, i64), end: (i64, i64)) -> Option<i64> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(Node {
        pos: start,
        steps: 0,
    });
    while let Some(cur) = queue.pop_front() {
        if cur.pos == end {
            return Some(cur.steps);
        }
        visited.insert(cur.pos);
        let adj = [(0, -1), (0, 1), (1, 0), (-1, 0)];
        for adj in adj {
            let next_pos = (cur.pos.0 + adj.0, cur.pos.1 + adj.1);
            if visited.contains(&next_pos)
                || grid.get(&next_pos).is_none()
                || grid.get(&next_pos) == Some(&'#')
            {
                continue;
            }
            queue.push_back(Node {
                pos: next_pos,
                steps: cur.steps + 1,
            })
        }
    }
    None
}

fn part1(input: &str) {
    let parsed = parse_grid(input);
    let start = *parsed.iter().find(|(_, v)| **v == 'S').unwrap().0;
    let end = *parsed.iter().find(|(_, v)| **v == 'E').unwrap().0;

    let mut from_start = HashMap::new();
    let mut from_end = HashMap::new();
    for (k, v) in &parsed {
        if *v == '#' {
            continue;
        }
        if let Some(dist) = bfs(&parsed, start, *k) {
            from_start.insert(*k, dist);
        }
        if let Some(dist) = bfs(&parsed, end, *k) {
            from_end.insert(*k, dist);
        }
    }
    let max_jump = 20i64;
    let atleast = 100;
    let mut count = 0;
    for (k, v) in &parsed {
        if !from_start.contains_key(k) || !from_end.contains_key(&k) {
            continue;
        }

        for y in (-max_jump)..=max_jump {
            for x in -(max_jump - y)..=max_jump - y {
                if x.abs() + y.abs() > max_jump {
                    continue;
                }
                if !from_end.contains_key(&(k.0 + y, k.1 + x)) {
                    continue;
                }
                let dist = from_start[&k] + x.abs() + y.abs() + from_end[&(k.0 + y, k.1 + x)];
                if dist < from_start[&k] + from_end[&k]
                    && (from_start[&k] + from_end[&k]) - dist >= atleast
                {
                    count += 1;
                }
            }
        }
    }
    println!("{}", from_start[&end]);
    println!("{count}");
}
fn main() {
    part1(TEST);
    part1(INPUT);
}
