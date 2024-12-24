use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const TEST: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

fn part1(input: &str) {
    let mut graph = HashMap::new();
    input
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .for_each(|(a, b)| {
            graph.entry(a).or_insert(vec![]).push(b);
            graph.entry(b).or_insert(vec![]).push(a)
        });
    dbg!(graph
        .keys()
        .combinations(3)
        .filter(|c| c.iter().any(|v| v.starts_with('t')))
        .filter(|combination| {
            combination
                .iter()
                .cartesian_product(combination.iter())
                .filter(|(a, b)| a != b)
                .all(|(a, b)| graph[*a].contains(b))
        })
        .map(|mut comb| {
            comb.sort();
            comb
        })
        .unique()
        .count());
    let largest_clique = graph
        .keys()
        .map(|k| max_clique(&graph, k))
        .max_by_key(|clique| clique.len())
        .unwrap();
    println!("{}", largest_clique.into_iter().sorted().join(","));
}

fn max_clique<'a>(graph: &HashMap<&'a str, Vec<&'a str>>, start: &'a str) -> HashSet<&'a str> {
    let mut vertices = HashSet::from([start]);
    for vertex in graph.keys() {
        if vertices.iter().all(|a| graph[a].contains(vertex)) {
            vertices.insert(vertex);
        }
    }
    vertices
}
fn main() {
    part1(TEST);
    part1(INPUT);
}
