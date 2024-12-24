use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");
const TEST: &str = include_str!("../test.txt");

fn parse_grid(input: &str) -> HashMap<(i64, i64), char> {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i64, j as i64), c))
        })
        .collect();

    grid
}

fn rot_clockwise(dir: (i64, i64)) -> (i64, i64) {
    match dir {
        (0, 1) => (1, 0),
        (0, -1) => (-1, 0),
        (1, 0) => (0, -1),
        (-1, 0) => (0, 1),
        _ => unreachable!(),
    }
}
fn rot_counter(dir: (i64, i64)) -> (i64, i64) {
    match dir {
        (0, -1) => (1, 0),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        (1, 0) => (0, 1),
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct HeapElem {
    cost: usize,
    pos: (i64, i64),
    dir: (i64, i64),
}

impl Ord for HeapElem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost
            .cmp(&other.cost)
            .reverse()
            .then(self.pos.cmp(&other.pos))
            .then(self.dir.cmp(&other.dir))
    }
}
impl PartialOrd for HeapElem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
fn dfs(
    cur: ((i64, i64), (i64, i64)),
    prev: &HashMap<((i64, i64), (i64, i64)), HashSet<((i64, i64), (i64, i64))>>,
    visited: &mut HashSet<((i64, i64), (i64, i64))>,
) {
    let mut stack = vec![cur];
    while let Some(cur) = stack.pop() {
        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur);
        for prev in prev.get(&cur).into_iter().flatten() {
            stack.push(*prev);
        }
    }
}

fn display(grid: &HashMap<(i64, i64), char>, visited: &HashSet<((i64, i64), (i64, i64))>) {
    let max_x = grid.keys().map(|(_, x)| *x).max().unwrap();
    let max_y = grid.keys().map(|(y, _)| *y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..max_x {
            if visited.iter().any(|(pos, _)| *pos == (y, x)) {
                print!("O")
            } else {
                print!("{}", grid[&(y, x)]);
            }
        }
        println!("");
    }
}
fn part1(input: &str) {
    let parsed = parse_grid(input);
    let start_pos = *parsed.iter().find(|(_, v)| **v == 'S').unwrap().0;
    let mut queue = BinaryHeap::new();
    let mut dist = HashMap::new();
    let mut prev: HashMap<((i64, i64), (i64, i64)), HashSet<((i64, i64), (i64, i64))>> =
        HashMap::new();
    dist.insert((start_pos, (0, 1)), 0usize);
    queue.push(HeapElem {
        cost: 0,
        pos: start_pos,
        dir: (0, 1),
    });
    let mut visited = HashSet::new();
    let mut min_score = usize::MAX;
    while !queue.is_empty() {
        let cur = queue.pop().unwrap();
        if parsed.get(&cur.pos).is_none() {
            continue;
        }
        if parsed[&cur.pos] == '#' {
            continue;
        }
        if dist.get(&(cur.pos, cur.dir)).unwrap_or(&usize::MAX) < &cur.cost {
            continue;
        }
        if parsed[&cur.pos] == 'E' {
            min_score = min_score.min(cur.cost);
            if cur.cost == min_score {
                dfs((cur.pos, cur.dir), &prev, &mut visited);
            }
            println!("score: {}", cur.cost);
            // break;
        }

        let next = (cur.pos.0 + cur.dir.0, cur.pos.1 + cur.dir.1);
        if dist.get(&(next, cur.dir)).copied().unwrap_or(usize::MAX) >= (cur.cost + 1) {
            queue.push(HeapElem {
                pos: next,
                dir: cur.dir,
                cost: cur.cost + 1,
            });
            dist.insert((next, cur.dir), cur.cost + 1);
            prev.entry((next, cur.dir))
                .or_insert(HashSet::new())
                .insert((cur.pos, cur.dir));
        }
        let clock_dir = rot_clockwise(cur.dir);
        let counter_dir = rot_counter(cur.dir);
        if dist
            .get(&(cur.pos, clock_dir))
            .copied()
            .unwrap_or(usize::MAX)
            >= cur.cost + 1000
        {
            queue.push(HeapElem {
                pos: cur.pos,
                dir: clock_dir,
                cost: cur.cost + 1000,
            });
            dist.insert((cur.pos, clock_dir), cur.cost + 1000);
            prev.entry((cur.pos, clock_dir))
                .or_insert(HashSet::new())
                .insert((cur.pos, cur.dir));
        }
        if dist
            .get(&(cur.pos, counter_dir))
            .copied()
            .unwrap_or(usize::MAX)
            >= cur.cost + 1000
        {
            queue.push(HeapElem {
                pos: cur.pos,
                dir: counter_dir,
                cost: cur.cost + 1000,
            });
            dist.insert((cur.pos, counter_dir), cur.cost + 1000);
            prev.entry((cur.pos, counter_dir))
                .or_insert(HashSet::new())
                .insert((cur.pos, cur.dir));
        }
    }
    // display(&parsed, &visited);
    println!("part 2: {}", visited.iter().map(|p| p.0).unique().count());
}
fn main() {
    part1(TEST);
    part1(INPUT);
}
