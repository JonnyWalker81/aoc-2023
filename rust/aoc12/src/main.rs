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

#[derive(Debug)]
struct Spring {
    pattern: Vec<char>,
    sizes: Vec<usize>,
}

impl From<&str> for Spring {
    fn from(s: &str) -> Self {
        let (pattern, nums) = s.split_once(' ').unwrap();
        let sizes: Vec<usize> = nums
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        Spring {
            pattern: pattern.chars().collect(),
            sizes,
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let records: Vec<Spring> = input.trim().lines().map(|l| l.into()).collect();
    // println!("{:?}", records);

    let count = records.iter().fold(0, |acc, spring| acc + count(spring));
    println!("{}", count);
    Ok(())
}

fn count(spring: &Spring) -> usize {
    let mut cache = HashMap::new();
    do_count(&spring.pattern, &spring.sizes, &mut cache)
}

type Cache = HashMap<(Vec<char>, Vec<usize>), usize>;

fn do_count(pattern: &[char], sizes: &[usize], cache: &mut Cache) -> usize {
    if let Some(count) = cache.get(&(pattern.to_vec(), sizes.to_vec())) {
        return *count;
    }

    if sizes.is_empty() {
        return (!pattern.contains(&'#')) as usize;
    }

    let min_remaining = sizes.iter().sum::<usize>() + sizes.len() - 1;

    if pattern.len() < min_remaining {
        return 0;
    }

    let count = match pattern[0] {
        '.' => do_count(&pattern[1..], sizes, cache),
        '#' => do_hash(&pattern, sizes, cache),
        '?' => do_count(&pattern[1..], sizes, cache) + do_hash(&pattern, sizes, cache),
        _ => panic!("unexpected char"),
    };

    cache.insert((pattern.to_vec(), sizes.to_vec()), count);
    count
}

fn do_hash(pattern: &[char], sizes: &[usize], cache: &mut Cache) -> usize {
    if pattern.len() < sizes[0] || pattern[0..sizes[0]].contains(&'.') {
        return 0;
    }

    if pattern.len() == sizes[0] {
        return (sizes.len() == 1) as usize;
    }

    if pattern.len() > sizes[0] && pattern[sizes[0]] == '#' {
        return 0;
    }

    do_count(&pattern[sizes[0] + 1..], &sizes[1..], cache)
}

fn part2(input: &str) -> Result<()> {
    let mut records: Vec<Spring> = input.trim().lines().map(|l| l.into()).collect();
    // println!("{:?}", records);

    let mut total = 0;
    let mut cache = HashMap::new();
    for r in records {
        let mut pattern = Vec::new();
        for _ in 0..4 {
            pattern.extend(r.pattern.iter().chain([&'?']));
        }
        pattern.extend(r.pattern.iter());

        let mut sizes = Vec::new();
        for _ in 0..5 {
            sizes.extend(r.sizes.iter());
        }

        total += do_count(&pattern, &sizes, &mut cache);
    }

    // let count = records.iter().fold(0, |acc, spring| acc + count(spring));
    println!("{}", total);
    Ok(())
}
