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
    cheat_positions: ArrayVec<(i64, i64), 20>,
    steps: usize,
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
fn bfs(
    grid: &HashMap<(i64, i64), char>,
    start: (i64, i64),
    end: (i64, i64),
    cheats_allowed: bool,
    without_cheats: Option<usize>,
    atleast: usize,
) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(Node {
        pos: start,
        cheat_positions: ArrayVec::new(),
        steps: 0,
    });
    let mut unique = HashSet::new();
    while let Some(cur) = queue.pop_front() {
        if cur.pos == end {
            if let Some(without) = without_cheats {
                if dbg!(cur.steps.abs_diff(without)) >= atleast {
                    unique.insert(cur.cheat_positions.clone());
                } else {
                    break;
                }
            } else {
                return Some(cur.steps);
            }
            println!("Goal at {} steps", cur.steps);
            continue;
        }
        if visited.contains(&(cur.pos, unique_pos(&cur.cheat_positions)))
            || cur.cheat_positions.len() == cur.cheat_positions.capacity()
                && grid.get(&cur.pos) == Some(&'#')
        {
            continue;
        }
        visited.insert((cur.pos, unique_pos(&cur.cheat_positions)));
        let adj = [(0, -1), (0, 1), (1, 0), (-1, 0)];
        for adj in adj {
            let next_pos = (cur.pos.0 + adj.0, cur.pos.1 + adj.1);
            let mut needs_cheat = false;
            if grid.get(&next_pos).is_none() {
                continue;
            }
            if grid.get(&next_pos) == Some(&'#') {
                if !cheats_allowed || cur.cheat_positions.len() >= cur.cheat_positions.capacity() {
                    continue;
                }
                needs_cheat = true;
            }

            let cheat_positions: ArrayVec<_, 20> = if cheats_allowed
                && (needs_cheat
                    || cur.cheat_positions.len() > 0
                        && cur.cheat_positions.len() < cur.cheat_positions.capacity())
            {
                cur.cheat_positions
                    .iter()
                    .copied()
                    .chain(std::iter::once(cur.pos))
                    .take(cur.cheat_positions.capacity())
                    .collect()
            } else {
                cur.cheat_positions.clone()
            };
            if visited.contains(&(next_pos, unique_pos(&cheat_positions))) {
                continue;
            }
            queue.push_back(Node {
                pos: next_pos,
                cheat_positions,
                steps: cur.steps + 1,
            });
        }
    }
    println!("atleast: {}", unique.len());
    None
}

fn part1(input: &str) {
    let parsed = parse_grid(input);
    let start = *parsed.iter().find(|(_, v)| **v == 'S').unwrap().0;
    let end = *parsed.iter().find(|(_, v)| **v == 'E').unwrap().0;
    let without_cheats = bfs(&parsed, start, end, false, None, 0).unwrap();
    bfs(&parsed, start, end, true, Some(without_cheats), 100);
}
fn main() {
    part1(TEST);
    part1(INPUT);
}
