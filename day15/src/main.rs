use hashbrown::HashMap;

use itertools::Itertools;

const TEST: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");
fn parse(input: &str) -> (HashMap<(i64, i64), char>, Vec<char>) {
    let (maze, steps) = input.split_once("\n\n").unwrap();
    let maze = maze
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i64, j as i64), c))
        })
        .collect();

    (maze, steps.chars().filter(|c| *c != '\n').collect())
}

fn part1(input: &str) {
    let (mut maze, steps) = parse(input);
    for step in steps {
        let robot_pos = maze.iter().find(|(_, v)| **v == '@').unwrap().0;
        let dir = match step {
            '^' => (-1, 0),
            'v' => (1, 0),
            '>' => (0, 1),
            '<' => (0, -1),
            _ => unreachable!(),
        };
        let mut next = (robot_pos.0 + dir.0, robot_pos.1 + dir.1);
        let mut stack = vec![*robot_pos];
        loop {
            stack.push(next);
            match maze.get(&next) {
                Some('#') => {
                    stack.clear();
                    break;
                }
                Some('.') => break,
                _ => {}
            }
            next = (next.0 + dir.0, next.1 + dir.1);
        }
        stack.iter().rev().tuple_windows().for_each(|(a, b)| {
            let tmp_b = maze.remove(b).unwrap();
            let tmp_a = maze.remove(a).unwrap();
            maze.insert(*b, tmp_a);
            maze.insert(*a, tmp_b);
        })
    }
    println!(
        "{}",
        maze.iter()
            .filter(|(_, v)| **v == 'O')
            .map(|((y, x), _)| 100 * y + x)
            .sum::<i64>()
    );
}

fn parse2(input: &str) -> (HashMap<(i64, i64), char>, Vec<char>) {
    let (maze, steps) = input.split_once("\n\n").unwrap();
    let maze = maze
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .flat_map(|c| match c {
                    '@' => ['@', '.'].into_iter(),
                    'O' => ['[', ']'].into_iter(),
                    c => [c, c].into_iter(),
                })
                .enumerate()
                .map(move |(j, c)| ((i as i64, j as i64), c))
        })
        .collect();

    (maze, steps.chars().filter(|c| *c != '\n').collect())
}

fn is_box(c: char) -> bool {
    c == '[' || c == ']'
}
fn box_pos(pos: (i64, i64), c: char) -> [(i64, i64); 2] {
    match c {
        ']' => [(pos.0, pos.1 - 1), (pos.0, pos.1)],
        '[' => [(pos.0, pos.1), (pos.0, pos.1 + 1)],
        _ => unreachable!(),
    }
}

fn display(maze: &HashMap<(i64, i64), char>) {
    let max_y = maze.keys().map(|(y, _)| *y).max().unwrap();
    let max_x = maze.keys().map(|(_, x)| *x).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            print!("{}", maze[&(y, x)]);
        }
        println!("");
    }
}
fn dfs(
    maze: &mut HashMap<(i64, i64), char>,
    position: (i64, i64),
    dir: (i64, i64),
    r#move: bool,
) -> bool {
    match maze.get(&position) {
        Some('#') => return false,
        Some('.') => return true,
        _ => {}
    }

    let next = (position.0 + dir.0, position.1 + dir.1);
    let res = if is_box(maze[&position]) {
        if dir.1 != 0 {
            let box_pos = box_pos(position, maze[&position]);
            dfs(
                maze,
                (
                    box_pos[((dir.1 + 1) / 2) as usize].0 + dir.0,
                    box_pos[((dir.1 + 1) / 2) as usize].1 + dir.1,
                ),
                dir,
                false,
            )
        } else {
            box_pos(position, maze[&position])
                .into_iter()
                .all(|pos| dfs(maze, (pos.0 + dir.0, pos.1 + dir.1), dir, false))
        }
    } else {
        dfs(maze, next, dir, false)
    };
    if res && r#move {
        if is_box(maze[&position]) {
            if dir.1 != 0 {
                let box_pos = box_pos(position, maze[&position]);
                dfs(
                    maze,
                    (
                        box_pos[((dir.1 + 1) / 2) as usize].0 + dir.0,
                        box_pos[((dir.1 + 1) / 2) as usize].1 + dir.1,
                    ),
                    dir,
                    true,
                )
            } else {
                box_pos(position, maze[&position])
                    .into_iter()
                    .all(|pos| dfs(maze, (pos.0 + dir.0, pos.1 + dir.1), dir, true))
            }
        } else {
            dfs(maze, next, dir, true)
        };
        if is_box(maze[&position]) {
            let mut box_pos = box_pos(position, maze[&position]);
            if dir == (0, 1) {
                box_pos.reverse();
            }
            box_pos.into_iter().for_each(|pos| {
                let [prev, next] = maze.get_many_mut([&pos, &(pos.0 + dir.0, pos.1 + dir.1)]);
                std::mem::swap(prev.unwrap(), next.unwrap());
            })
        } else {
            let [prev, next] =
                maze.get_many_mut([&position, &(position.0 + dir.0, position.1 + dir.1)]);
            std::mem::swap(prev.unwrap(), next.unwrap());
        }
    }
    res
}
fn part2(input: &str) {
    let (mut maze, steps) = parse2(input);
    let original = maze.clone();
    for step in steps {
        display(&maze);
        let robot_pos = *maze.iter().find(|(_, v)| **v == '@').unwrap().0;
        let dir = match step {
            '^' => (-1, 0),
            'v' => (1, 0),
            '>' => (0, 1),
            '<' => (0, -1),
            _ => unreachable!(),
        };
        dfs(&mut maze, robot_pos, dir, true);
        println!("Move {}", step);
    }
    original
        .iter()
        .filter(|(_, v)| **v == '#')
        .for_each(|(k, v)| assert_eq!(maze[k], *v));
    println!(
        "part 2: {}",
        maze.iter()
            .filter(|(_, v)| **v == '[')
            .map(|((y, x), _)| 100 * y + x)
            .sum::<i64>()
    );
}
fn main() {
    part1(TEST);
    part2(TEST);
    part1(INPUT);
    part2(INPUT);
}
