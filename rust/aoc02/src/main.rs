use std::{
    collections::HashSet,
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

#[derive(Debug, Default)]
struct Game {
    id: u32,
    turns: Vec<Turn>,
}

#[derive(Debug, Default)]
struct Turn {
    red: u32,
    green: u32,
    blue: u32,
}

const RED_LIMIT: u32 = 12;
const GREEN_LIMIT: u32 = 13;
const BLUE_LIMIT: u32 = 14;

fn part1(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let games = lines.iter().filter_map(|l| {
        let game_parts: Vec<&str> = l.split(':').collect();
        let id_parts = game_parts[0].split(' ').collect::<Vec<&str>>();
        let id = id_parts[1].trim().parse::<u32>().unwrap();
        let turn_parts = game_parts[1].split(';').collect::<Vec<&str>>();
        let mut turns = Vec::new();
        for t in turn_parts {
            let cube_parts = t.split(',').collect::<Vec<&str>>();
            let mut turn = Turn::default();
            for c in cube_parts {
                let count_parts = c.trim().split(' ').collect::<Vec<&str>>();
                let count = count_parts[0].trim().parse::<u32>().unwrap();
                let color = count_parts[1];
                if color == "red" {
                    turn.red = count;
                } else if color == "green" {
                    turn.green = count;
                } else if color == "blue" {
                    turn.blue = count;
                }
            }
            turns.push(turn);
        }
        Some(Game { id, turns })
    });

    let mut valid = HashSet::new();
    for g in games {
        let mut is_valid = true;
        for t in g.turns {
            if t.red > RED_LIMIT || t.green > GREEN_LIMIT || t.blue > BLUE_LIMIT {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            valid.insert(g.id);
        }
    }

    println!("{:?}", valid);
    let sum = valid.iter().sum::<u32>();
    println!("{sum}");
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let games = lines.iter().filter_map(|l| {
        let game_parts: Vec<&str> = l.split(':').collect();
        let id_parts = game_parts[0].split(' ').collect::<Vec<&str>>();
        let id = id_parts[1].trim().parse::<u32>().unwrap();
        let turn_parts = game_parts[1].split(';').collect::<Vec<&str>>();
        let mut turns = Vec::new();
        for t in turn_parts {
            let cube_parts = t.split(',').collect::<Vec<&str>>();
            let mut turn = Turn::default();
            for c in cube_parts {
                let count_parts = c.trim().split(' ').collect::<Vec<&str>>();
                let count = count_parts[0].trim().parse::<u32>().unwrap();
                let color = count_parts[1];
                if color == "red" {
                    turn.red = count;
                } else if color == "green" {
                    turn.green = count;
                } else if color == "blue" {
                    turn.blue = count;
                }
            }
            turns.push(turn);
        }
        Some(Game { id, turns })
    });

    let mut powers = Vec::new();
    for g in games {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for t in g.turns {
            max_red = max_red.max(t.red);
            max_green = max_green.max(t.green);
            max_blue = max_blue.max(t.blue);
        }

        let power = max_red * max_green * max_blue;
        powers.push(power);
    }

    let sum = powers.iter().sum::<u32>();
    println!("{:?}", sum);

    Ok(())
}
