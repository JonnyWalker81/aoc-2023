use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Read},
};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let height = input.trim().lines().count();
    let width = input.trim().lines().next().unwrap().len();

    let mut start = (0, 0);
    let mut map: HashSet<(usize, usize)> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(j, c)| {
                    if c == '#' {
                        Some((i as usize, j as usize))
                    } else if c == 'S' {
                        start = (i as usize, j as usize);
                        None
                    } else {
                        None
                    }
                })
                .collect::<HashSet<(usize, usize)>>()
        })
        .flatten()
        .collect();

    print_map(&map, height, width);

    println!("start: {:?}", start);

    let steps = step(&map, start);

    print_map(&map, height, width);

    println!("total: {}", steps);

    Ok(())
}

fn step(map: &HashSet<(usize, usize)>, start: (usize, usize)) -> usize {
    let mut new_visited = HashSet::new();
    let mut visited = HashSet::new();
    visited.insert(start);
    for _ in 0..64 {
        new_visited.clear();
        for v in &visited {
            let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            for d in dirs {
                let rx = v.0 as i64 + d.0;
                let cx = v.1 as i64 + d.1;
                if map.contains(&(rx as usize, cx as usize)) {
                    continue;
                }

                new_visited.insert((rx as usize, cx as usize));
            }
        }
        std::mem::swap(&mut visited, &mut new_visited);
    }

    visited.len()
}

fn step2(map: &HashSet<(usize, usize)>, s: (usize, usize), height: usize, width: usize) -> i64 {
    const LOOPS: i64 = 26501365;
    let mut new_visited = HashSet::new();
    let mut visited = HashSet::new();
    let mut start = 0;
    let mut prev_start = 0;
    visited.insert(s);
    let mut values = vec![];
    let remainder = 26501365 % height;
    let mut loop_count = 0;
    while values.len() < 3 {
        loop_count += 1;
        if loop_count >= 26501365 {
            break;
        }
        // for loop_count in 0..26501365 {
        // for loop_count in 0..100 {
        new_visited.clear();
        for v in &visited {
            let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            for d in dirs {
                let rx = v.0 as i64 + d.0;
                let cx = v.1 as i64 + d.1;
                let lookup_rx = (v.0 as i64 + d.0).rem_euclid(height as i64);
                let lookup_cx = (v.1 as i64 + d.1).rem_euclid(width as i64);
                if map.contains(&(lookup_rx as usize, lookup_cx as usize)) {
                    continue;
                }

                new_visited.insert((rx as usize, cx as usize));
            }
        }

        if loop_count >= remainder && (loop_count - remainder) % height == 0 {
            let delta = new_visited.len() as i64 - start;
            let step = [new_visited.len() as i64, delta, delta - prev_start];
            // println!("{}: {}, {}", new_visited.len(), delta, delta - prev_start);
            values.push(step[values.len()]);
            println!("{:?}", values);
            start = new_visited.len() as i64;
            prev_start = delta;
        }
        std::mem::swap(&mut visited, &mut new_visited);
    }

    let a: i64 = values[2] / 2;
    let b: i64 = values[1] - 3 * a;
    let c: i64 = values[0] - a - b;
    println!("a={} b={} c={}", a, b, c);

    let n: i64 = 1 + LOOPS / height as i64;

    a * n * n + b * n + c
}

fn part2(input: &str) -> Result<()> {
    let height = input.trim().lines().count();
    let width = input.trim().lines().next().unwrap().len();

    let mut start = (0, 0);
    let map: HashSet<(usize, usize)> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(j, c)| {
                    if c == '#' {
                        Some((i as usize, j as usize))
                    } else if c == 'S' {
                        start = (i as usize, j as usize);
                        None
                    } else {
                        None
                    }
                })
                .collect::<HashSet<(usize, usize)>>()
        })
        .flatten()
        .collect();

    print_map(&map, height, width);

    println!("start: {:?}", start);

    let steps = step2(&map, start, height, width);
    println!("total: {}", steps);

    Ok(())
}

fn print_map(map: &HashSet<(usize, usize)>, height: usize, width: usize) {
    for row in 0..height {
        for col in 0..width {
            if map.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
