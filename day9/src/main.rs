use std::{collections::BTreeMap, ops::Range};

use rangemap::RangeMap;

const TEST: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

// #[derive(Debug, PartialEq, Eq, Clone)]
// struct Block {
//     range: Range<usize>,
//     block_id: Option<usize>,
// }
fn part1(input: &str) {
    let input = &input.as_bytes()[0..input.len() - 1];
    let mut id = 0;

    let mut ranges = RangeMap::new();
    let mut range_start = 0;
    for i in 0..input.len() {
        let parsed = (input[i] - b'0') as usize;
        if i % 2 == 0 {
            // ranges.push(Block {
            //     range: range_start..range_start + parsed,
            //     block_id: Some(id),
            // });
            ranges.insert(range_start..range_start + parsed, id);
            id += 1;
        } else {
            // ranges.push(Block {
            //     range: range_start..range_start + parsed,
            //     block_id: None,
            // });
        }
        range_start += parsed;
    }
    loop {
        let Some(gap) = ranges.gaps(&(0..range_start)).next() else {
            break;
        };
        let last = ranges
            .last_range_value()
            .map(|(a, b)| (a.clone(), b.clone()))
            .unwrap();
        if gap.start >= last.0.end {
            break;
        }
        let range_len = last.0.len().min(gap.len());
        let new_range = last.0.start..last.0.end - range_len;
        ranges.remove(last.0.clone());
        ranges.insert(gap.start..gap.start + range_len, last.1);
        if new_range.len() != 0 {
            ranges.insert(new_range, last.1);
        }
    }
    println!(
        "{}",
        ranges
            .into_iter()
            .map(|(r, v)| r.sum::<usize>() * v)
            .sum::<usize>()
    );
}

fn part2(input: &str) {
    let input = &input.as_bytes()[0..input.len() - 1];
    let mut id = 0;

    let mut ranges = RangeMap::new();
    let mut by_id = BTreeMap::new();
    let mut range_start = 0;
    for i in 0..input.len() {
        let parsed = (input[i] - b'0') as usize;
        if i % 2 == 0 {
            ranges.insert(range_start..range_start + parsed, id);
            by_id.insert(id, range_start..range_start + parsed);
            id += 1;
        }
        range_start += parsed;
    }
    loop {
        let Some(last_id) = by_id.pop_last() else {
            break;
        };
        let Some(gap) = ranges
            .gaps(&(0..range_start))
            .take_while(|r| r.end <= last_id.1.start)
            .find(|r| r.len() >= last_id.1.len())
        else {
            continue;
        };
        ranges.remove(last_id.1.clone());
        ranges.insert(gap.start..gap.start + last_id.1.len(), last_id.0);
    }
    println!(
        "{}",
        ranges
            .into_iter()
            .map(|(r, v)| r.sum::<usize>() * v)
            .sum::<usize>()
    );
}
fn main() {
    part1(TEST);
    part1(INPUT);
    part2(TEST);
    part2(INPUT);
}
