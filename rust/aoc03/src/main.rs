use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
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

#[derive(Debug, Default, Clone)]
struct PartNumber {
    num: u32,
    locations: HashSet<(usize, usize)>,
}

impl Display for PartNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

#[derive(Debug, Default, Clone)]
enum CellKind {
    Number(PartNumber),
    Symbol(char, (usize, usize)),
    #[default]
    Empty,
}

impl Display for CellKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CellKind::Number(n) => write!(f, "{}", n),
            CellKind::Symbol(s, _) => write!(f, "{}", s),
            CellKind::Empty => write!(f, "."),
        }
    }
}

// impl CellKind {
//     fn is_symbol(&self) -> bool {
//         match self {
//             CellKind::Symbol(_, _) => true,
//             _ => false,
//         }
//     }

//     fn is_number(&self) -> bool {
//         match self {
//             CellKind::Number(_) => true,
//             _ => false,
//         }
//     }

//     fn locations(&self) -> HashSet<(usize, usize)> {
//         match self {
//             CellKind::Number(n) => n.locations.clone(),
//             _ => HashSet::new(),
//         }
//     }
// }

// impl From<&str> for CellKind {
//     fn from(s: &str) -> Self {
//         if s == "." {
//             CellKind::Empty((0, 0))
//         } else if s.chars().all(|c| c.is_digit(10)) {
//             CellKind::Number(PartNumber {
//                 num: s.parse::<u32>().unwrap(),
//                 ..Default::default()
//             })
//         } else {
//             CellKind::Symbol(s.chars().next().unwrap())
//         }
//     }
// }

// impl From<char> for CellKind {
//     fn from(c: char) -> Self {
//         if c == '.' {
//             return CellKind::Empty;
//         }
//         CellKind::Symbol(c)
//     }
// }

fn part1(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let mut part_numbers = vec![];
    let mut symbols: HashSet<(usize, usize)> = HashSet::new();

    let mut count = 0;
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            for c in l.chars() {
                if !c.is_ascii_digit() && c != '.' {
                    // println!("{}", c);
                    count += 1;
                }
            }
            l.chars().collect()
        })
        .collect();

    println!("{}", count);

    // let mut grid: Vec<Vec<CellKind>> = vec![vec![]];
    for (r, l) in lines.iter().enumerate() {
        // let mut row = vec![];
        let mut num = String::new();
        let mut part_number = PartNumber::default();
        for (col, c) in l.chars().enumerate() {
            if c.is_ascii_digit() {
                num.push(c);
                part_number.locations.insert((r, col));
                // row.push(c);
            } else {
                if !num.is_empty() {
                    part_number.num = num.parse::<u32>().unwrap();
                    // let cell = CellKind::Number(part_number.clone());
                    let neighbors = neighbors(&grid, r, col, &part_number.locations);
                    part_number.locations.extend(neighbors);
                    part_numbers.push(part_number.clone());
                    // row.push(cell);
                    num.clear();
                    part_number = PartNumber::default();
                }
                if c != '.' {
                    // println!("{}", grid[r][col]);
                    symbols.insert((r, col));
                }
                // row.push(c.into());
            }
        }
        // grid.push(row);
    }

    // println!("parts: {:?}", part_numbers);
    println!("symbols: {:?}", symbols.len());

    let mut sum = 0;
    let mut numbers = HashSet::new();
    for p in part_numbers {
        let i = p.locations.intersection(&symbols).collect::<Vec<_>>();
        // println!("{:?}", i.len());
        if i.len() > 0 {
            sum += p.num;
            if !numbers.insert(p.num) {
                println!("{}: {:?}", p.num, i);
            }
        }
    }

    println!("{}", sum);

    // println!("{:?}", grid);
    // for x in 0..grid.len() {
    //     for y in 0..grid[x].len() {
    //         if grid[x][y].is_symbol() {
    //             println!("({}, {}) -> {}", x, y, grid[x][y]);
    //         }
    //     }
    // }

    // for x in 0..grid.len() {
    //     for y in 0..grid[x].len() {
    //         print!("{}", grid[x][y]);
    //     }
    //     println!();
    // }

    // for row in 0..grid.len() {
    //     for col in 0..grid[row].len() {
    //         if grid[row][col].is_number() && symbol_neighbor(row, col, &grid) {
    //             println!("{:?}", grid[row][col]);
    //         }
    //     }
    // }
    Ok(())
}

fn neighbors(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    locations: &HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let dirs = vec![
        (-1, -1), // up-left
        (-1, 0),  // up
        (-1, 1),  // up-right
        (0, -1),  // left
        (0, 1),   // right
        (1, -1),  // down-left
        (1, 0),   // down
        (1, 1),   // down-right
    ];

    // for x in 0..grid.len() {
    // for y in 0..grid[x].len() {
    // let dirs = dirs.clone();
    let mut neighbors = vec![];
    for l in locations {
        for d in &dirs {
            let dr = l.0 as i32 + d.0;
            let dc = l.1 as i32 + d.1;

            // if dr < 0 || dc < 0 || dr >= grid.len() as i32 || dc >= grid[dr as usize].len() as i32 {
            if dr < 0 || dc < 0 || dr >= grid.len() as i32 || dc >= grid[0].len() as i32 {
                continue;
            }

            neighbors.push((dr as usize, dc as usize));
            // if grid[dr as usize][dc as usize].is_symbol() {
            //     println!("({}, {})  ->  {:?}", dr, dc, grid[dr as usize][dc as usize]);
            //     return true;
            // }
        }
    }
    // }
    // }
    // }

    return neighbors;
}
