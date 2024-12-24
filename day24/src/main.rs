use hashbrown::{HashMap, HashSet};
use std::str::FromStr;

use itertools::Itertools;

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

fn solve<'a>(
    val: &'a str,
    values: &mut HashMap<String, u8>,
    circuit: &HashMap<String, Gate>,
) -> Option<u8> {
    if let Some(bit) = values.get(val) {
        return Some(*bit);
    }
    let gate = circuit.get(val)?;
    let res = match gate.op {
        Type::Xor => solve(&gate.a, values, circuit)? ^ solve(&gate.b, values, circuit)?,
        Type::And => solve(&gate.a, values, circuit)? & solve(&gate.b, values, circuit)?,
        Type::Or => solve(&gate.a, values, circuit)? | solve(&gate.b, values, circuit)?,
    };
    values.insert(val.to_owned(), res);
    Some(res)
}

#[derive(PartialEq, Clone, Copy, Eq, Debug)]
enum Type {
    And,
    Or,
    Xor,
}
impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "AND" => Self::And,
            "XOR" => Self::Xor,
            "OR" => Self::Or,
            _ => return Err(()),
        })
    }
}
#[derive(Debug, Clone)]
struct Gate {
    a: String,
    b: String,
    op: Type,
}

impl PartialEq for Gate {
    fn eq(&self, other: &Self) -> bool {
        ((self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a))
            && self.op == other.op
    }
}
fn and(a: &str, b: &str) -> Gate {
    Gate {
        a: a.to_owned(),
        b: b.to_owned(),
        op: Type::And,
    }
}
fn xor(a: &str, b: &str) -> Gate {
    Gate {
        a: a.to_owned(),
        b: b.to_owned(),
        op: Type::Xor,
    }
}
fn or(a: &str, b: &str) -> Gate {
    Gate {
        a: a.to_owned(),
        b: b.to_owned(),
        op: Type::Or,
    }
}

fn gate_name(prefix: &str, i: usize) -> String {
    format!("{prefix}{i:02}")
}
fn find_gate(circuit: &HashMap<String, Gate>, gate: &Gate) -> Option<String> {
    circuit
        .iter()
        .find(|(_, v)| *v == gate)
        .map(|(k, _)| k.to_owned())
}

fn needs_swap(
    circuit: &mut HashMap<String, Gate>,
    gate: &Gate,
    swaps: &mut HashSet<String>,
) -> bool {
    let res = find_gate(circuit, gate);
    if let Some(_) = res {
        return false;
    }
    // found a gate where one of the inputs matches but other doesnt, give input wire that doesnt match
    let partial = circuit
        .iter()
        .find(|(_, v)| {
            v.op == gate.op && (gate.a == v.a || gate.a == v.b || gate.b == v.a || gate.b == v.b)
        })
        .map(|(_, res)| {
            if gate.a == res.a {
                (gate.b.clone(), res.b.clone())
            } else if gate.b == res.b {
                (gate.a.clone(), res.a.clone())
            } else if gate.a == res.b {
                (gate.b.clone(), res.a.clone())
            } else if gate.b == res.a {
                (gate.a.clone(), res.b.clone())
            } else {
                unreachable!() // gud for debugging
            }
        })
        .unwrap();
    println!("partial: Swapping {} and {}", partial.0, partial.1);
    swaps.insert(partial.0.clone());
    swaps.insert(partial.1.clone());
    let mut many = circuit.get_many_mut([&partial.0, &partial.1]);
    std::mem::swap(many[0].take().unwrap(), many[1].take().unwrap());
    true
}
fn swap_wire(circuit: &mut HashMap<String, Gate>, a: &str, b: &str, swaps: &mut HashSet<String>) {
    swaps.insert(a.to_owned());
    swaps.insert(b.to_owned());
    let [a, b] = circuit.get_many_mut([a, b]);
    std::mem::swap(a.unwrap(), b.unwrap());
}
// return carry
// all this only works because i manually went in and swapped fsq and dvb in code
fn find_adder(
    z_i: usize,
    circuit: &mut HashMap<String, Gate>,
    carry_in: Option<String>,
    swaps: &mut HashSet<String>,
) -> Option<String> {
    let output_wire = gate_name("z", z_i);
    let half_add_gate = xor(&gate_name("x", z_i), &gate_name("y", z_i));
    let half_carry_gate = and(&gate_name("x", z_i), &gate_name("y", z_i));
    if let Some(carry_in) = carry_in {
        if carry_in == output_wire {
            return None;
        }
        let add_gate = xor(&find_gate(&circuit, &half_add_gate).unwrap(), &carry_in);
        let add_wire = if needs_swap(circuit, &add_gate, swaps) {
            // if add gate was swapped restart the whole computation
            return find_adder(z_i, circuit, Some(carry_in), swaps);
        } else {
            find_gate(&circuit, &add_gate).expect("add gate not found")
        };
        if add_wire != output_wire {
            println!(
                "oops {output_wire}  actual circuit: {:?}, expected: {add_gate:?}",
                circuit[&output_wire]
            );
            swap_wire(circuit, &add_wire, &output_wire, swaps);
        }
        let other_half_carry = and(&find_gate(&circuit, &half_add_gate).unwrap(), &carry_in); // (xi xor yi) AND ci
        let other_half_carry_wire = &find_gate(circuit, &other_half_carry).unwrap();
        return Some(
            find_gate(
                circuit,
                &or(
                    &find_gate(&circuit, &half_carry_gate).unwrap(),
                    other_half_carry_wire,
                ),
            )
            .unwrap(),
        );
    }
    println!(
        "{half_add_gate:?}, {:?}",
        find_gate(&circuit, &half_add_gate)
    );
    println!("{:?} {output_wire}", circuit[&output_wire]);
    find_gate(&circuit, &half_carry_gate)
}

fn part1(input: &str) {
    let split_once = input.split_once("\n\n").unwrap();
    let (init, circuit) = split_once;
    let values = init
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(a, b)| (a.to_owned(), b.parse::<u8>().unwrap()))
        .collect::<HashMap<_, _>>();
    let mut circuit = circuit
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(a, b)| -> (String, (String, String, String)) {
            (
                b.to_owned(),
                a.split(" ").map(String::from).collect_tuple().unwrap(),
            )
        })
        .map(|(a, b)| {
            (
                a,
                Gate {
                    a: b.0,
                    b: b.2,
                    op: Type::from_str(&b.1).unwrap(),
                },
            )
        })
        .collect::<HashMap<String, Gate>>();
    let res = simulate(values, &circuit);
    println!("{res}");
    let mut swaps = HashSet::new();
    let mut prev_carry = None;
    for i in 0..48 {
        prev_carry = find_adder(i, &mut circuit, prev_carry.take(), &mut swaps);
        if prev_carry.is_none() {
            break;
        }
    }
    println!("{} {:?}", swaps.len(), swaps.iter().sorted().join(","));
}

fn simulate(mut values: HashMap<String, u8>, circuit: &HashMap<String, Gate>) -> u64 {
    let mut bv = vec![];
    let mut z = 0;
    loop {
        let z_str = format!("z{z:02}");
        // println!("{z_str}");
        if let Some(res) = solve(&z_str, &mut values, &circuit) {
            bv.push(res);
        } else {
            break;
        }
        z += 1;
    }
    let res = bv.into_iter().rev().fold(0u64, |a, b| (a << 1) | b as u64);
    res
}
fn main() {
    part1(INPUT);
    part1(TEST);
}
