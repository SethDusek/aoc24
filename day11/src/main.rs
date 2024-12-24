use std::collections::HashMap;

const INPUT: &str = include_str!("../test.txt");
fn inc_or(map: &mut HashMap<u64, u64>, v: u64, count: u64) {
    map.entry(v).and_modify(|c| *c += count).or_insert(count);
}
fn dec(map: &mut HashMap<u64, u64>, v: u64, count: u64) {
    map.entry(v).and_modify(|c| *c -= count);
}

fn part1(input: &str, iters: u8) {
    let mut parsed = input
        .split_whitespace()
        .map(|w| w.parse::<u64>().unwrap())
        .fold(HashMap::new(), |mut map, v| {
            map.entry(v).and_modify(|c| *c += 1).or_insert(1);
            map
        });

    for _ in 0..iters {
        let mut new_map = parsed.clone();
        let count = new_map.remove(&0u32.into()).unwrap_or(0);
        inc_or(&mut new_map, 1u32.into(), count);
        for (k, v) in &parsed {
            if *k == 0u32.into() {
                continue;
            }
            let mut any = false;
            let digits = k.checked_ilog10().unwrap() + 1;
            if digits % 2 == 0 {
                any = true;
                let split = (k / 10u64.pow(digits / 2), k % 10u64.pow(digits / 2));
                dec(&mut new_map, *k, *v);
                inc_or(&mut new_map, split.0, *v);
                inc_or(&mut new_map, split.1, *v);
            }
            if !any {
                dec(&mut new_map, *k, *v);
                inc_or(&mut new_map, k * 2024, *v);
            }
        }
        parsed = new_map;
    }
    println!("{}", parsed.into_iter().map(|(k, v)| v).sum::<u64>());
}
fn main() {
    part1(INPUT, 25);
    part1(INPUT, 75);
}
