use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;
const TEST: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

fn det(a: (i64, i64), b: (i64, i64)) -> i64 {
    a.0 * b.1 - b.0 * a.1
}
fn shoelace(vertices: &[(i64, i64)]) -> i64 {
    let mut sum = 0;
    //return (0..vertices.len()).into_par_iter().fold(|| 0, |a, b| a+det(vertices[b], vertices[(b + 1) % vertices.len()])).sum::<i64>().abs() / 2;
    for i in 0..vertices.len() {
        sum += det(vertices[i], vertices[(i + 1) % vertices.len()]);
    }
    sum.abs() / 2
}
fn part1(input: &str) {
    let mut parsed: HashMap<(i64, i64), char> = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(move |(j, c)| (((i * 2) as i64, (j * 2) as i64), c))
        })
        .collect();
    let max_x = parsed.keys().map(|(_, x)| *x).max().unwrap();
    let max_y = parsed.keys().map(|(y, _)| *y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            parsed.entry((y, x)).or_insert(' ');
        }
    }
    let mut visited = HashSet::new();
    let mut sum = 0;
    let mut part2 = 0;
    for y in 0..=max_y {
        for x in 0..=max_x {
            if visited.contains(&(y, x)) || parsed.get(&(y, x)) == Some(&' ') {
                continue;
            }
            let area = dfs(&parsed, (y, x), &mut visited);
            sum += area.0;
            part2 += area.1;
        }
    }
    println!("ans = {sum}");
    println!("ans2 = {part2}");
}

fn dfs(
    grid: &HashMap<(i64, i64), char>,
    cur_pos: (i64, i64),
    visited: &mut HashSet<(i64, i64)>,
) -> (u64, u64) {
    let cur_char = grid[&cur_pos];
    let adj = [(0, 2), (2, 0), (0, -2), (-2, 0)];
    let mut stack = vec![cur_pos];
    let mut visited_count = 0;
    let mut vertices = vec![];
    while !stack.is_empty() {
        let cur = stack.pop().unwrap();
        if visited.contains(&cur) {
            continue;
        }
        vertices.push(cur);
        visited.insert(cur);
        visited_count += 1;
        for adj in adj {
            let new_pos = (cur.0 + adj.0, cur.1 + adj.1);
            if let Some(c) = grid.get(&new_pos) {
                if *c == cur_char && !visited.contains(&new_pos) {
                    stack.push(new_pos);
                }
            }
        }
    }
    let mut unique = BTreeSet::new();
    for vert in &vertices {
        let adj = [(0, -1), (0, 1), (1, 0), (-1, 0)];
        for adj in adj {
            let pos = (vert.0 + adj.0, vert.1 + adj.1);
            if grid.get(&(pos.0 + adj.0, pos.1 + adj.1)) != Some(&cur_char) {
                unique.insert((pos, adj));
            }
        }
    }
    (
        unique.len() as u64 * visited_count,
        visited_count * horizontal(&grid, &unique, cur_char),
    )
}

fn horizontal(
    grid: &HashMap<(i64, i64), char>,
    set: &BTreeSet<((i64, i64), (i64, i64))>,
    c: char,
) -> u64 {
    let mut unique = set.iter().copied().collect::<Vec<_>>();
    let mut i = 0;
    let mut cur: Option<usize> = None;
    let mut horizontal = 0;
    while i < unique.len() {
        if unique[i].1 != (-1, 0) && unique[i].1 != (1, 0) {
            i += 1;
            continue;
        }
        // if grid.get(&(unique[i].0 + 1, unique[i].1)) != Some(&c)
        //     && grid.get(&(unique[i].0 - 1, unique[i].1)) != Some(&c)
        // {
        //     i += 1;
        //     continue;
        // }
        if let Some(seg) = cur {
            if unique[i].1 == unique[seg].1
                && unique[i].0 .0 == unique[seg].0 .0
                && unique[i].0 .1 - unique[seg].0 .1 == 2
            {
                cur = Some(i);
            } else {
                cur = Some(i);
                horizontal += 1;
            }
        } else {
            cur = Some(i);
            horizontal += 1;
        }
        i += 1;
    }
    horizontal * 2
}

fn part2(input: &str) {}
fn main() {
    part1(TEST);
    part1(INPUT);
    part2(TEST);
    part2(INPUT);
}
