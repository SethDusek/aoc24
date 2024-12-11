use std::collections::{HashMap, HashSet};

const TEST: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");
fn part1(input: &str) {
    let grid: HashMap<(i64, i64), u32> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((y as i64, x as i64), u32::from(c) - u32::from('0')))
        })
        .collect();
    // let mut visited = HashMap::new();
    println!(
        "{}",
        grid.iter()
            .filter(|(k, v)| **v == 0)
            .map(|(k, v)| {
                let mut visited = HashSet::new();
                dfs(*k, &grid, &mut visited, true)
            })
            .sum::<u32>()
    );
    println!(
        "{}",
        grid.iter()
            .filter(|(k, v)| **v == 0)
            .map(|(k, v)| {
                let mut visited = HashSet::new();
                dfs(*k, &grid, &mut visited, false)
            })
            .sum::<u32>()
    );
}

fn dfs(
    cur: (i64, i64),
    grid: &HashMap<(i64, i64), u32>,
    visited: &mut HashSet<(i64, i64)>,
    part1: bool,
) -> u32 {
    let adj = [
        (cur.0 - 1, cur.1),
        (cur.0 + 1, cur.1),
        (cur.0, cur.1 - 1),
        (cur.0, cur.1 + 1),
    ];
    if part1 && visited.contains(&cur) {
        return 0;
    }
    if let Some(9) = grid.get(&cur) {
        visited.insert(cur);
        return 1;
    }
    let mut sum = 0;
    for adj in adj {
        if let Some(&v) = grid.get(&adj) {
            if v > grid[&cur] && v == grid[&cur] + 1 {
                sum += dfs(adj, grid, visited, part1);
            }
        }
    }
    sum
}
fn main() {
    part1(TEST);
    part1(INPUT);
}
