use std::collections::BTreeMap;

use num_bigint::BigUint;
use num_traits::{ConstZero, FromPrimitive};

const INPUT: &str = include_str!("../test.txt");
fn inc_or(map: &mut BTreeMap<BigUint, u64>, v: BigUint, count: u64) {
    map.entry(v).and_modify(|c| *c += count).or_insert(count);
}
fn dec(map: &mut BTreeMap<BigUint, u64>, v: BigUint, count: u64) {
    map.entry(v).and_modify(|c| *c -= count);
}

fn log10(num: &BigUint) -> u32 {
    let mut cloned = num.clone();
    let mut count = 0;
    while cloned != BigUint::ZERO {
        cloned /= BigUint::from_i32(10).unwrap();
        count += 1;
    }
    count
}
fn part1(input: &str, iters: u8) {
    let mut parsed = input
        .split_whitespace()
        .map(|w| w.parse::<BigUint>().unwrap())
        .fold(BTreeMap::new(), |mut map, v| {
            map.entry(v).and_modify(|c| *c += 1).or_insert(1);
            map
        });

    let mut max_num = BigUint::ZERO;
    for i in 0..iters {
        let mut new_map = parsed.clone();
        let count = new_map.remove(&0u32.into()).unwrap_or(0);
        inc_or(&mut new_map, 1u32.into(), count);
        for (k, v) in &parsed {
            if *k == 0u32.into() {
                continue;
            }
            let mut any = false;
            let log = log10(k);
            if log % 2 == 0 {
                any = true;
                let split = (k / 10u64.pow(log / 2), k % 10u64.pow(log / 2));
                dec(&mut new_map, k.clone(), *v);
                inc_or(&mut new_map, split.0, *v);
                inc_or(&mut new_map, split.1, *v);
            }
            if !any {
                dec(&mut new_map, k.clone(), *v);
                inc_or(&mut new_map, k * BigUint::from_u32(2024).unwrap(), *v);
            }
        }
        max_num = max_num.max(new_map.last_key_value().unwrap().0.clone());
        parsed = new_map;
    }
    println!("max = {max_num}");
    println!("{}", parsed.len());
    println!("{}", parsed.into_iter().map(|(k, v)| v).sum::<u64>());
}
fn main() {
    part1(INPUT, 25);
    part1(INPUT, 75);
}
