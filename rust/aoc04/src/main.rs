use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let mut sum = 0;
    for l in lines {
        let game_parts = l.split(':').collect::<Vec<&str>>();
        let num_parts = game_parts[1].split('|').collect::<Vec<&str>>();
        let winning_nums = num_parts[0]
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let nums = num_parts[1]
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut score = 0;
        for n in nums {
            if winning_nums.contains(&n) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        sum += score;
    }

    println!("{}", sum);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let mut game = vec![1usize; lines.len()];

    for l in lines {
        let game_parts = l.split(':').collect::<Vec<&str>>();
        let game_num_parts = game_parts[0].split_whitespace().collect::<Vec<&str>>();
        let game_num = game_num_parts[1].parse::<u32>().unwrap() - 1;
        let num_parts = game_parts[1].split('|').collect::<Vec<&str>>();
        let winning_nums = num_parts[0]
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let nums = num_parts[1]
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();

        let score = winning_nums.intersection(&nums).count() as u32;
        for i in game_num + 1..game_num + 1 + score {
            let index = game_num;
            game[i as usize] += game[index as usize];
        }
    }

    println!("{:?}", game.iter().sum::<usize>());
    Ok(())
}
