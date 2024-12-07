use std::collections::HashMap;

const DATA: &str = include_str!("../data.txt");
const TEST: &str = include_str!("../test.txt");

type Ways = Vec<HashMap<usize, usize>>;
fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(target, nums)| {
            (
                target.parse::<usize>().unwrap(),
                nums.split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn concat(a: usize, b: usize) -> usize {
    let pow = b.checked_ilog10().unwrap_or(0) + 1;
    a * 10usize.pow(pow) + b
}
fn ways(target: usize, arr: &[usize], cur_num: usize) -> usize {
    if arr.len() == 0 && cur_num == target {
        1
    } else if arr.len() == 0 {
        return 0;
    } else {
        // let num = arr.last().unwrap();
        let mut num_ways = if let Some(next) = target.checked_sub(cur_num) {
            ways(next, &arr[0..arr.len() - 1], *arr.last().unwrap())
        } else {
            0
        };
        if target % cur_num == 0 {
            num_ways += ways(
                target / cur_num,
                &arr[0..arr.len() - 1],
                *arr.last().unwrap(),
            );
        }
        if arr.len() >= 1 {
            num_ways += ways(
                target,
                &arr[0..arr.len() - 1],
                dbg!(concat(*dbg!(arr.last().unwrap()), dbg!(cur_num))),
            );
        }
        // if arr.len() > 1 {
        //     let pow = arr[arr.len() - 1].checked_ilog10().unwrap_or(0) + 1;
        //     arr[arr.len() - 2]
        // }
        num_ways
    }
}

fn part1(input: &str) {
    let parsed = parse(input);
    let mut sum = 0;
    for (target, nums) in parsed {
        let num_ways = ways(target, &nums[0..nums.len() - 1], *nums.last().unwrap());
        if num_ways > 0 {
            println!("{target} {num_ways}");
            sum += target;
        }
    }
    println!("{sum}");
}
fn main() {
    part1(TEST);
    // part1(DATA);
}
