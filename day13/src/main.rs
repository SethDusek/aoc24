use pyo3::{ffi::c_str, prelude::*, types::PyTuple};

const TEST: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");
fn parse_eq(eq: &str, sep: char) -> [i64; 2] {
    let (a, b) = eq.split_once(", ").unwrap();
    [
        a.split_once(sep).unwrap().1.parse().unwrap(),
        b.split_once(sep).unwrap().1.parse().unwrap(),
    ]
}
fn parse(input: &str) -> Vec<[[i64; 2]; 3]> {
    input
        .split("\n\n")
        .map(|l| {
            let mut l = l.lines();
            let a = parse_eq(l.next().unwrap().split_once(" ").unwrap().1, '+');
            let b = parse_eq(l.next().unwrap().split_once(" ").unwrap().1, '+');
            let prize = parse_eq(l.next().unwrap().split_once(" ").unwrap().1, '=');
            [a, b, prize]
        })
        .collect()
}
fn part1(input: &str) {
    let parsed = parse(input);
    let mut sum = 0;
    for eq in parsed {
        let mut min = None;
        for i in 0..=100i64 {
            let [x, y] = [i * eq[0][0], i * eq[0][1]];
            let sum = [eq[2][0] - x, eq[2][1] - y];
            if sum[0] < 0 || sum[1] < 0 {
                continue;
            }
            if sum[0] % eq[1][0] == 0 && sum[1] == eq[1][1] * (sum[0] / eq[1][0]) {
                let tokens = 3 * i + sum[0] / eq[1][0];
                min = min.map(|x| std::cmp::min(x, tokens)).or(Some(tokens));
            }
        }
        sum += min.unwrap_or(0);
    }
    println!("{sum}");
}

fn part2(input: &str) {
    let parsed = parse(input);
    let mut sum = 0;
    for mut eq in parsed {
        eq[2][0] += 10000000000000;
        eq[2][1] += 10000000000000;
        let sol = solve(eq);
        if sol.len() != 0 {
            sum += sol[0] * 3 + sol[1];
        }
    }
    println!("{sum}");
}
fn solve(args: [[i64; 2]; 3]) -> Vec<i64> {
    Python::with_gil(|py| {
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            &c_str!(include_str!("../solve.py")),
            c_str!(""),
            c_str!(""),
        )
        .unwrap()
        .getattr("solve_sol")
        .unwrap()
        .into();
        let args = PyTuple::new(
            py,
            &[
                args[0][0], args[0][1], args[1][0], args[1][1], args[2][0], args[2][1],
            ],
        )
        .unwrap();
        let res: Vec<i64> = fun.call1(py, args).unwrap().extract(py).unwrap();
        res
    })
}
fn main() {
    part1(TEST);
    part1(INPUT);
    part2(TEST);
    part2(INPUT);
}
