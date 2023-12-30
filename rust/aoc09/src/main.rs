use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

// AoC 2023 Day 9 Part 1
fn part1(input: &str) -> Result<()> {
    let mut lines: Vec<Vec<i64>> = input
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|l| l.trim())
                .map(String::from)
                .map(|l| l.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let mut total = 0;
    for line in lines.iter_mut() {
        // let mut ends = vec![line[line.len() - 1]];
        let mut diffs = line.clone();
        let mut sum = line[line.len() - 1];
        loop {
            diffs = diffs
                .as_slice()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<_>>();
            println!("{:?}", diffs);
            // ends.push(diffs[diffs.len() - 1]);
            sum += diffs[diffs.len() - 1];

            if diffs.iter().all(|&x| x == 0) {
                println!("all zero");
                // let sum = ends.iter().fold(0, |acc, x| acc + x);
                println!("{}", sum);
                total += sum;
                break;
            }
        }
        // let mut current_diff = line.clone();
        // loop {
        //     for i in 0..line.len() - 1 {
        //         current_diff.push(line[i + 1] - line[i]);
        //     }
        //     let mut all_zeros = false;
        //     if current_diff.iter().all(|&x| x == 0) {
        //         all_zeros = true;
        //     }
        //     diff.push(current_diff);

        //     if all_zeros {
        //         println!("{:?}", diff);
        //         break;
        //     }
        // }
    }
    println!("{}", total);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut lines: Vec<Vec<i64>> = input
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|l| l.trim())
                .map(String::from)
                .map(|l| l.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let mut total = 0;
    for line in lines.iter_mut() {
        // let mut ends = vec![line[line.len() - 1]];
        let mut diffs = line.iter().rev().copied().collect::<Vec<_>>();
        let mut sum = diffs[diffs.len() - 1];
        loop {
            diffs = diffs
                .as_slice()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<_>>();
            println!("{:?}", diffs);
            // ends.push(diffs[diffs.len() - 1]);
            sum += diffs[diffs.len() - 1];

            if diffs.iter().all(|x| *x == 0) {
                println!("all zero");
                // let sum = ends.iter().fold(0, |acc, x| acc + x);
                println!("{}", sum);
                total += sum;
                break;
            }
        }
        // let mut current_diff = line.clone();
        // loop {
        //     for i in 0..line.len() - 1 {
        //         current_diff.push(line[i + 1] - line[i]);
        //     }
        //     let mut all_zeros = false;
        //     if current_diff.iter().all(|&x| x == 0) {
        //         all_zeros = true;
        //     }
        //     diff.push(current_diff);

        //     if all_zeros {
        //         println!("{:?}", diff);
        //         break;
        //     }
        // }
    }
    println!("{}", total);

    Ok(())
}
