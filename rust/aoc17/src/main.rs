use std::{
    collections::{BTreeMap, HashSet, VecDeque},
    io::{self, Read},
};

use pathfinding::directed::dijkstra::{self, dijkstra};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    position: (usize, usize),
    temp: u8,
    prev_dir: Direction,
    dir_count: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

    fn is_backwards(&self, other: &Direction) -> bool {
        match self {
            Direction::Up => *other == Direction::Down,
            Direction::Down => *other == Direction::Up,
            Direction::Left => *other == Direction::Right,
            Direction::Right => *other == Direction::Left,
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let city: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    print_city(&city);

    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(State {
        position: (0, 0),
        temp: city[0][0],
        prev_dir: Direction::Right,
        dir_count: 0,
    });
    println!("start: {:?}", city[0][0]);

    let result = dijkstra(
        &State {
            position: (0, 0),
            temp: 0,
            prev_dir: Direction::Right,
            dir_count: 0,
        },
        |state| {
            let mut neighbors = vec![];
            let dirs = vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ];
            for d in dirs {
                if d.is_backwards(&state.prev_dir) {
                    continue;
                }

                let rx = state.position.0 as i64 + d.dir().0;
                let cx = state.position.1 as i64 + d.dir().1;
                if rx < 0 || cx < 0 || rx >= city.len() as i64 || cx >= city[0].len() as i64 {
                    continue;
                }

                let dir_count = if state.prev_dir == d {
                    state.dir_count + 1
                } else {
                    0
                };

                let neighbor = State {
                    position: (rx as usize, cx as usize),
                    temp: city[rx as usize][cx as usize],
                    prev_dir: d.clone(),
                    dir_count,
                };

                if rx >= 0
                    && cx >= 0
                    && rx < city.len() as i64
                    && cx < city[0].len() as i64
                    && dir_count < 3
                {
                    neighbors.push((neighbor.clone(), neighbor.temp as usize));
                }
            }
            neighbors
        },
        |state| state.position == (city.len() - 1, city[0].len() - 1),
    );

    // println!("result: {:?}", result);

    let count = result
        .unwrap()
        .0
        .iter()
        .map(|s| s.temp as usize)
        .sum::<usize>();
    println!("count: {}", count);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let city: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    print_city(&city);

    let result = dijkstra(
        &State {
            position: (0, 0),
            temp: 0,
            prev_dir: Direction::Right,
            dir_count: 0,
        },
        |state| {
            let mut neighbors = vec![];
            let dirs = vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ];
            for d in dirs {
                if d.is_backwards(&state.prev_dir) {
                    continue;
                }

                let rx = state.position.0 as i64 + d.dir().0;
                let cx = state.position.1 as i64 + d.dir().1;
                if rx < 0 || cx < 0 || rx >= city.len() as i64 || cx >= city[0].len() as i64 {
                    continue;
                }

                // let dir_count = if state.prev_dir == d {
                //     state.dir_count + 1
                // } else {
                //     0
                // };

                if state.prev_dir == d && state.dir_count < 10 {
                    let neighbor = State {
                        position: (rx as usize, cx as usize),
                        temp: city[rx as usize][cx as usize],
                        prev_dir: d.clone(),
                        dir_count: state.dir_count + 1,
                    };
                    neighbors.push((neighbor.clone(), neighbor.temp as usize));
                } else if (state.prev_dir != d && state.dir_count >= 4) || state.position == (0, 0)
                {
                    let neighbor = State {
                        position: (rx as usize, cx as usize),
                        temp: city[rx as usize][cx as usize],
                        prev_dir: d.clone(),
                        dir_count: 1,
                    };
                    neighbors.push((neighbor.clone(), neighbor.temp as usize));
                }
            }
            neighbors
        },
        |state| state.position == (city.len() - 1, city[0].len() - 1),
    );

    // println!("result: {:?}", result);

    let count = result
        .unwrap()
        .0
        .iter()
        .map(|s| s.temp as usize)
        .sum::<usize>();
    println!("count: {}", count);

    Ok(())
}

fn print_city(city: &Vec<Vec<u8>>) {
    for row in city {
        for col in row {
            print!("{}", *col);
        }
        println!();
    }
}
