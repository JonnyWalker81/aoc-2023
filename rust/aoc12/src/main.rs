use std::{
    collections::{HashSet, VecDeque},
    io::{self, Read},
};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    // part2(&input, &path)?;

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

    let mut found_horizontal = true;
    let mut found_vertical = true;
    for b in &boards {
        // check vertical reflection
        for cell in 0..b.board[0].len() - 1 {
            found_vertical = true;
            for row in 0..b.board.len() - 1 {
                // println!("{:?} - {:?}", b.board[row][cell], b.board[row][cell + 1]);
                if b.board[row][cell] != b.board[row][cell + 1] {
                    // println!("No vertical reflection");
                    found_vertical = false;
                    break;
                }
            }
            if found_vertical {
                println!("Vertical reflection");
                break;
            }
            println!();
        }

        if found_vertical {
            continue;
        }

        for row in 0..b.board.len() - 1 {
            found_horizontal = true;
            // found_horizontal = b.board[row].iter().eq(b.board[row + 1].iter());
            for cell in 0..b.board[0].len() - 1 {
                if b.board[row][cell] != b.board[row + 1][cell] {
                    // println!("No horizontal reflection");
                    found_horizontal = false;
                    break;
                }
            }

            if found_horizontal {
                println!("Horizontal reflection");
                break;
            }
        }

        if found_horizontal {
            println!("Horizontal reflection");
        }
    }
    Ok(())
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
