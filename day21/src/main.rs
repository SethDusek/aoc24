use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    iter::once,
};

use itertools::Itertools;

const TEST: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");
type Grid = HashMap<(i64, i64), char>;
fn parse_grid(input: &str) -> Grid {
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

fn backtrack(
    mut cur: (i64, i64),
    start: (i64, i64),
    prev: &HashMap<(i64, i64), (i64, i64)>,
) -> Vec<char> {
    let mut path = vec!['A'];
    while cur != start {
        let previous = prev[&cur];
        let diff = (cur.0 - previous.0, cur.1 - previous.1);
        path.push(match diff {
            (0, 1) => '>',
            (0, -1) => '<',
            (1, 0) => 'v',
            (-1, 0) => '^',
            _ => unreachable!(),
        });
        cur = prev[&cur];
    }
    path.reverse();
    path
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
    }
}
impl PartialOrd for HeapElem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn bfs(
    grid: &Grid,
    start_pos: (i64, i64),
    goal: (i64, i64),
    dir_map: &HashMap<(i64, i64), char>,
    start_dir: (i64, i64),
) -> Option<((i64, i64), Vec<char>)> {
    // let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    let mut dist = HashMap::new();
    queue.push(HeapElem {
        cost: 0,
        pos: start_pos,
        dir: start_dir,
    });
    let mut prev = HashMap::new();
    while let Some(cur) = queue.pop() {
        if cur.pos == goal {
            return Some((cur.dir, backtrack(cur.pos, start_pos, &prev)));
        }
        if dist.get(&cur.pos).copied().unwrap_or(usize::MAX) < cur.cost {
            continue;
        }
        // if visited.contains(&cur.pos) {
        //     continue;
        // }
        // visited.insert(cur.pos);
        // let adj = [((0, 1), 1), ((-1, 0), 1), ((1, 0), 2), ((0, -1), 3)];
        let adjacents = [((0, 1), '>'), ((-1, 0), '^'), ((1, 0), 'v'), ((0, -1), '<')];
        for (adj, adj_cost) in adjacents {
            let next_pos = (cur.pos.0 + adj.0, cur.pos.1 + adj.1);
            let next_cost = {
                let a = find_char(&dir_map, adj_cost);
                let b = find_char(
                    &dir_map,
                    adjacents
                        .iter()
                        .find(|p| p.0 == cur.dir)
                        .map(|p| p.1)
                        .unwrap_or('A'),
                );
                cur.cost + (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as usize
            };
            if !grid.contains_key(&next_pos)
                || dist.get(&next_pos).copied().unwrap_or(usize::MAX) <= next_cost
            {
                continue;
            }
            dist.insert(next_pos, next_cost);
            prev.insert(next_pos, cur.pos);
            queue.push(HeapElem {
                pos: next_pos,
                cost: next_cost,
                dir: adj,
            });
        }
    }
    None
}

fn find_char(grid: &Grid, c: char) -> (i64, i64) {
    *grid.iter().find(|(_, v)| c == **v).unwrap().0
}

// not guaranteed to be valid
fn path(start_pos: (i64, i64), end_pos: (i64, i64)) -> Vec<char> {
    let y_diff = end_pos.0 - start_pos.0;
    let x_diff = end_pos.1 - start_pos.1;
    let x_char = match x_diff.signum() {
        0 => None,
        1 => Some('>'),
        -1 => Some('<'),
        _ => unreachable!(),
    };
    let x_count = x_diff.abs();
    let y_char = match y_diff.signum() {
        0 => None,
        1 => Some('v'),
        -1 => Some('^'),
        _ => unreachable!(),
    };
    let y_count = y_diff.abs();
    x_char
        .into_iter()
        .flat_map(|c| std::iter::repeat_n(c, x_count as usize))
        .chain(
            y_char
                .into_iter()
                .flat_map(|c| std::iter::repeat_n(c, y_count as usize)),
        )
        .collect()
}

fn char_to_dir(c: char) -> (i64, i64) {
    match c {
        '>' => (0, 1),
        '<' => (0, -1),
        'v' => (1, 0),
        '^' => (-1, 0),
        _ => unreachable!(),
    }
}

fn valid_path(grid: &Grid, start_pos: (i64, i64), path: &[char]) -> bool {
    path.iter()
        .copied()
        .scan(start_pos, |pos, path| {
            *pos = (pos.0 + char_to_dir(path).0, pos.1 + char_to_dir(path).1);
            Some(*pos)
        })
        .all(|pos| grid.contains_key(&pos))
}

fn shortest(
    numpad: &Grid,
    directional: &Grid,
    start: char,
    end: char,
    depth: usize,
    max_depth: usize,
    dp: &mut HashMap<(char, char, usize, usize), usize>,
) -> usize {
    if let Some(res) = dp.get(&(start, end, depth, max_depth)) {
        return *res;
    }
    let grid = if depth == 0 { numpad } else { directional };
    let path = path(find_char(grid, start), find_char(grid, end));
    // println!("{path:?}");
    if depth == max_depth {
        return path.len() + 1; // +A
    }
    let path_len = path.len();
    let mut min_len = usize::MAX;
    for mut perm in path
        .into_iter()
        .permutations(path_len)
        .filter(|path| valid_path(grid, find_char(grid, start), &path))
    {
        perm.insert(0, 'A');
        perm.push('A');
        // println!("perm: {perm:?}");
        let mut sum = 0;
        for (a, b) in perm.into_iter().tuple_windows() {
            sum += shortest(numpad, directional, a, b, depth + 1, max_depth, dp);
        }
        min_len = min_len.min(sum);
    }
    dp.insert((start, end, depth, max_depth), min_len);
    min_len
}

fn part1(input: &str) {
    let mut directional = HashMap::new();
    directional.insert((0, 1), '^');
    directional.insert((0, 2), 'A');
    directional.insert((1, 0), '<');
    directional.insert((1, 1), 'v');
    directional.insert((1, 2), '>');
    let mut numpad: HashMap<(i64, i64), char> = HashMap::new();
    let mut cur = 7;
    for y in 0..=3 {
        if y == 3 {
            numpad.insert((y, 1), '0');
            numpad.insert((y, 2), 'A');
            continue;
        }
        for i in 0..=2 {
            numpad.insert((y, i), char::from_digit((cur + i) as u32, 10).unwrap());
        }
        cur -= 3;
    }
    let mut sum = 0;
    for code in input.lines() {
        let mut len = 0;
        let mut s = String::from("A");
        s.push_str(code);
        let mut dp = HashMap::new();
        for (a, b) in s.chars().tuple_windows() {
            len += shortest(&numpad, &directional, a, b, 0, 25, &mut dp);
        }
        sum += code[0..3].parse::<u64>().unwrap() * len as u64;
    }
    println!("{sum}");
}
fn main() {
    part1(TEST);
    part1(INPUT);
}
