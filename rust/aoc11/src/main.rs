use itertools::Itertools;
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

fn part1(input: &str) -> Result<()> {
    let mut grid: Vec<Vec<char>> = input
        .trim()
        .split('\n')
        .map(|l| l.chars().collect())
        .collect();

    print_grid(&grid);

    // check rows
    let mut empty_rows = vec![];
    for row in 0..grid.len() {
        let mut no_galaxies = true;
        for col in 0..grid[row].len() {
            if grid[row][col] == '#' {
                no_galaxies = false;
                break;
            }
        }
        if no_galaxies {
            empty_rows.push(row);
            // let r = vec!['.'; grid[row].len()];
            // grid.insert(row, r);
            // for row in 0..grid.len() {
            //     grid[row].insert(idx, '.');
            // }
        }
    }

    for (i, e) in empty_rows.iter().enumerate() {
        let r = vec!['.'; grid[0].len()];
        grid.insert(e + i, r);
    }

    println!();

    print_grid(&grid);

    // check cols
    let mut empty_cols = vec![];
    for col in 0..grid[0].len() {
        let mut no_galaxies = true;
        for row in 0..grid.len() {
            if grid[row][col] == '#' {
                no_galaxies = false;
                break;
            }
        }
        if no_galaxies {
            empty_cols.push(col);
        }
    }

    println!("{:?}", empty_cols);
    for (i, e) in empty_cols.iter().enumerate() {
        for row in 0..grid.len() {
            grid[row].insert(e + i, '.');
        }
    }

    println!();

    print_grid(&grid);

    let mut galaxy_locations = HashMap::new();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '#' {
                galaxy_locations.insert(galaxy_locations.len() + 1, (row, col));
            }
        }
    }

    // println!("{:#?}", galaxy_locations);

    let combs = galaxy_locations
        .iter()
        .map(|(k, _)| *k)
        .combinations(2)
        .collect::<Vec<Vec<usize>>>();
    // println!("{:?}", combs);
    println!("combinations: {:?}", combs.len());

    let mut sum = 0;
    for c in combs {
        if let Some(v) = galaxy_locations.get(&c[0]) {
            if let Some(v2) = galaxy_locations.get(&c[1]) {
                // sum += bfs(*v, *v2, &grid);
                sum += (v.0 as i32 - v2.0 as i32).abs() as usize
                    + (v.1 as i32 - v2.1 as i32).abs() as usize;
            }
        }
    }

    println!("{}", sum);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let lines = input.trim().lines().collect::<Vec<_>>();
    let mut row_offset = 0;
    let mut col_has_galaxy = vec![false; lines[0].len()];
    let mut galaxies = HashSet::new();
    let mut row_offsets = vec![];
    let mut col_offsets = vec![];

    for (row, line) in lines.iter().enumerate() {
        let mut galaxy_spotted = false;
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.insert((row as i64, col as i64));
                galaxy_spotted = true;
                col_has_galaxy[col] = true;
            }
        }
        if !galaxy_spotted {
            row_offset += 1;
        }
        row_offsets.push(row_offset);
    }

    let mut col_offset = 0;
    for col in col_has_galaxy {
        if !col {
            col_offset += 1;
        }
        col_offsets.push(col_offset);
    }

    let distance = calculate_distances(&galaxies, &row_offsets, &col_offsets, 1_000_000);
    println!("{}", distance);
    Ok(())
}

fn calculate_distances(
    galaxies: &HashSet<(i64, i64)>,
    row_offsets: &Vec<i64>,
    col_offsets: &Vec<i64>,
    scale: i64,
) -> i64 {
    let galaxies = galaxies.iter().collect::<Vec<_>>();
    let scale = scale - 1;
    let rows = row_offsets
        .iter()
        .map(|row| row * scale)
        .collect::<Vec<_>>();
    let cols = col_offsets
        .iter()
        .map(|row| row * scale)
        .collect::<Vec<_>>();

    let mut total_distance = 0;
    for (i, first) in galaxies.iter().enumerate() {
        let (irow, icol) = (
            first.0 + rows[first.0 as usize],
            first.1 + cols[first.1 as usize],
        );

        for second in galaxies.iter().skip(i) {
            let (jrow, jcol) = (
                second.0 + rows[second.0 as usize],
                second.1 + cols[second.1 as usize],
            );
            total_distance += (irow - jrow).abs() + (icol - jcol).abs();
        }
    }

    total_distance
}

// fn bfs(from: (usize, usize), to: (usize, usize), grid: &Vec<Vec<char>>) -> usize {
//     // println!("{:?} -> {:?}", from, to);
//     let mut queue = VecDeque::new();
//     let mut visited = HashSet::new();

//     queue.push_back((from, 0));

//     let dirs = vec![(1, 0), (-1, 0), (0, -1), (0, 1)];
//     while !queue.is_empty() {
//         let current = queue.pop_front().unwrap();
//         visited.insert(current.0);
//         if current.0 == to {
//             println!("found: {:?}", current);
//             return current.1;
//         }

//         // if visited.contains(&current.0) {
//         //     continue;
//         // }

//         for d in &dirs {
//             let rx = current.0 .0 as i32 + d.0;
//             let cx = current.0 .1 as i32 + d.1;
//             if !visited.contains(&(rx as usize, cx as usize))
//                 && rx >= 0
//                 && cx >= 0
//                 && rx < grid.len() as i32
//                 && cx < grid[0].len() as i32
//             {
//                 println!("pushing: {:?}", ((rx as usize, cx as usize), current.1 + 1));
//                 queue.push_back(((rx as usize, cx as usize), current.1 + 1));
//             }
//         }
//     }
//     println!("not found");
//     0
// }

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}
