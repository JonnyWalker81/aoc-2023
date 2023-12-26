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
                sum += (v.0 as i32 * 1_000_000 - v2.0 as i32 * 1_000_000).abs() as usize
                    + (v.1 as i32 * 1_000_000 - v2.1 as i32 * 1_000_000).abs() as usize;
            }
        }
    }

    println!("{}", sum);

    Ok(())
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
