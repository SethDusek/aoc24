use std::sync::Arc;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

const TEST: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");
#[derive(Clone, Debug)]
struct State {
    ip: usize,
    a: usize,
    b: usize,
    c: usize,
    program: Vec<usize>,
}

impl State {
    fn decode_operand(&self, operand: usize) -> usize {
        match operand {
            x @ 0..=3 => x,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
    fn tick(&mut self, buf: &mut Vec<usize>) -> Option<()> {
        let cur_instr = self.program.get(self.ip)?;

        let mut jump = true;
        match cur_instr {
            // adv
            0 => {
                let num = self.a;
                let div = dbg!(self.decode_operand(self.program[self.ip + 1]));
                self.a = dbg!(num / 2usize.pow(div as u32));
                dbg!(self.a % 8);
            }
            // bxl
            1 => {
                self.b ^= self.program[self.ip + 1];
            }
            2 => self.b = self.decode_operand(self.program[self.ip + 1]) % 8,
            // jnz
            3 if self.a == 0 => {}
            3 => {
                self.ip = self.program[self.ip + 1];
                jump = false
            }
            // bxc
            4 => self.b = self.b ^ self.c,
            // out
            5 => {
                buf.push(self.decode_operand(self.program[self.ip + 1]) % 8);
            }
            6 => {
                let num = self.a;
                let div = self.decode_operand(self.program[self.ip + 1]);
                self.b = num / 2usize.pow(div as u32);
            }
            7 => {
                let num = self.a;
                let div = self.decode_operand(self.program[self.ip + 1]);
                self.c = num / 2usize.pow(div as u32);
            }
            _ => unreachable!(),
        }
        if jump {
            self.ip += 2;
        }
        Some(())
    }
}
fn parse(input: &str) -> State {
    let (regs, program) = input.split_once("\n\n").unwrap();
    let mut regs = regs
        .lines()
        .map(|r| r.split_once(": ").unwrap().1.parse::<usize>().unwrap());
    let program: Vec<usize> = program
        .split_once(": ")
        .unwrap()
        .1
        .trim_end()
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    State {
        ip: 0,
        a: regs.next().unwrap(),
        b: regs.next().unwrap(),
        c: regs.next().unwrap(),
        program,
    }
}

fn part1(input: &str) {
    let mut state = parse(input);
    state.a = 107416870455451;
    let mut buf = vec![];
    while let Some(_) = state.tick(&mut buf) {}
    println!("{buf:?}");
}
fn part2(input: &str) {
    let mut state = parse(input);
    let mut program = state.program.clone();

    let input_len = state.program.len();
    let loops_min = 8usize.pow((input_len - 1) as u32);
    let loops_max = loops_min * 8 - 1;
    // let mut buf = vec![];
    // for i in dbg!(loops_min)..=dbg!(loops_max) {
    //     let mut state = State {
    //         a: i,
    //         program,
    //         ..state
    //     };
    //     if i % 1_000_000_000 == 0 {
    //         println!("{i}");
    //     }
    //     let mut prev_len = buf.len();
    //     while state.tick(&mut buf).is_some() {
    //         if buf.len() != prev_len && buf[buf.len() - 1] != state.program[buf.len() - 1] {
    //             break;
    //         }
    //         prev_len = buf.len();
    //     }
    //     if buf == state.program {
    //         println!("A = {i}");
    //         break;
    //     }
    //     buf.clear();
    //     program = state.program;
    // }
    // let mut program: Arc<[usize]> = program.into();
    // println!(
    //     "A = {}",
    //     (loops_min..=loops_max)
    //         .into_par_iter()
    //         .map(|i| {
    //             let mut buf = vec![];
    //             let mut state = State {
    //                 a: i,
    //                 program: program.to_vec(),
    //                 ..state
    //             };
    //             let mut prev_len = buf.len();
    //             while state.tick(&mut buf).is_some() {
    //                 if buf.len() != prev_len && buf[buf.len() - 1] != state.program[buf.len() - 1] {
    //                     break;
    //                 }
    //                 prev_len = buf.len();
    //             }
    //             (i, buf)
    //         })
    //         .find_first(|(i, buf)| *buf == &program[..])
    //         .unwrap()
    //         .0
    // );
}

// fn part2_try(input: &str) {
//     let parsed = parse(input);
//     let mut cur = 0;
//     let mut a = 0;
//     while cur < parsed.program.len() {
//         for c in 0..8 {
//             parsed.program[cur] ^
//         }
//     }
// }
fn main() {
    part1(TEST);
    part1(INPUT);
    part2(TEST);
    part2(INPUT);
}
