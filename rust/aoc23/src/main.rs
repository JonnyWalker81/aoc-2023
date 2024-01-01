use std::{
    collections::{HashMap, HashSet, VecDeque},
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MapKind {
    Path,
    Forest,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: (i64, i64),
    steps: i64,
}

fn part1(input: &str) -> Result<()> {
    let mut map: Vec<Vec<MapKind>> = input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => MapKind::Path,
                    '#' => MapKind::Forest,
                    '>' => MapKind::Right,
                    '<' => MapKind::Left,
                    '^' => MapKind::Up,
                    'v' => MapKind::Down,
                    _ => panic!("invalid map char"),
                })
                .collect()
        })
        .collect();

    print_map(&map);

    let start = (0i64, 1i64);
    let end = (map.len() as i64 - 1, map[0].len() as i64 - 2);
    let mut points = vec![start, end];

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == MapKind::Forest {
                continue;
            }

            let mut neighbors = 0;
            let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            for d in dirs {
                let rx = row as i64 + d.0;
                let cx = col as i64 + d.1;
                if is_in_bounds(&map, (rx, cx)) {
                    neighbors += 1;
                }
            }

            if neighbors >= 3 {
                points.push((row as i64, col as i64));
            }
        }
    }

    println!("points: {:?}", points.len());
    // let mut queue: VecDeque<State> = VecDeque::new();
    // queue.push_back(State {
    //     pos: (0, 1),
    //     steps: 0,
    // });
    // let end = (map.len() as i64 - 1, map[0].len() as i64 - 2);
    // let mut visited: HashSet<(i64, i64)> = HashSet::new();

    // while let Some(state) = queue.pop_back() {
    //     visited.insert(state.pos);

    //     println!("state: {:?}", state);

    //     if state.pos == end {
    //         println!("found end: {}", state.steps);
    //         // break;
    //     }

    //     let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    //     'outer: for d in dirs {
    //         match map[state.pos.0 as usize][state.pos.1 as usize] {
    //             MapKind::Up => {
    //                 let rx = state.pos.0 - 1;
    //                 let cx = state.pos.1;
    //                 if is_in_bounds(&map, (rx, cx)) && !visited.contains(&(rx, cx)) {
    //                     let state = State {
    //                         pos: (rx, cx),
    //                         steps: state.steps + 1,
    //                     };
    //                     queue.push_back(state);
    //                     break 'outer;
    //                 }
    //             }
    //             MapKind::Down => {
    //                 let rx = state.pos.0 + 1;
    //                 let cx = state.pos.1;
    //                 if is_in_bounds(&map, (rx, cx)) && !visited.contains(&(rx, cx)) {
    //                     let state = State {
    //                         pos: (rx, cx),
    //                         steps: state.steps + 1,
    //                     };
    //                     queue.push_back(state);
    //                     break 'outer;
    //                 }
    //             }
    //             MapKind::Left => {
    //                 let rx = state.pos.0;
    //                 let cx = state.pos.1 - 1;
    //                 if is_in_bounds(&map, (rx, cx)) && !visited.contains(&(rx, cx)) {
    //                     let state = State {
    //                         pos: (rx, cx),
    //                         steps: state.steps + 1,
    //                     };
    //                     queue.push_back(state);
    //                     break 'outer;
    //                 }
    //             }
    //             MapKind::Right => {
    //                 let rx = state.pos.0;
    //                 let cx = state.pos.1 + 1;
    //                 if is_in_bounds(&map, (rx, cx)) && !visited.contains(&(rx, cx)) {
    //                     let state = State {
    //                         pos: (rx, cx),
    //                         steps: state.steps + 1,
    //                     };
    //                     queue.push_back(state);
    //                     break 'outer;
    //                 }
    //             }
    //             _ => {}
    //         }

    //         let rx = state.pos.0 + d.0;
    //         let cx = state.pos.1 + d.1;

    //         if rx < 0
    //             || cx < 0
    //             || rx >= map.len() as i64
    //             || cx >= map[0].len() as i64
    //             || map[rx as usize][cx as usize] == MapKind::Forest
    //             || visited.contains(&(rx, cx))
    //         {
    //             continue;
    //         }

    //         queue.push_back(State {
    //             pos: (rx, cx),
    //             steps: state.steps + 1,
    //         });
    //     }
    // }

    Ok(())
}

fn is_in_bounds(map: &[Vec<MapKind>], pos: (i64, i64)) -> bool {
    pos.0 >= 0
        && pos.1 >= 0
        && pos.0 < map.len() as i64
        && pos.1 < map[0].len() as i64
        && map[pos.0 as usize][pos.1 as usize] != MapKind::Forest
}

fn print_map(map: &[Vec<MapKind>]) {
    for row in map {
        for col in row {
            match col {
                MapKind::Path => print!("."),
                MapKind::Forest => print!("#"),
                MapKind::Up => print!("^"),
                MapKind::Down => print!("v"),
                MapKind::Left => print!("<"),
                MapKind::Right => print!(">"),
            }
        }
        println!();
    }
}
