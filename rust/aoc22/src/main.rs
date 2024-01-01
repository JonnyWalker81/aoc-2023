use std::{
    collections::HashMap,
    io::{self, Read},
};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    // part2(&input)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let parts = s.split(',').collect::<Vec<_>>();
        let x = parts[0].parse::<i64>().unwrap();
        let y = parts[1].parse::<i64>().unwrap();
        let z = parts[2].parse::<i64>().unwrap();
        Self { x, y, z }
    }
}

#[derive(Debug, Clone)]
struct Brick {
    id: usize,
    left: Point,
    right: Point,
}

fn part1(input: &str) -> Result<()> {
    let mut snapshot: Vec<Brick> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let (lhs, rhs) = l.split_once('~').unwrap();
            Brick {
                id: i,
                left: lhs.into(),
                right: rhs.into(),
            }
        })
        .collect();

    snapshot.sort_by(|a, b| a.left.z.cmp(&b.left.z));

    println!("{:?}", snapshot);

    Ok(())
}
