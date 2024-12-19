use std::hash::BuildHasher;

use hashbrown::{
    hash_map::{Entry, OccupiedEntry},
    HashMap, HashSet,
};

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
fn part1(input: &str) {
    let (pat, input) = input.split_once("\n\n").unwrap();
    let pats = pat.split(", ").collect::<Vec<&str>>();
    let input = input.lines();
    let mut count = 0;
    let mut part2 = 0;
    let mut hs = HashMap::new();
    let mut dp = HashMap::new();
    for input in input {
        if possible(input, &pats, &mut hs) {
            // println!("{input} possible");
            count += 1;
        } else {
            // println!("{input} impossible");
        }
        // println!("{input} ways: {}", ways(input, &pats, &mut dp));
        part2 += ways(input, &pats, &mut dp);
    }
    println!("{count}");
    println!("{part2}");
}

fn compute_hash<K: std::hash::Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}
fn possible<'a>(design: &'a str, pats: &[&str], dp: &mut HashMap<&'a str, bool>) -> bool {
    let hash = compute_hash(dp.hasher(), design);
    if let Some((_, v)) = dp.raw_entry().from_key_hashed_nocheck(hash, design) {
        return *v;
    }
    if let Some(res) = dp.get(design) {
        return *res;
    }
    if design.len() == 0 {
        return true;
    }
    let mut any = false;
    for pat in pats {
        if design[0..pat.len().min(design.len())] == **pat {
            if pat.len() == design.len() {
                any = true;
                break;
            }
            any |= possible(&design[pat.len()..], pats, dp);
            if any {
                break;
            }
        }
    }
    dp.raw_entry_mut()
        .from_key_hashed_nocheck(hash, design)
        .or_insert(design, any);
    any
}
fn ways<'a>(design: &'a str, pats: &[&str], dp: &mut HashMap<&'a str, u64>) -> u64 {
    let hash = compute_hash(dp.hasher(), design);
    if let Some((_, v)) = dp.raw_entry().from_key_hashed_nocheck(hash, design) {
        return *v;
    }
    if design.len() == 0 {
        return 1;
    }
    let mut num_ways = 0;
    for pat in pats {
        if design[0..pat.len().min(design.len())] == **pat {
            // println!("part2: {} {design} matched {pat}", design.as_ptr() as usize);
            if pat.len() == design.len() {
                num_ways += 1;
                continue;
            }
            num_ways += ways(&design[pat.len()..], pats, dp);
        }
    }
    dp.raw_entry_mut()
        .from_key_hashed_nocheck(hash, design)
        .or_insert(design, num_ways);
    // dp.insert(design, num_ways);
    return num_ways;
}
fn main() {
    part1(TEST);
    part1(INPUT);
}
