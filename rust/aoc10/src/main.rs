use std::{
    collections::{HashMap, HashSet, VecDeque},
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
    Vertical,   //NorthSouth
    Horizontal, //EastWest
    LBend,      // NorthEast
    JBend,      // NorthWest
    SevenBend,  // SouthWest
    FBend,      // SouthEast
    Start,
}

impl Pipe {
    fn next(&self, dir: Direction) -> Direction {
        // println!("{:?} {:?}", self, dir);
        match (self, dir) {
            // (Pipe::Ground, Direction::North) => dir,
            // (Pipe::Ground, Direction::South) => dir,
            // (Pipe::Ground, Direction::East) => dir,
            // (Pipe::Ground, Direction::West) => dir,
            (Pipe::Vertical, Direction::North) => Direction::North,
            (Pipe::Vertical, Direction::South) => Direction::South,

            (Pipe::Horizontal, Direction::East) => Direction::East,
            (Pipe::Horizontal, Direction::West) => Direction::West,

            (Pipe::LBend, Direction::South) => Direction::East,
            (Pipe::LBend, Direction::West) => Direction::North,

            (Pipe::JBend, Direction::South) => Direction::West,
            (Pipe::JBend, Direction::East) => Direction::North,

            (Pipe::SevenBend, Direction::North) => Direction::West,
            (Pipe::SevenBend, Direction::East) => Direction::South,

            (Pipe::FBend, Direction::North) => Direction::East,
            (Pipe::FBend, Direction::West) => Direction::South,
            _ => panic!("Unknown pipe"),
        }
    }
    fn can_go(&self, dir: Direction) -> bool {
        match self {
            Pipe::Ground => false,
            Pipe::Vertical => matches!(dir, Direction::North | Direction::South),
            Pipe::Horizontal => matches!(dir, Direction::East | Direction::West),
            Pipe::LBend => matches!(dir, Direction::North | Direction::East),
            Pipe::JBend => matches!(dir, Direction::North | Direction::West),
            Pipe::SevenBend => matches!(dir, Direction::South | Direction::West),
            Pipe::FBend => matches!(dir, Direction::South | Direction::East),
            Pipe::Start => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

// AoC 2023 Day 10 Part 1
fn part1(input: &str) -> Result<HashSet<(i64, i64)>> {
    let mut start = (0i64, 0i64);
    let mut lines: HashMap<(i64, i64), Pipe> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(r, l)| {
            l.chars()
                .enumerate()
                .map(|(c, ch)| {
                    (
                        (r as i64, c as i64),
                        match ch {
                            '.' => Pipe::Ground,
                            '|' => Pipe::Vertical,
                            '-' => Pipe::Horizontal,
                            'L' => Pipe::LBend,
                            'J' => Pipe::JBend,
                            '7' => Pipe::SevenBend,
                            'F' => Pipe::FBend,
                            'S' => {
                                start = (r as i64, c as i64);
                                Pipe::Start
                            }
                            _ => panic!("Unknown pipe"),
                        },
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let height = input.trim().lines().count();
    let width = input.trim().lines().next().unwrap().len();
    // print_map(&lines, height, width);

    // let path = attempt1(&mut lines)?;
    let path = attempt2(&mut lines, start)?;
    println!("{:?}", path.len() / 2);

    Ok(path)
}

const DIRS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

const DELTA: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn get(map: &HashMap<(i64, i64), Pipe>, pos: (i64, i64)) -> Pipe {
    if let Some(pipe) = map.get(&pos) {
        pipe.clone()
    } else {
        Pipe::Ground
    }
}

fn attempt2(
    lines: &mut HashMap<(i64, i64), Pipe>,
    start: (i64, i64),
) -> Result<HashSet<(i64, i64)>> {
    let mut path = HashSet::new();

    let north = get(&lines, (start.0 - 1, start.1)).can_go(Direction::South);
    let south = get(&lines, (start.0 + 1, start.1)).can_go(Direction::North);
    let east = get(&lines, (start.0, start.1 + 1)).can_go(Direction::West);
    let west = get(&lines, (start.0, start.1 - 1)).can_go(Direction::East);

    let start_tile = match (north, south, east, west) {
        (true, true, false, false) => Pipe::Vertical,
        (true, false, true, false) => Pipe::LBend,
        (true, false, false, true) => Pipe::JBend,
        (false, true, true, false) => Pipe::FBend,
        (false, true, false, true) => Pipe::SevenBend,
        (false, false, true, true) => Pipe::Horizontal,
        _ => panic!("unknown direction"),
    };

    lines.insert(start, start_tile);
    let mut cur_pos = (start.0 as i64, start.1 as i64);

    let mut cur_tile = get(&lines, cur_pos);
    let mut cur_dir = Direction::West;
    for d in &DIRS {
        if cur_tile.can_go(*d) {
            cur_dir = *d;
            break;
        }
    }

    while path.insert(cur_pos) {
        let delta = DELTA[cur_dir as usize];
        cur_pos = (cur_pos.0 + delta.0, cur_pos.1 + delta.1);
        cur_tile = get(&lines, cur_pos);
        cur_dir = cur_tile.next(cur_dir);
    }

    Ok(path)
}

fn attempt1(lines: &mut Vec<Vec<Pipe>>) -> Result<HashSet<(u32, u32)>> {
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
        // if visited.contains(&current.0) {
        //     if queue.is_empty() {
        //         println!("no more pipes: {}", current.1);
        //         break;
        //     }
        //     continue;
        // }

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

fn part2(input: &str, path: &HashSet<(i64, i64)>) -> Result<()> {
    let mut start = (0i64, 0i64);
    let mut lines: HashMap<(i64, i64), Pipe> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(r, l)| {
            l.chars()
                .enumerate()
                .map(|(c, ch)| {
                    (
                        (r as i64, c as i64),
                        match ch {
                            '.' => Pipe::Ground,
                            '|' => Pipe::Vertical,
                            '-' => Pipe::Horizontal,
                            'L' => Pipe::LBend,
                            'J' => Pipe::JBend,
                            '7' => Pipe::SevenBend,
                            'F' => Pipe::FBend,
                            'S' => {
                                start = (r as i64, c as i64);
                                Pipe::Start
                            }
                            _ => panic!("Unknown pipe"),
                        },
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let height = input.trim().lines().count();
    let width = input.trim().lines().next().unwrap().len();

    print_path(&lines, path, height, width);

    let north = get(&lines, (start.0 - 1, start.1)).can_go(Direction::South);
    let south = get(&lines, (start.0 + 1, start.1)).can_go(Direction::North);
    let east = get(&lines, (start.0, start.1 + 1)).can_go(Direction::West);
    let west = get(&lines, (start.0, start.1 - 1)).can_go(Direction::East);

    let start_tile = match (north, south, east, west) {
        (true, true, false, false) => Pipe::Vertical,
        (true, false, true, false) => Pipe::LBend,
        (true, false, false, true) => Pipe::JBend,
        (false, true, true, false) => Pipe::FBend,
        (false, true, false, true) => Pipe::SevenBend,
        (false, false, true, true) => Pipe::Horizontal,
        _ => panic!("unknown direction"),
    };

    lines.insert(start, start_tile);

    let mut inside = false;
    let mut count = 0;
    for row in 0..height {
        let mut tile = Pipe::Ground;
        for col in 0..width {
            if path.contains(&(row as i64, col as i64)) {
                let ch = get(&lines, (row as i64, col as i64));
                match ch {
                    Pipe::Ground => panic!("Ground"),
                    Pipe::Vertical => inside = !inside, // NorthSouth
                    Pipe::Horizontal => {}              // EastWest
                    Pipe::LBend => tile = ch,           // NorthEast
                    Pipe::FBend => tile = ch,           // SouthEast
                    Pipe::JBend => {
                        // NorthWest
                        if tile == Pipe::FBend {
                            inside = !inside;
                        }
                    }
                    Pipe::SevenBend => {
                        // SouthWest
                        if tile == Pipe::LBend {
                            inside = !inside;
                        }
                    }
                    Pipe::Start => {
                        panic!("Start")
                    }
                }
            } else if inside {
                count += 1;
            }
        }
        // println!();
    }

    println!("{}", count);
    Ok(())
}

fn print_map(map: &HashMap<(i64, i64), Pipe>, height: usize, width: usize) {
    for row in 0..height {
        for col in 0..width {
            if let Some(pipe) = map.get(&(row as i64, col as i64)) {
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
        }
        println!();
    }
}

fn print_path(
    map: &HashMap<(i64, i64), Pipe>,
    path: &HashSet<(i64, i64)>,
    height: usize,
    width: usize,
) {
    for row in 0..height {
        for col in 0..width {
            if let Some(pipe) = map.get(&(row as i64, col as i64)) {
                if path.contains(&(row as i64, col as i64)) {
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
        }
    }
}
