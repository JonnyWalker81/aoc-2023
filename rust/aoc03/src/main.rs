use std::{
    collections::{BTreeMap, HashSet},
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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct PartNumber {
    points: HashSet<(i64, i64)>,
    value: i64,
}

impl PartNumber {
    fn new(row: i64, col: i64, ch: char) -> Self {
        let mut points = HashSet::from([
            (row - 1, col - 1),
            (row, col - 1),
            (row + 1, col - 1),
            (row - 1, col),
            (row + 1, col),
            (row - 1, col + 1),
            (row, col + 1),
            (row + 1, col + 1),
        ]);
        Self {
            points,
            value: (ch as u8 - b'0') as i64,
        }
    }

    fn add_digit(&mut self, row: i64, col: i64, ch: char) {
        self.value *= 10;
        self.value += (ch as u8 - b'0') as i64;
        self.points
            .extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)]);
    }
}

impl Display for PartNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.points)
    }
}

#[derive(Debug, Default, Clone)]
enum CellKind {
    Number(u32),
    Symbol(char),
    #[default]
    Empty,
}

impl Display for CellKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CellKind::Number(n) => write!(f, "{}", n),
            CellKind::Symbol(s) => write!(f, "{}", s),
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
    let mut symbols: HashSet<(i64, i64)> = HashSet::new();

    let mut cur_number: Option<PartNumber> = None;
    for (r, line) in lines.iter().enumerate() {
        let mut num = String::new();
        let mut part_number = PartNumber::default();
        for (c, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if let Some(ref mut num) = cur_number {
                    num.add_digit(r as i64, c as i64, ch);
                } else {
                    cur_number = Some(PartNumber::new(r as i64, c as i64, ch));
                }
            } else {
                if let Some(num) = cur_number.take() {
                    part_numbers.push(num);
                    cur_number = None;
                }
                if ch != '.' {
                    symbols.insert((r as i64, c as i64));
                }
            }
        }
    }

    // println!("symbols: {:?}", part_numbers);

    let mut sum = 0;
    for p in part_numbers {
        let i = p.points.intersection(&symbols).collect::<Vec<_>>();
        // println!("{:?}", i.len());
        if i.len() > 0 {
            sum += p.value;
            // if !numbers.insert(p.num) {
            // println!("{}: {:?}", p.num, i);
            // }
        }
    }

    println!("{}", sum);

    // let grid = lines
    //     .iter()
    //     .enumerate()
    //     .flat_map(|(r, l)| {
    //         l.chars().enumerate().map(move |(c, ch)| {
    //             (
    //                 (r, c),
    //                 match ch {
    //                     '.' => CellKind::Empty,
    //                     cc if cc.is_ascii_digit() => {
    //                         CellKind::Number(cc.to_digit(10).unwrap() as u32)
    //                     }
    //                     cc => CellKind::Symbol(cc),
    //                 },
    //             )
    //         })
    //     })
    //     .collect::<BTreeMap<(usize, usize), CellKind>>();

    // // println!("{:?}", grid);
    // let mut numbers = HashSet::new();

    // for ((r, c), cell) in &grid {
    //     match cell {
    //         CellKind::Number(_) => {
    //             numbers.insert((*r, *c));
    //         }
    //         CellKind::Symbol(_) => {
    //             symbols.insert((*r, *c));
    //         }
    //         _ => {}
    //     }
    // }

    // println!("{:?}", numbers.len());
    // println!("{:?}", symbols.len());
    // let i = numbers.intersection(&symbols).collect::<Vec<_>>();
    // println!("{:?}", i.len());

    // let mut numbers = vec![];
    // for ((r, c), cell) in grid.iter() {
    //     if let CellKind::Number(n) = cell {
    //         // println!("({}, {}) -> {}", r, c, n);
    //         match numbers.iter().last() {
    //             Some(v) => {
    //                 let last_num = v.iter().last();
    //                 match last_num {
    //                     Some(((last_num_x, _), _)) => todo!(),
    //                     None => todo!(),
    //                 }
    //             }
    //             None => {
    //                 numbers.push(vec![((*r, *c), *n)]);
    //             }
    //         }
    //         numbers.push((r, c, n));
    //     }
    // }

    // let mut grid: Vec<Vec<CellKind>> = vec![vec![]];
    // for (r, l) in lines.iter().enumerate() {
    //     // let mut row = vec![];
    //     let mut num = String::new();
    //     let mut part_number = PartNumber::default();
    //     for (col, c) in l.chars().enumerate() {
    //         if c.is_ascii_digit() {
    //             num.push(c);
    //             part_number.locations.insert((r, col));
    //             // row.push(c);
    //         } else {
    //             if !num.is_empty() {
    //                 part_number.num = num.parse::<u32>().unwrap();
    //                 // let cell = CellKind::Number(part_number.clone());
    //                 let neighbors = neighbors(&grid, r, col, &part_number.locations);
    //                 part_number.locations.extend(neighbors);
    //                 part_numbers.push(part_number.clone());
    //                 // row.push(cell);
    //                 num.clear();
    //                 part_number = PartNumber::default();
    //             }
    //             if c != '.' {
    //                 // println!("{}", grid[r][col]);
    //                 symbols.insert((r, col));
    //             }
    //             // row.push(c.into());
    //         }
    //     }
    //     // grid.push(row);
    // }

    // // println!("parts: {:?}", part_numbers);
    // println!("symbols: {:?}", symbols.len());

    // let mut sum = 0;
    // let mut numbers = HashSet::new();
    // for p in part_numbers {
    //     let i = p.locations.intersection(&symbols).collect::<Vec<_>>();
    //     // println!("{:?}", i.len());
    //     if i.len() > 0 {
    //         sum += p.num;
    //         if !numbers.insert(p.num) {
    //             println!("{}: {:?}", p.num, i);
    //         }
    //     }
    // }

    // println!("{}", sum);

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

fn part2(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let mut part_numbers = vec![];
    let mut symbols: HashSet<(i64, i64, char)> = HashSet::new();

    let mut cur_number: Option<PartNumber> = None;
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if let Some(ref mut num) = cur_number {
                    num.add_digit(r as i64, c as i64, ch);
                } else {
                    cur_number = Some(PartNumber::new(r as i64, c as i64, ch));
                }
            } else {
                if let Some(num) = cur_number.take() {
                    part_numbers.push(num);
                    cur_number = None;
                }
                if ch != '.' {
                    symbols.insert((r as i64, c as i64, ch));
                }
            }
        }
    }

    let gears = symbols
        .iter()
        .filter(|(r, c, ch)| *ch == '*')
        .map(|(r, c, _)| (*r, *c))
        .collect::<HashSet<_>>();

    let mut sum = 0;
    for g in gears {
        let mut found_gears = Vec::new();
        for p in &part_numbers {
            let g = HashSet::from([g]);
            let i = p.points.intersection(&g).collect::<Vec<_>>();

            if i.len() > 0 {
                found_gears.push(p.value);
            }
        }
        if found_gears.len() == 2 {
            let prod = found_gears.iter().product::<i64>();
            sum += prod;
        }
    }

    println!("{}", sum);

    Ok(())
}
