use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    // part2(&input)?;

    Ok(())
}

// AoC 2023 Day 9 Part 1
fn part1(input: &str) -> Result<()> {
    let mut lines: Vec<Vec<usize>> = input
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|l| l.trim())
                .map(String::from)
                .map(|l| l.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    for line in lines.iter_mut() {
        let mut diff = vec![vec![]];
        println!("{:?}", line);
        let mut current_diff = line.clone();
        loop {
            for i in 0..line.len() - 1 {
                current_diff.push(line[i + 1] - line[i]);
            }
            let mut all_zeros = false;
            if current_diff.iter().all(|&x| x == 0) {
                all_zeros = true;
            }
            diff.push(current_diff);

            if all_zeros {
                println!("{:?}", diff);
                break;
            }
        }
    }

    Ok(())
}
