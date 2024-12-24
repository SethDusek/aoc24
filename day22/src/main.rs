use std::collections::HashSet;

const TEST: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");
fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}
fn prune(a: u64) -> u64 {
    a % 16777216
}

fn evolve(a: u64) -> u64 {
    let s1 = prune(a ^ (a * 64));
    let s2 = prune(s1 ^ (s1 / 32));
    let s3 = prune(s2 ^ (s2 * 2048));
    s3
}

fn find_diff(diff: &[i64], diff_arr: &[i64]) -> Option<usize> {
    diff_arr
        .windows(4)
        .enumerate()
        .find(|(_, diffs)| *diffs == diff)
        .map(|(i, _)| i + diff.len())
}
fn part1(input: &str) {
    let mut sum = 0;
    let mut all_diffs = vec![];
    let mut all_nums = vec![];
    for mut num in input.lines().map(|l| l.parse::<u64>().unwrap()) {
        let mut diffs = vec![];
        let mut nums = vec![num];
        let mut prev = Some((num as i64) % 10);
        for _ in 0..2000 {
            num = evolve(num);
            if let Some(prev) = prev {
                diffs.push((num % 10) as i64 - prev);
            }
            prev = Some((num % 10) as i64);
            nums.push(num);
        }
        // println!("{diffs:?}");
        all_diffs.push(diffs);
        all_nums.push(nums);
        sum += num;
    }
    // println!(
    //     "{:?}",
    //     // all_nums[0][find_diff(&[-1, -1, 0, 2], &all_diffs[0]).unwrap()] % 10
    // );

    println!("{}", all_diffs[0].len());
    let mut max_sum = 0;
    let mut visited = HashSet::new();
    for secret_diffs in &all_diffs {
        for window in secret_diffs.windows(4) {
            if visited.contains(&window) {
                continue;
            }
            visited.insert(window);
            let mut sum = 0;
            for (i, s2) in all_diffs.iter().enumerate() {
                if let Some(pos) = find_diff(&window, s2) {
                    // println!("{i} {window:?} {}", all_nums[i][pos] % 10);
                    sum += all_nums[i][pos] % 10;
                }
            }
            max_sum = max_sum.max(sum);
        }
        println!("part2 (maybe): {max_sum:?}");
    }
    println!("{sum}");
    println!("part2: {max_sum}");
}
fn main() {
    // part1("123");
    part1(TEST);
    part1(INPUT);
}
