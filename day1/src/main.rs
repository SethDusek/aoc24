use core::str;

const DATA: &'static str = include_str!("../bigboy.txt");
const TEST: &'static str = include_str!("../test.txt");

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let (left, right): (Vec<u64>, Vec<u64>) = input
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(l, r)| {
            (
                l.parse::<u64>().unwrap(),
                r.trim_start().parse::<u64>().unwrap(),
            )
        })
        .unzip();
    (left, right)
}

const MUL_TABLE: [u64; 11] = [
    10u64.pow(0),
    10u64.pow(1),
    10u64.pow(2),
    10u64.pow(3),
    10u64.pow(4),
    10u64.pow(5),
    10u64.pow(6),
    10u64.pow(7),
    10u64.pow(8),
    10u64.pow(9),
    10u64.pow(10),
];
fn parse_num(input: &[u8]) -> u64 {
    input
        .iter()
        .rev()
        .enumerate()
        .fold(0u64, |res, (i, b)| res + (b - b'0') as u64 * MUL_TABLE[i])
}

fn parse_fast(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut input = input.as_bytes();
    let digits = input.iter().position(|&c| c == b' ').unwrap();
    let whitespaces = input[digits..].iter().take_while(|&&c| c == b' ').count();
    let mut a = vec![];
    let mut b = vec![];
    while !input.is_empty() {
        let num = parse_num(&input[..digits]);
        let num2 = parse_num(&input[whitespaces + digits..whitespaces + 2 * digits]);
        a.push(num);
        b.push(num2);
        input = &input[whitespaces + 2 * digits..];
        if !input.is_empty() && input[0] == b'\n' {
            input = &input[1..];
        }
    }
    (a, b)
}
// optimized for big boy input
fn both(input: &str) {
    let (mut left, mut right) = parse_fast(input);
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
    // let now = std::time::Instant::now();
    println!(
        "{}",
        left.into_iter()
            .map(|v| v * map.get(v as usize - 1).copied().unwrap() as u64)
            .sum::<u64>()
    );
}
fn main() {
    both(TEST);
    both(DATA);
}
