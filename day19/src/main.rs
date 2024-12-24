#![feature(iter_from_coroutine, coroutines)]
use std::hash::BuildHasher;

use hashbrown::HashMap;

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
fn both(input: &str) {
    let (pat, input) = input.split_once("\n\n").unwrap();
    let pats = pat.split(", ");

    let mut trie = Trie::new();
    pats.for_each(|pat| trie.insert(pat));

    // let matcher = AhoCorasick::builder().build(&pats).unwrap();
    let input = input.lines();
    let mut count = 0;
    let mut part2 = 0;
    // let mut dp = HashMap::new();
    let mut dp = vec![];
    for input in input {
        dp.clear();
        dp.resize(input.as_bytes().len(), u64::MAX);
        let ways = ways(input, &trie, input.as_ptr() as usize, &mut dp);
        if ways > 0 {
            count += 1;
        }
        part2 += ways;
    }
    println!("{count}\n{part2}");
}

#[derive(Clone)]
struct Node {
    children: Box<[Option<Node>]>,
    terminal: bool,
}

impl Node {
    fn new() -> Node {
        Node {
            children: vec![None; 26].into(),
            terminal: false,
        }
    }
}

struct Trie {
    root: Node,
}

impl Trie {
    fn new() -> Self {
        Trie { root: Node::new() }
    }
    fn insert(&mut self, s: &str) {
        let bytes = s.as_bytes();
        let mut cur_pos = &mut self.root;
        for i in 0..bytes.len() {
            let next = cur_pos.children[(bytes[i] - b'a') as usize].get_or_insert(Node::new());
            if i == bytes.len() - 1 {
                next.terminal = true;
            }
            cur_pos = next;
        }
    }
    fn walk<'a>(&'a self, s: &'a str) -> impl Iterator<Item = usize> + 'a {
        let bytes = s.as_bytes();
        std::iter::from_coroutine(
            #[coroutine]
            || {
                let mut cur_pos = &self.root;
                for i in 0..bytes.len() {
                    if let Some(next) = cur_pos.children[(bytes[i] - b'a') as usize].as_ref() {
                        if next.terminal {
                            yield i + 1;
                        }
                        cur_pos = next;
                    } else {
                        return;
                    }
                }
            },
        )
    }
}

fn ways<'a>(design: &'a str, matcher: &Trie, base: usize, dp: &mut [u64]) -> u64 {
    let addr = design.as_ptr() as *const _ as usize - base;
    if dp[addr] != u64::MAX {
        return dp[addr];
    }
    if design.as_bytes().len() == 0 {
        return 1;
    }
    let mut num_ways = 0;
    for matched in matcher.walk(&design) {
        if matched == design.as_bytes().len() {
            num_ways += 1;
            continue;
        }
        num_ways += ways(&design[matched..], matcher, base, dp);
    }
    dp[addr] = num_ways;

    return num_ways;
}

fn main() {
    let now = std::time::Instant::now();
    both(TEST);
    both(INPUT);
    println!("{:?}", now.elapsed());
}
