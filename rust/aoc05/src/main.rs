use std::{
    collections::HashMap,
    io::{self, Read},
    str::Lines,
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
struct Almanac {
    destination: usize,
    source: usize,
    len: usize,
}

fn part1(input: &str) -> Result<()> {
    let mut seeds: Vec<usize> = vec![];
    let mut lines = input.lines();
    let mut maps = HashMap::new();

    while let Some(l) = lines.next() {
        if l.trim().starts_with("seeds") {
            let seed_parts = l.trim().split(':').collect::<Vec<&str>>();
            seeds = seed_parts[1]
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
        }

        if l.trim().starts_with("seed-to-soil") {
            let almanacs = build_alamanac(&mut lines)?;
            maps.insert("seed-to-soil", almanacs);
        }

        if l.trim().starts_with("soil-to-fertilize") {
            let almanacs = build_alamanac(&mut lines)?;
            maps.insert("soil-to-fertilize", almanacs);
        }

        if l.trim().starts_with("fertilizer-to-water") {
            let almanacs = build_alamanac(&mut lines)?;
            maps.insert("fertilizer-to-water", almanacs);
        }

        if l.trim().starts_with("water-to-light") {
            let almanacs = build_alamanac(&mut lines)?;
            maps.insert("water-to-light", almanacs);
        }

        if l.trim().starts_with("light-to-temperature") {
            let almanacs = build_alamanac(&mut lines)?;
            maps.insert("light-to-temperature", almanacs);
        }

        if l.trim().starts_with("temperature-to-humidity") {
            let almanacs = build_alamanac(&mut lines)?;
            maps.insert("temperature-to-humidity", almanacs);
        }

        if l.trim().starts_with("humidity-to-location") {
            let almanacs = build_alamanac(&mut lines)?;
            maps.insert("humidity-to-location", almanacs);
        }
    }

    let chain = vec![
        "seed-to-soil",
        "soil-to-fertilize",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    let mut min_location = usize::MAX;
    for s in seeds {
        let mut next = s;
        for c in &chain {
            let almanacs = maps.get(c).unwrap();
            let almanac = almanacs
                .iter()
                .find(|a| next > a.source && next < a.source + a.len);
            next = if let Some(a) = almanac {
                if next > a.source {
                    next - a.source + a.destination
                } else {
                    a.source - next + a.destination
                }
            } else {
                next
            };
        }
        min_location = min_location.min(next);
    }

    println!("part 1: {}", min_location);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut seeds: Vec<usize> = vec![];
    let mut lines = input.lines();
    let mut maps = HashMap::new();

    while let Some(l) = lines.next() {
        if l.trim().starts_with("seeds") {
            let seed_parts = l.trim().split(':').collect::<Vec<&str>>();
            let seed_ranges = seed_parts[1]
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            for i in (0..seed_ranges.len()).step_by(2) {
                println!("{} -> {}", seed_ranges[i], seed_ranges[i + 1]);
                for s in seed_ranges[i]..seed_ranges[i] + seed_ranges[i + 1] {
                    seeds.push(s);
                }
            }
        }

        if l.trim().starts_with("seed-to-soil") {
            let almanacs = build_alamanac(&mut lines)?;
            maps.insert("seed-to-soil", almanacs);
        }

        if l.trim().starts_with("soil-to-fertilize") {
            let almanacs = build_alamanac(&mut lines)?;
            maps.insert("soil-to-fertilize", almanacs);
        }

        if l.trim().starts_with("fertilizer-to-water") {
            let almanacs = build_alamanac(&mut lines)?;
            maps.insert("fertilizer-to-water", almanacs);
        }

        if l.trim().starts_with("water-to-light") {
            let almanacs = build_alamanac(&mut lines)?;
            maps.insert("water-to-light", almanacs);
        }

        if l.trim().starts_with("light-to-temperature") {
            let almanacs = build_alamanac(&mut lines)?;
            maps.insert("light-to-temperature", almanacs);
        }

        if l.trim().starts_with("temperature-to-humidity") {
            let almanacs = build_alamanac(&mut lines)?;
            maps.insert("temperature-to-humidity", almanacs);
        }

        if l.trim().starts_with("humidity-to-location") {
            let almanacs = build_alamanac(&mut lines)?;
            maps.insert("humidity-to-location", almanacs);
        }
    }

    let chain = vec![
        "seed-to-soil",
        "soil-to-fertilize",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    let mut min_locations = usize::MAX;
    for s in seeds {
        let mut next = s;
        for c in &chain {
            let almanacs = maps.get(c).unwrap();
            let almanac = almanacs
                .iter()
                .find(|a| next >= a.source && next <= a.source + a.len);
            next = if let Some(a) = almanac {
                if next > a.source {
                    next - a.source + a.destination
                } else {
                    a.source - next + a.destination
                }
            } else {
                next
            };
        }
        min_locations = min_locations.min(next);
    }

    println!("part 2: {}", min_locations);

    Ok(())
}

fn build_alamanac(lines: &mut Lines) -> Result<Vec<Almanac>> {
    let mut alamanacs = vec![];
    loop {
        if let Some(mapping_line) = lines.next() {
            if mapping_line.trim().is_empty() {
                break;
            }

            let parts = mapping_line
                .trim()
                .split_whitespace()
                .collect::<Vec<&str>>();
            let destination = parts[0].parse::<usize>().unwrap();
            let source = parts[1].parse::<usize>().unwrap();
            let len = parts[2].parse::<usize>().unwrap();

            let almanac = Almanac {
                destination,
                source,
                len,
            };
            alamanacs.push(almanac);
        } else {
            break;
        }
    }

    Ok(alamanacs)
}
