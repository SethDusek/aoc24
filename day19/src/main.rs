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
    let pats = pat.split(", ").collect::<Vec<&str>>();

    let mut trie = Trie::new();
    pats.iter().for_each(|pat| trie.insert(pat));

    // let matcher = AhoCorasick::builder().build(&pats).unwrap();
    let input = input.lines();
    let mut count = 0;
    let mut part2 = 0;
    let mut dp = HashMap::new();
    for input in input {
        let ways = ways(input, &trie, &pats, &mut dp);
        if ways > 0 {
            count += 1;
        }
        part2 += ways;
        dp.clear();
    }
    println!("{count}");
    println!("{part2}");
}

#[derive(Clone)]
struct Node {
    children: Vec<Option<Node>>,
    terminal: bool,
}

impl Node {
    fn new() -> Node {
        Node {
            children: vec![None; 26],
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

fn compute_hash<K: std::hash::Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}

fn ways<'a>(design: &'a str, matcher: &Trie, pats: &[&str], dp: &mut HashMap<usize, u64>) -> u64 {
    let addr = design.as_ptr() as *const _ as usize;
    let hash = compute_hash(dp.hasher(), &addr);
    if let Some((_, v)) = dp.raw_entry().from_key_hashed_nocheck(hash, &addr) {
        return *v;
    }
    if design.len() == 0 {
        return 1;
    }
    let mut num_ways = 0;
    for matched in matcher.walk(&design) {
        if matched == design.len() {
            num_ways += 1;
            continue;
        }
        num_ways += ways(&design[matched..], matcher, pats, dp);
    }
    dp.raw_entry_mut()
        .from_key_hashed_nocheck(hash, &addr)
        .or_insert(addr, num_ways);
    return num_ways;
}

fn main() {
    both(TEST);
    both(INPUT);
}
