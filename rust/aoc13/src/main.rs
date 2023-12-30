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

struct Board {
    board: Vec<Vec<CellKind>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CellKind {
    Ash,
    Rocks,
}

impl CellKind {
    fn toggle(&mut self) {
        match self {
            CellKind::Ash => *self = CellKind::Rocks,
            CellKind::Rocks => *self = CellKind::Ash,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Vertical,
    Horizontal,
}

fn part1(input: &str) -> Result<()> {
    let boards: Vec<Board> = input
        .trim()
        .split("\n\n")
        .map(|board| {
            let board = board
                .trim()
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => CellKind::Rocks,
                            '.' => CellKind::Ash,
                            _ => panic!("Unknown cell kind"),
                        })
                        .collect()
                })
                .collect::<Vec<Vec<CellKind>>>();

            // board.insert(0, vec![CellKind::Ash; board[0].len()]);
            // board.push(vec![CellKind::Ash; board[0].len()]);

            // for row in board.iter_mut() {
            //     row.insert(0, CellKind::Rocks);
            //     row.push(CellKind::Rocks);
            // }

            Board { board }
        })
        .collect();

    for b in &boards {
        print_board(&b.board);
    }

    let mut found_reflections = vec![];
    for b in &boards {
        println!("processing board...");
        if let Some((left, right)) = has_vertical_reflection(&b) {
            println!("vertical reflection");
            found_reflections.push((Direction::Vertical, (left + 1, right + 1)));
        } else if let Some((left, right)) = has_horizontal_refelction(&b) {
            println!("Horizontal reflection");
            found_reflections.push((Direction::Horizontal, (left + 1, right + 1)));
        } else {
            print_board(&b.board);
            panic!("No reflection found...");
        }
    }

    println!("Boards: {}", boards.len());
    println!("{:?}", found_reflections);
    println!("{:?}", found_reflections.len());

    let mut left = 0;
    let mut above = 0;
    for (k, (l, r)) in found_reflections {
        match k {
            Direction::Vertical => {
                left += l;
            }
            Direction::Horizontal => {
                above += l;
            }
        }
    }

    let sum = left + 100 * above;
    println!("{}", sum);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut boards: Vec<Board> = input
        .trim()
        .split("\n\n")
        .map(|board| {
            let board = board
                .trim()
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => CellKind::Rocks,
                            '.' => CellKind::Ash,
                            _ => panic!("Unknown cell kind"),
                        })
                        .collect()
                })
                .collect::<Vec<Vec<CellKind>>>();
            Board { board }
        })
        .collect();

    for b in &boards {
        print_board(&b.board);
    }

    let mut found_reflections = vec![];
    for b in boards.iter_mut() {
        println!("processing board...");
        if let Some((left, right)) = has_vertical_reflection(&b) {
            println!("vertical reflection");
            let mut current = (Direction::Vertical, (left + 1, right + 1));
            'search: for row in 0..b.board.len() {
                for cell in 0..b.board[row].len() {
                    println!("before toggling {:?}", b.board[row][cell]);
                    b.board[row][cell].toggle();
                    println!("after toggling {:?}", b.board[row][cell]);
                    if let Some((new_left, new_right)) = has_vertical_reflection(&b) {
                        if (new_left + 1, new_right + 1) != current.1 {
                            println!("Found vertical reflection");
                            current = (Direction::Vertical, (new_left + 1, new_right + 1));
                            break 'search;
                        }
                    } else if let Some((new_left, new_right)) = has_horizontal_refelction(&b) {
                        if (new_left + 1, new_right + 1) != current.1 {
                            println!("Found vertical reflection");
                            current = (Direction::Horizontal, (new_left + 1, new_right + 1));
                            break 'search;
                        }
                    }
                    b.board[row][cell].toggle();
                }
            }
            found_reflections.push(current);
        } else if let Some((left, right)) = has_horizontal_refelction(&b) {
            println!("Horizontal reflection");
            let mut current = (Direction::Horizontal, (left + 1, right + 1));
            'search: for row in 0..b.board.len() {
                for cell in 0..b.board[row].len() {
                    println!("before toggling {:?}", b.board[row][cell]);
                    b.board[row][cell].toggle();
                    println!("after toggling {:?}", b.board[row][cell]);
                    if let Some((new_left, new_right)) = has_vertical_reflection(&b) {
                        if (new_left + 1, new_right + 1) != current.1 {
                            println!("Found vertical reflection");
                            current = (Direction::Vertical, (new_left + 1, new_right + 1));
                            break 'search;
                        }
                    } else if let Some((new_left, new_right)) = has_horizontal_refelction(&b) {
                        if (new_left + 1, new_right + 1) != current.1 {
                            println!("Found vertical reflection");
                            current = (Direction::Horizontal, (new_left + 1, new_right + 1));
                            break 'search;
                        }
                    }
                    b.board[row][cell].toggle();
                }
            }

            found_reflections.push(current);
            // found_reflections.push((Direction::Horizontal, (left + 1, right + 1)));
        } else {
            print_board(&b.board);
            panic!("No reflection found...");
        }
    }

    println!("Boards: {}", boards.len());
    println!("{:?}", found_reflections);
    println!("{:?}", found_reflections.len());

    let mut left = 0;
    let mut above = 0;
    for (k, (l, r)) in found_reflections {
        match k {
            Direction::Vertical => {
                left += l;
            }
            Direction::Horizontal => {
                above += l;
            }
        }
    }

    let sum = left + 100 * above;
    println!("{}", sum);
    Ok(())
}

fn has_horizontal_refelction(b: &Board) -> Option<(usize, usize)> {
    for row in 0..b.board.len() - 1 {
        let mut found_horizontal = true;
        // found_horizontal = b.board[row].iter().eq(b.board[row + 1].iter());
        for cell in 0..b.board[0].len() {
            if b.board[row][cell] != b.board[row + 1][cell] {
                // println!("No horizontal reflection");
                found_horizontal = false;
                break;
            }
        }

        if found_horizontal {
            // println!("Horizontal reflection");
            let left_count = row;
            let right_count = b.board.len() - (row + 1 + 1);
            let limit = left_count.min(right_count);
            // let diff = b.board[0].len() - (cell + 1 + 1);
            // println!(
            //     "Horizontal reflection: row={}, left_count={}, right_count={}, limit={}",
            //     row, left_count, right_count, limit
            // );
            let left_range = (row - limit..row).rev();
            let right_range = row + 2..row + 2 + limit;
            // println!(
            //     "{:?} - {:?}, {}, {}",
            //     left_range,
            //     right_range,
            //     left_range.len(),
            //     right_range.len()
            // );
            let other_zip = left_range
                .clone()
                .zip(right_range.clone())
                .collect::<Vec<_>>();
            // println!("other zip: {:?}", other_zip);
            let mut found = true;
            'outer: for i in left_range.zip(right_range) {
                // println!("horizontal pair: {:?}", i);
                for cell in 0..b.board[row].len() {
                    // println!("{:?} - {:?}", b.board[row][cell], b.board[row][cell + 1]);
                    if b.board[i.0][cell] != b.board[i.1][cell] {
                        // println!("No vertical reflection");
                        // println!("No vertical reflection: {}, {}, {}", i.0, i.1, cell);
                        found = false;
                        break 'outer;
                    }
                }
            }

            if found {
                // println!("Found horizontal reflection");
                // found_reflections.insert((cell, cell + 1));
                return Some((row, row + 1));
            }

            // break;
        }
    }

    None
}

fn has_vertical_reflection(b: &Board) -> Option<(usize, usize)> {
    let mut found_vertical = true;
    for cell in 0..b.board[0].len() - 1 {
        found_vertical = true;
        for row in 0..b.board.len() {
            // println!("{:?} - {:?}", b.board[row][cell], b.board[row][cell + 1]);
            if b.board[row][cell] != b.board[row][cell + 1] {
                // println!("No vertical reflection");
                found_vertical = false;
                break;
            }
        }
        if found_vertical {
            let left_count = cell;
            let right_count = b.board[0].len() - (cell + 1 + 1);
            let limit = left_count.min(right_count);
            // let diff = b.board[0].len() - (cell + 1 + 1);
            // println!(
            //     "Vertical reflection: cell={}, left_count={}, right_count={}, limit={}",
            //     cell, left_count, right_count, limit
            // );
            let left_range = (cell - limit..cell).rev();
            let right_range = cell + 2..cell + 2 + limit;
            // println!(
            //     "{:?} - {:?}, {}, {}",
            //     left_range,
            //     right_range,
            //     left_range.len(),
            //     right_range.len()
            // );
            let mut found = true;
            'outer: for i in left_range.zip(right_range) {
                // println!("pair: {:?}", i);
                for row in 0..b.board.len() {
                    // println!("{:?} - {:?}", b.board[row][cell], b.board[row][cell + 1]);
                    if b.board[row][i.0] != b.board[row][i.1] {
                        // println!("No vertical reflection");
                        // println!("No vertical reflection");
                        found = false;
                        break 'outer;
                    }
                }
            }

            if found {
                // println!("Found vertical reflection");
                // found_reflections.insert((cell, cell + 1));
                return Some((cell, cell + 1));
            }

            // break;
        }
        println!();
    }

    None
}

fn print_board(board: &Vec<Vec<CellKind>>) {
    for row in board.iter() {
        for cell in row.iter() {
            match cell {
                CellKind::Ash => print!("."),
                CellKind::Rocks => print!("#"),
            }
        }
        println!();
    }
    println!();
}
