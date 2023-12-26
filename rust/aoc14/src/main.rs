use std::{
    collections::{HashMap, HashSet},
    hash::DefaultHasher,
    hash::Hash,
    hash::Hasher,
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
    let mut grid = input
        .trim()
        .split('\n')
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    print_grid(&grid);

    //tilt north
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'O' {
                println!("found O at {}, {}...moving north", row, col);
                let start = (row, col);
                let mut end = start;
                for i in (0..row).rev() {
                    println!("moving O to {}, {}", i, col);
                    if grid[i][col] == '#' || grid[i][col] == 'O' {
                        break;
                    }
                    end = (i, col);
                }
                grid[start.0][start.1] = '.';
                grid[end.0][end.1] = 'O';
            }
        }
    }

    println!();

    print_grid(&grid);

    let mut sum = 0;
    let mut load = grid.len();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'O' {
                sum += load;
            }
        }

        load -= 1;
    }

    println!("{}", sum);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut grid = input
        .trim()
        .split('\n')
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    print_grid(&grid);

    let mut count = 0;
    //tilt north
    // for _ in 0..1_000_000_000 {
    let mut seen = HashMap::new();
    let mut found = (0, 0);
    for i in 0..1_000_000_0 {
        move_north(&mut grid);
        // print_grid(&grid);
        move_west(&mut grid);
        // print_grid(&grid);
        move_south(&mut grid);
        // print_grid(&grid);
        move_east(&mut grid);
        // print_grid(&grid);

        // println!("after {} cycle(s)", i + 1);
        let mut sum = 0;
        let mut load = grid.len();
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == 'O' {
                    sum += load;
                }
            }

            load -= 1;
        }

        // println!();

        // print_grid(&grid);
        println!("{}", sum);
        let key_hash = calculate_hash(&grid);
        if let Some(val) = seen.insert(key_hash, i + 1) {
            println!("found {} at {}", sum, i + 1);
            found = (val, i + 1);
            break;
        }
        // if sum == 64 {
        //     count += 1;
        //     println!("found 64");
        // }
    }

    println!("found: {:?}", found);
    let diff = found.1 - found.0;
    let rem = (1_000_000_000 - found.0) % diff;
    println!("rem: {}", rem);

    for _ in 0..rem {
        move_north(&mut grid);
        // print_grid(&grid);
        move_west(&mut grid);
        // print_grid(&grid);
        move_south(&mut grid);
        // print_grid(&grid);
        move_east(&mut grid);
        // print_grid(&grid);
    }

    println!("{}", count);

    // println!();

    // print_grid(&grid);

    let mut sum = 0;
    let mut load = grid.len();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'O' {
                sum += load;
            }
        }

        load -= 1;
    }

    println!("{}", sum);
    Ok(())
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn move_north(grid: &mut Vec<Vec<char>>) {
    // println!("moving north");

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'O' {
                let start = (row, col);
                let mut end = start;
                for i in (0..row).rev() {
                    // println!("moving O to {}, {}", i, col);
                    if grid[i][col] == '#' || grid[i][col] == 'O' {
                        break;
                    }
                    end = (i, col);
                }
                grid[start.0][start.1] = '.';
                grid[end.0][end.1] = 'O';
            }
        }
    }
}

fn move_west(grid: &mut Vec<Vec<char>>) {
    // println!("moving west");
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'O' {
                // println!("moving west");
                let start = (row, col);
                let mut end = start;
                for i in (0..col).rev() {
                    // println!("  moving west to {}, {}", i, row);
                    if grid[row][i] == '#' || grid[row][i] == 'O' {
                        break;
                    }
                    end = (row, i);
                }
                grid[start.0][start.1] = '.';
                grid[end.0][end.1] = 'O';
            }
        }
    }
}

fn move_south(grid: &mut Vec<Vec<char>>) {
    // println!("moving south");
    // println!("found O at {}, {}...moving north", row, col);
    for row in (0..grid.len()).rev() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'O' {
                let start = (row, col);
                let mut end = start;
                for i in row + 1..grid.len() {
                    // println!("moving O to {}, {}", i, col);
                    if grid[i][col] == '#' || grid[i][col] == 'O' {
                        break;
                    }
                    end = (i, col);
                }
                grid[start.0][start.1] = '.';
                grid[end.0][end.1] = 'O';
            }
        }
    }
}

fn move_east(grid: &mut Vec<Vec<char>>) {
    // println!("moving east");
    // println!("found O at {}, {}...moving north", row, col);
    for row in 0..grid.len() {
        for col in (0..grid[row].len()).rev() {
            if grid[row][col] == 'O' {
                let start = (row, col);
                let mut end = start;
                for i in col + 1..grid[row].len() {
                    // println!("moving O to {}, {}", i, row);
                    if grid[row][i] == '#' || grid[row][i] == 'O' {
                        break;
                    }
                    end = (row, i);
                }
                grid[start.0][start.1] = '.';
                grid[end.0][end.1] = 'O';
            }
        }
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
}
