use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{self, Display, Formatter},
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

#[derive(Debug, Clone)]
enum Tile {
    Empty,
    Pipe,
    Dash,
    ForwardSlash,
    BackwardSlash,
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

#[derive(Debug, Clone)]
struct Beam {
    position: (i64, i64),
    direction: Direction,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Pipe => write!(f, "|"),
            Tile::Dash => write!(f, "-"),
            Tile::ForwardSlash => write!(f, "/"),
            Tile::BackwardSlash => write!(f, "\\"),
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let board: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '|' => Tile::Pipe,
                    '-' => Tile::Dash,
                    '/' => Tile::ForwardSlash,
                    '\\' => Tile::BackwardSlash,
                    _ => panic!("unknown tile: {}", c),
                })
                .collect()
        })
        .collect();

    print_board(&board);

    let count = determine_energized_tiles(&board, (0, 0), Direction::Right);
    println!("count: {}", count);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let board: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '|' => Tile::Pipe,
                    '-' => Tile::Dash,
                    '/' => Tile::ForwardSlash,
                    '\\' => Tile::BackwardSlash,
                    _ => panic!("unknown tile: {}", c),
                })
                .collect()
        })
        .collect();

    print_board(&board);

    let mut tile_counts = HashSet::new();
    // top edge
    for s in 0..board[0].len() {
        let start = (0i64, s as i64);
        let count = determine_energized_tiles(&board, start, Direction::Down);
        tile_counts.insert(count);
    }

    // bottom edge
    for s in 0..board[0].len() {
        let start = (board.len() as i64 - 1, s as i64);
        let count = determine_energized_tiles(&board, start, Direction::Up);
        tile_counts.insert(count);
    }

    // left edge
    for s in 0..board.len() {
        let start = (s as i64, 0i64);
        let count = determine_energized_tiles(&board, start, Direction::Right);
        tile_counts.insert(count);
    }

    // right edge
    for s in 0..board.len() {
        let start = (s as i64, board[0].len() as i64 - 1);
        let count = determine_energized_tiles(&board, start, Direction::Left);
        tile_counts.insert(count);
    }

    let max = tile_counts.iter().max().unwrap();
    println!("max: {}", max);

    Ok(())
}

fn determine_energized_tiles(
    board: &Vec<Vec<Tile>>,
    start: (i64, i64),
    start_dir: Direction,
) -> i64 {
    let mut beams: Vec<Beam> = vec![];
    let mut current = start;

    let mut visited = HashSet::new();
    beams.push(Beam {
        position: current,
        direction: start_dir,
    });

    while let Some(beam) = beams.pop() {
        if !visited.insert((beam.position, beam.direction.clone())) {
            continue;
        }
        current = beam.position;
        match board[current.0 as usize][current.1 as usize] {
            Tile::Empty => {
                let next_pos = (
                    current.0 + beam.direction.dir().0,
                    current.1 + beam.direction.dir().1,
                );
                if is_in_bounds(&board, next_pos) {
                    beams.push(Beam {
                        position: next_pos,
                        direction: beam.direction,
                    });
                }
            }
            Tile::Pipe => {
                match beam.direction {
                    Direction::Up => {
                        let next_pos = (current.0 - 1, current.1);
                        if is_in_bounds(&board, next_pos) {
                            beams.push(Beam {
                                position: next_pos,
                                direction: Direction::Up,
                            });
                        }
                    }
                    Direction::Down => {
                        let next_pos = (current.0 + 1, current.1);
                        if is_in_bounds(&board, next_pos) {
                            beams.push(Beam {
                                position: next_pos,
                                direction: Direction::Down,
                            });
                        }
                    }
                    Direction::Left | Direction::Right => {
                        // split
                        let next_pos_top = (current.0 - 1, current.1);
                        if is_in_bounds(&board, next_pos_top) {
                            beams.push(Beam {
                                position: next_pos_top,
                                direction: Direction::Up,
                            });
                        }

                        let next_pos_bottom = (current.0 + 1, current.1);
                        if is_in_bounds(&board, next_pos_bottom) {
                            beams.push(Beam {
                                position: next_pos_bottom,
                                direction: Direction::Down,
                            });
                        }
                    }
                };
            }
            Tile::Dash => {
                match beam.direction {
                    Direction::Left => {
                        let next_pos = (current.0, current.1 - 1);
                        if is_in_bounds(&board, next_pos) {
                            beams.push(Beam {
                                position: next_pos,
                                direction: Direction::Left,
                            });
                        }
                    }
                    Direction::Right => {
                        let next_pos = (current.0, current.1 + 1);
                        if is_in_bounds(&board, next_pos) {
                            beams.push(Beam {
                                position: next_pos,
                                direction: Direction::Right,
                            });
                        }
                    }
                    Direction::Up | Direction::Down => {
                        // split
                        let next_pos_left = (current.0, current.1 - 1);
                        if is_in_bounds(&board, next_pos_left) {
                            beams.push(Beam {
                                position: next_pos_left,
                                direction: Direction::Left,
                            });
                        }

                        let next_pos_right = (current.0, current.1 + 1);
                        if is_in_bounds(&board, next_pos_right) {
                            beams.push(Beam {
                                position: next_pos_right,
                                direction: Direction::Right,
                            });
                        }
                    }
                };
            }
            Tile::ForwardSlash => {
                let next_dir = match beam.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
                let next_pos = (current.0 + next_dir.dir().0, current.1 + next_dir.dir().1);
                if is_in_bounds(&board, next_pos) {
                    beams.push(Beam {
                        position: next_pos,
                        direction: next_dir,
                    });
                }
            }
            Tile::BackwardSlash => {
                let next_dir = match beam.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                let next_pos = (current.0 + next_dir.dir().0, current.1 + next_dir.dir().1);
                if is_in_bounds(&board, next_pos) {
                    beams.push(Beam {
                        position: next_pos,
                        direction: next_dir,
                    });
                }
            }
        };
    }

    let energized = visited
        .iter()
        .map(|(k, _)| *k)
        .collect::<HashSet<(i64, i64)>>();
    energized.len() as i64
}

fn print_board(board: &Vec<Vec<Tile>>) {
    for row in board {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn is_in_bounds(board: &Vec<Vec<Tile>>, pos: (i64, i64)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < board.len() as i64 && pos.1 < board[0].len() as i64
}
