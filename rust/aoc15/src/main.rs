use std::{
    collections::HashMap,
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
    let instructions: Vec<String> = input
        .trim()
        .lines()
        .flat_map(|l| l.split(","))
        .map(|l| l.to_string())
        .collect();
    // let instructions: Vec<String> = vec!["rn=1".to_string()];

    let mut sum = 0;
    for i in instructions {
        // println!("hashing: {}", i);
        let hash = compute_hash(&i);
        // println!("  hash: {}", hash);
        sum += hash;
    }

    println!("{}", sum);
    Ok(())
}

#[derive(Debug, Clone)]
struct Instruction {
    label: String,
    focal_length: u64,
}

fn part2(input: &str) -> Result<()> {
    let instructions: Vec<String> = input
        .trim()
        .lines()
        .flat_map(|l| l.split(","))
        .map(|l| l.to_string())
        .collect();
    // let instructions: Vec<String> = vec!["rn=1".to_string()];

    let mut map: HashMap<u64, Vec<Instruction>> = HashMap::new();
    for i in instructions {
        let (label, focal_length) = i.split_once(['=', '-']).unwrap();
        // println!("hashing: {}", label);
        let hash = compute_hash(&label);
        // println!("  hash: {}", hash);
        if i.contains('-') {
            if let Some(ref mut v) = map.get_mut(&hash) {
                if let Some(p) = v.iter().position(|x| x.label == label) {
                    v.remove(p);
                }
            }
        } else if i.contains('=') {
            if let Some(ref mut v) = map.get_mut(&hash) {
                if let Some(p) = v.iter().position(|x| x.label == label) {
                    v[p].focal_length = focal_length.parse::<u64>().unwrap();
                } else {
                    v.push(Instruction {
                        label: label.to_string(),
                        focal_length: focal_length.parse::<u64>().unwrap(),
                    });
                }
            } else {
                let mut v = Vec::new();
                v.push(Instruction {
                    label: label.to_string(),
                    focal_length: focal_length.parse::<u64>().unwrap(),
                });
                map.insert(hash, v);
            }
        }
        // sum += hash;
    }

    // println!("{:#?}", map);

    let mut sum = 0;
    for (k, v) in map {
        for (j, l) in v.iter().enumerate() {
            let power = (1 + k as usize) * (j + 1) * l.focal_length as usize;
            // println!(
            //     "{}: {} * {} * {} = {}",
            //     i,
            //     1 + k,
            //     j + 1,
            //     l.focal_length,
            //     power
            // );
            sum += power
        }
    }

    println!("{}", sum);
    Ok(())
}

fn compute_hash(input: &str) -> u64 {
    let mut value: u64 = 0;
    for (_, c) in input.chars().enumerate() {
        value += c as u64;
        value *= 17;
        value = value % 256;
    }

    value
}
