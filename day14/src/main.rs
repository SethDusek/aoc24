use colored::Colorize;
use image::{ImageBuffer, Rgb, RgbImage};
use std::{
    collections::{BTreeSet, HashMap},
    io::BufRead,
    time::Duration,
};

const TEST: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

fn parse_posvel(input: &str) -> (i64, i64) {
    let split = input.split_once("=").unwrap().1.split_once(",").unwrap();
    (split.0.parse().unwrap(), split.1.parse().unwrap())
}
fn parse(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    input
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(p, v)| (parse_posvel(p), parse_posvel(v)))
        .collect()
}
fn display(robots: &BTreeSet<(i64, i64)>, mod_x: i64, mod_y: i64) {
    for y in 0..mod_y {
        for x in 0..mod_x {
            if robots.contains(&(x, y)) {
                print!("{}", ".".green());
            } else {
                print!(" ")
            }
        }
        println!("");
    }
}
fn save_image(idx: usize, robots: &BTreeSet<(i64, i64)>, mod_x: i64, mod_y: i64) {
    let mut buf = RgbImage::new(mod_x as u32, mod_y as u32);
    for y in 0..mod_y {
        for x in 0..mod_x {
            let color = if robots.contains(&(x, y)) {
                Rgb([0, 255, 0])
            } else {
                Rgb([0, 0, 0])
            };
            buf.put_pixel(x as u32, y as u32, color);
        }
    }
    buf.save(format!("images/image-{idx:04}.png")).unwrap();
}
fn part1(input: &str, mod_x: i64, mod_y: i64) {
    let mut parsed = parse(input);
    let steps = 100;
    for (pos, vel) in parsed.iter_mut() {
        *pos = (
            (pos.0 + vel.0 * steps).rem_euclid(mod_x),
            (pos.1 + vel.1 * steps).rem_euclid(mod_y),
        );
    }
    let mut quads = [[0; 2]; 2];
    for (pos, vel) in &parsed {
        let qx = if pos.0 < mod_x / 2 {
            0
        } else if pos.0 > mod_x / 2 {
            1
        } else {
            2
        };
        let qy = if pos.1 < mod_y / 2 {
            0
        } else if pos.1 > mod_y / 2 {
            1
        } else {
            2
        };
        if let Some(quad) = quads.get_mut(qx).and_then(|q| q.get_mut(qy)) {
            *quad += 1;
        }
    }
    println!("{quads:?}");
    println!("{}", quads.iter().flatten().product::<u64>());
}
fn part2(input: &str, mod_x: i64, mod_y: i64) {
    let mut parsed = parse(input);
    let steps = 100;
    let mut pats: HashMap<BTreeSet<(i64, i64)>, (usize, usize)> = HashMap::new();
    for step in 1..=101 * 103 {
        println!("{step}");
        for (pos, vel) in parsed.iter_mut() {
            *pos = (
                (pos.0 + vel.0).rem_euclid(mod_x),
                (pos.1 + vel.1).rem_euclid(mod_y),
            );
        }
        let set: BTreeSet<_> = parsed.iter().map(|(pos, vel)| *pos).collect();
        save_image(step, &set, mod_x, mod_y);
        // display(&set, mod_x, mod_y);
        // pats.entry(set)
        //     .and_modify(|(_, count)| *count += 1)
        //     .or_insert((step, 1));
        // println!("{step}");
        // std::io::stdin()
        //     .lock()
        //     .read_line(&mut String::new())
        //     .unwrap();
        // break;
    }
    // for (pat, (step, count)) in pats {
    //     if count <= 1 {
    //         continue;
    //     }
    //     display(&pat, mod_x, mod_y);
    //     println!("{step} {count}");
    //     std::io::stdin()
    //         .lock()
    //         .read_line(&mut String::new())
    //         .unwrap();
    // }
}
fn main() {
    part1(TEST, 11, 7);
    part1(INPUT, 101, 103);
    // part2(TEST, 11, 7);
    part2(INPUT, 101, 103);
}
