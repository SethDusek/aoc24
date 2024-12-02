const DATA: &'static str = include_str!("../bigboy.txt");
const TEST: &'static str = include_str!("../test.txt");

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let (left, right): (Vec<u64>, Vec<u64>) = input
        .lines()
        .map(|l| l.split_once("   ").unwrap())
        .map(|(l, r)| (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap()))
        .unzip();
    (left, right)
}

// optimized for big boy input
fn both(input: &str) {
    let now = std::time::Instant::now();
    let (mut left, mut right) = parse(input);
    println!("parse: {:?}", now.elapsed());
    left.sort_unstable();
    right.sort_unstable();
    println!(
        "{}",
        left.iter()
            .zip(right.iter())
            .map(|(l, r)| r.abs_diff(*l))
            .sum::<u64>()
    );
    let max = *right.last().unwrap();
    let map = right
        .into_iter()
        .fold(vec![0u32; max as usize], |mut map, v| {
            map[v as usize - 1] += 1;
            map
        });
    // let mut idx = 0;
    // let mut sum = 0u64;
    // binary search thing that would have been faster if there were a lot of duplicates
    // while idx < left.len() {
    //     let val = left[idx];
    //     let next_pos = left[idx..].partition_point(|&v| v <= val) as u64;
    //     sum += val * next_pos * map.get(val as usize - 1).copied().unwrap_or(0) as u64;
    //     idx = idx + next_pos as usize;
    // }
    // println!("{}", sum);
    let now = std::time::Instant::now();
    println!(
        "{}",
        left.into_iter()
            .map(|v| v * map.get(v as usize - 1).copied().unwrap() as u64)
            .sum::<u64>()
    );
    println!("{:?}", now.elapsed());
}
fn main() {
    both(TEST);
    both(DATA);
}
