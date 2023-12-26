use std::{
    collections::{HashSet, VecDeque},
    io::{self, Read},
};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let path = part1(&input)?;
    part2(&input, &path)?;

    Ok(())
}

#[derive(PartialEq, Debug, Clone)]
enum Pipe {
    Ground,
    Vertical,
    Horizontal,
    LBend,
    JBend,
    SevenBend,
    FBend,
    Start,
}

// AoC 2023 Day 10 Part 1
fn part1(input: &str) -> Result<HashSet<(u32, u32)>> {
    let mut lines: Vec<Vec<Pipe>> = input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Pipe::Ground,
                    '|' => Pipe::Vertical,
                    '-' => Pipe::Horizontal,
                    'L' => Pipe::LBend,
                    'J' => Pipe::JBend,
                    '7' => Pipe::SevenBend,
                    'F' => Pipe::FBend,
                    'S' => Pipe::Start,
                    _ => panic!("Unknown pipe"),
                })
                .collect()
        })
        .collect();

    print_map(&lines);

    let mut pipe_points = HashSet::new();
    let mut start = (0, 0);
    for row in 0..lines.len() {
        for col in 0..lines.len() {
            if lines[row][col] != Pipe::Ground {
                pipe_points.insert((row as u32, col as u32));
            }
            if lines[row][col] == Pipe::Start {
                start = (row as u32, col as u32);
            }
        }
    }

    // let mut step = 0;
    let mut queue: VecDeque<((u32, u32), u32)> = VecDeque::new();
    queue.push_back((start, 0));
    let mut visited = HashSet::new();
    let mut count = 0;
    let mut path = HashSet::new();
    loop {
        // step += 1;
        if queue.is_empty() {
            println!("No more pipes");
            break;
        }
        let current = queue.pop_front().unwrap();
        path.insert(current.0);
        if current.0 == start && visited.len() > 0 {
            println!("{}", current.1);
            break;
        }
        if visited.contains(&current.0) {
            if queue.is_empty() {
                println!("no more pipes: {}", current.1);
                break;
            }
            continue;
        }

        visited.insert(current.0);
        let dirs: Vec<(u32, u32)> = vec![
            (current.0 .0 as u32 - 1, current.0 .1 as u32),
            (current.0 .0 as u32 + 1, current.0 .1 as u32),
            (current.0 .0 as u32, current.0 .1 as u32 - 1),
            (current.0 .0 as u32, current.0 .1 as u32 + 1),
        ];

        // println!("{:?}", current);

        let neighbors = dirs
            .iter()
            .filter(|dir| pipe_points.contains(*dir))
            .map(|dir| (*dir, current.1 + 1))
            .collect::<Vec<_>>();

        // if neighbors.len() == 2 {
        for n in neighbors {
            let pipe = lines[current.0 .0 as usize][current.0 .1 as usize].clone();
            // println!("pipe: {:?}", pipe);
            match pipe {
                Pipe::Ground => {}
                Pipe::Vertical => {
                    // Up
                    if n.0 .0 == current.0 .0 - 1
                        && n.0 .1 == current.0 .1
                        && !visited.contains(&n.0)
                    {
                        queue.push_back(n);
                    }

                    //Down
                    if n.0 .0 == current.0 .0 + 1
                        && n.0 .1 == current.0 .1
                        && !visited.contains(&n.0)
                    {
                        queue.push_back(n);
                    }
                }
                Pipe::Horizontal => {
                    // Left
                    if n.0 .0 == current.0 .0
                        && n.0 .1 == current.0 .1 - 1
                        && !visited.contains(&n.0)
                    {
                        queue.push_back(n);
                    }

                    // Right
                    if n.0 .0 == current.0 .0
                        && n.0 .1 == current.0 .1 + 1
                        && !visited.contains(&n.0)
                    {
                        queue.push_back(n);
                    }
                }
                Pipe::LBend => {
                    // Up
                    if n.0 .0 == current.0 .0 - 1
                        && n.0 .1 == current.0 .1
                        && !visited.contains(&n.0)
                    {
                        queue.push_back(n);
                    }

                    // Right
                    if n.0 .0 == current.0 .0
                        && n.0 .1 == current.0 .1 + 1
                        && !visited.contains(&n.0)
                    {
                        queue.push_back(n);
                    }
                }
                Pipe::FBend => {
                    // Down
                    if n.0 .0 == current.0 .0 + 1
                        && n.0 .1 == current.0 .1
                        && !visited.contains(&n.0)
                    {
                        queue.push_back(n);
                    }

                    // Right
                    if n.0 .0 == current.0 .0
                        && n.0 .1 == current.0 .1 + 1
                        && !visited.contains(&n.0)
                    {
                        queue.push_back(n);
                    }
                }
                Pipe::JBend => {
                    // Up
                    if n.0 .0 == current.0 .0 - 1
                        && n.0 .1 == current.0 .1
                        && !visited.contains(&n.0)
                    {
                        queue.push_back(n);
                    }

                    // Left
                    if n.0 .0 == current.0 .0
                        && n.0 .1 == current.0 .1 - 1
                        && !visited.contains(&n.0)
                    {
                        queue.push_back(n);
                    }
                }
                Pipe::SevenBend => {
                    // Down
                    if n.0 .0 == current.0 .0 + 1
                        && n.0 .1 == current.0 .1
                        && !visited.contains(&n.0)
                    {
                        queue.push_back(n);
                    }

                    // Left
                    if n.0 .0 == current.0 .0
                        && n.0 .1 == current.0 .1 - 1
                        && !visited.contains(&n.0)
                    {
                        queue.push_back(n);
                    }
                }
                Pipe::Start => {
                    if !visited.contains(&n.0) {
                        let neighbor_pipe = lines[n.0 .0 as usize][n.0 .1 as usize].clone();
                        match neighbor_pipe {
                            Pipe::Ground => {}
                            Pipe::Vertical => {
                                // Up
                                if n.0 .0 - 1 == start.0
                                    && n.0 .1 == start.1
                                    && !visited.contains(&n.0)
                                {
                                    queue.push_back(n);
                                }

                                //Down
                                if n.0 .0 + 1 == start.0
                                    && n.0 .1 == start.1
                                    && !visited.contains(&n.0)
                                {
                                    queue.push_back(n);
                                }
                            }
                            Pipe::Horizontal => {
                                // Left
                                if n.0 .0 == start.0
                                    && n.0 .1 - 1 == start.1
                                    && !visited.contains(&n.0)
                                {
                                    queue.push_back(n);
                                }

                                // Right
                                if n.0 .0 == start.0
                                    && n.0 .1 + 1 == start.1
                                    && !visited.contains(&n.0)
                                {
                                    queue.push_back(n);
                                }
                            }
                            Pipe::FBend => {
                                // Down
                                if n.0 .0 + 1 == start.0
                                    && n.0 .1 == start.1
                                    && !visited.contains(&n.0)
                                {
                                    queue.push_back(n);
                                }

                                // Right
                                if n.0 .0 == start.0
                                    && n.0 .1 + 1 == start.1
                                    && !visited.contains(&n.0)
                                {
                                    queue.push_back(n);
                                }
                            }
                            Pipe::JBend => {
                                // Up
                                if n.0 .0 - 1 == start.0
                                    && n.0 .1 == start.1
                                    && !visited.contains(&n.0)
                                {
                                    queue.push_back(n);
                                }

                                // Left
                                if n.0 .0 == start.0
                                    && n.0 .1 - 1 == start.1
                                    && !visited.contains(&n.0)
                                {
                                    queue.push_back(n);
                                }
                            }
                            Pipe::LBend => {
                                // Up
                                if n.0 .0 - 1 == start.0
                                    && n.0 .1 == start.1
                                    && !visited.contains(&n.0)
                                {
                                    queue.push_back(n);
                                }

                                // Right
                                if n.0 .0 == start.0
                                    && n.0 .1 + 1 == start.1
                                    && !visited.contains(&n.0)
                                {
                                    queue.push_back(n);
                                }
                            }
                            Pipe::SevenBend => {
                                // Down
                                if n.0 .0 + 1 == start.0
                                    && n.0 .1 == start.1
                                    && !visited.contains(&n.0)
                                {
                                    queue.push_back(n);
                                }

                                // Left
                                if n.0 .0 == start.0
                                    && n.0 .1 - 1 == start.1
                                    && !visited.contains(&n.0)
                                {
                                    queue.push_back(n);
                                }
                            }
                            Pipe::Start => {}
                        }

                        // queue.push_back(n);
                    }
                }
            }
            // if !visited.contains(&n.0) {
            // queue.push_back(n);
            // }
        }
        // }
        // println!("{:?}", queue);

        if queue.is_empty() {
            println!("{}", current.1);
            break;
        }

        // if count == 10 {
        //     break;
        // }
        count += 1;
    }

    for (row, line) in lines.iter_mut().enumerate() {
        for (col, pipe) in line.iter().enumerate() {
            if path.contains(&(row as u32, col as u32)) {
                match pipe {
                    Pipe::Ground => print!("."),
                    Pipe::Vertical => print!("|"),
                    Pipe::Horizontal => print!("-"),
                    Pipe::LBend => print!("L"),
                    Pipe::JBend => print!("J"),
                    Pipe::SevenBend => print!("7"),
                    Pipe::FBend => print!("F"),
                    Pipe::Start => print!("S"),
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }

    Ok(path)
}

fn part2(input: &str, path: &HashSet<(u32, u32)>) -> Result<()> {
    Ok(())
}

fn print_map(map: &Vec<Vec<Pipe>>) {
    for line in map {
        for pipe in line {
            match pipe {
                Pipe::Ground => print!("."),
                Pipe::Vertical => print!("|"),
                Pipe::Horizontal => print!("-"),
                Pipe::LBend => print!("L"),
                Pipe::JBend => print!("J"),
                Pipe::SevenBend => print!("7"),
                Pipe::FBend => print!("F"),
                Pipe::Start => print!("S"),
            }
        }
        println!();
    }
}
