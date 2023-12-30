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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Plan {
    direction: Direction,
    count: i64,
    hex_code: String,
}

// R 6 (#70c710)
impl From<&str> for Plan {
    fn from(s: &str) -> Self {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let direction = match parts[0].trim() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid direction"),
        };
        let count = parts[1].trim().parse::<i64>().unwrap();
        let hex_str = parts[2].trim().trim_start_matches("(#");
        let hex_code = hex_str.trim().trim_end_matches(")").to_string();
        Self {
            direction,
            count,
            hex_code,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn dir(&self) -> (i64, i64) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let plans: Vec<Plan> = input.trim().lines().map(|l| l.into()).collect();
    println!("{:?}", plans);

    let mut dig: HashSet<(i64, i64)> = HashSet::new();

    let mut current = (0, 0);
    // let first_hex = plans[0].hex_code.clone();
    dig.insert(current);
    for p in plans {
        for _ in 0..p.count {
            match p.direction {
                Direction::Up => current.0 -= 1,
                Direction::Down => current.0 += 1,
                Direction::Left => current.1 -= 1,
                Direction::Right => current.1 += 1,
            }
            dig.insert(current);
        }
    }

    let min_height = dig.iter().map(|(r, _)| r).min().unwrap();
    let height = dig.iter().map(|(r, _)| r).max().unwrap() + 1;
    let width = dig.iter().map(|(_, c)| c).max().unwrap() + 1;

    println!("height: {}, width: {}", height, width);

    // find start
    let mut start = (0, 0);
    for c in 0..width {
        if dig.contains(&(*min_height, c)) {
            start = (*min_height, c);
            break;
        }
    }

    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    queue.push_back((start.0 + 1, start.1 + 1));

    while let Some(pos) = queue.pop_back() {
        if !dig.insert(pos) {
            continue;
        }

        let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        for d in dirs {
            let rx = pos.0 as i64 + d.0;
            let cx = pos.1 as i64 + d.1;
            queue.push_back((rx, cx));
        }
    }

    // for r in 0..height {
    //     for c in 0..width {
    //         if dig.contains(&(r, c)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    let count = dig.len();
    println!("count: {}", count);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let initial_plans: Vec<Plan> = input.trim().lines().map(|l| l.into()).collect();
    let plans = initial_plans
        .iter()
        .map(|p| {
            let last_digit = p.hex_code.chars().last().unwrap();
            let hex_code = p.hex_code.chars().take(5).collect::<String>();
            let hex_value = i64::from_str_radix(&hex_code, 16).unwrap();
            let dir = match last_digit {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!("invalid direction"),
            };
            Plan {
                direction: dir,
                count: hex_value as i64,
                hex_code: p.hex_code.clone(),
            }
        })
        .collect::<Vec<_>>();

    println!("{:?}", plans);

    // let mut dig: HashSet<(i64, i64)> = HashSet::new();

    // let mut current = (0, 0);
    // // // let first_hex = plans[0].hex_code.clone();
    // dig.insert(current);
    // for p in &plans {
    //     for _ in 0..p.count {
    //         match p.direction {
    //             Direction::Up => current.0 -= 1,
    //             Direction::Down => current.0 += 1,
    //             Direction::Left => current.1 -= 1,
    //             Direction::Right => current.1 += 1,
    //         }
    //         dig.insert(current);
    //     }
    // }

    // let min_height = dig.iter().map(|(r, _)| r).min().unwrap();
    // let height = dig.iter().map(|(r, _)| r).max().unwrap() + 1;
    // let width = dig.iter().map(|(_, c)| c).max().unwrap() + 1;

    // println!("height: {}, width: {}", height, width);

    // find start
    // let mut start = (0, 0);
    // for c in 0..width {
    //     if dig.contains(&(*min_height, c)) {
    //         start = (*min_height, c);
    //         break;
    //     }
    // }

    // let points = dig.iter().map(|p| *p).collect::<Vec<(i64, i64)>>();
    // let area = points
    //     .windows(2)
    //     .map(|w| (w[0], w[1]))
    //     .map(|((x1, y1), (x2, y2))| (y1 + y2) * (x2 - x1))
    //     .sum::<i64>()
    //     / 2;

    // println!("area: {}", area);

    // let full_area = area - (dig.len() as i64 / 2) + 1;
    // println!("full area: {}", full_area + dig.len() as i64);
    //
    let mut prev = (0, 0);
    let mut total = 0i64;
    let mut outline = 0;
    for Plan {
        direction,
        count,
        hex_code,
    } in plans
    {
        println!("{:?} {} {}", direction, count, hex_code);
        outline += count;
        let dir = direction.dir();
        let next = (prev.0 + dir.0 * count, prev.1 + dir.1 * count);
        total += (prev.1 * next.0) - (prev.0 * next.1);
        prev = next;
    }

    println!("total: {}", (total / 2) + (outline / 2) + 1);

    // let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    // queue.push_back((start.0 + 1, start.1 + 1));

    // while let Some(pos) = queue.pop_back() {
    //     if !dig.insert(pos) {
    //         continue;
    //     }

    //     let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    //     for d in dirs {
    //         let rx = pos.0 as i64 + d.0;
    //         let cx = pos.1 as i64 + d.1;
    //         queue.push_back((rx, cx));
    //     }
    // }

    // for r in 0..height {
    //     for c in 0..width {
    //         if dig.contains(&(r, c)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    Ok(())
}
