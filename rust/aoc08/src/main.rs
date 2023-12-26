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
    let mut lines: Vec<String> = input.trim().lines().map(|l| l.to_string()).collect();

    let mut iter = lines.iter_mut();
    let instructions = iter.next().unwrap();
    iter.next();

    let mapping = iter
        .map(|l| {
            let (node, choices) = l.split_once(" = ").unwrap();
            let (left_, right_) = choices.split_once(", ").unwrap();
            let left = left_.trim_start_matches('(');
            let right = right_.trim_end_matches(')');
            (node, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let mut current = mapping.get("AAA").unwrap();
    let mut count = 0;
    'outer: loop {
        for i in instructions.chars() {
            count += 1;
            match i {
                'R' => {
                    if current.1 == "ZZZ" {
                        println!("Found it: {count}");
                        break 'outer ();
                    }
                    current = mapping.get(current.1).unwrap();
                }
                'L' => {
                    if current.0 == "ZZZ" {
                        println!("Found it: {count}");
                        break 'outer;
                    }

                    current = mapping.get(current.0).unwrap();
                }
                _ => panic!("Unknown instruction"),
            }
        }
    }
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut lines: Vec<String> = input.trim().lines().map(|l| l.to_string()).collect();

    let mut iter = lines.iter_mut();
    let instructions = iter.next().unwrap();
    iter.next();

    let mapping: HashMap<String, (String, String)> = iter
        .map(|l| {
            let (node, choices) = l.split_once(" = ").unwrap();
            let (left_, right_) = choices.split_once(", ").unwrap();
            let left = left_.trim_start_matches('(');
            let right = right_.trim_end_matches(')');
            (node.to_string(), (left.to_string(), right.to_string()))
        })
        .collect::<HashMap<_, _>>();

    let all_as = mapping
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<_>>();

    let mut found: HashMap<String, usize> = HashMap::new();
    for a in &all_as {
        let mut count = 0;
        let mut current = mapping.get(&a.to_string()).unwrap();

        'outer: loop {
            for i in instructions.chars() {
                count += 1;
                match i {
                    'R' => {
                        if current.1.ends_with('Z') {
                            println!("Found it (right): {count}");
                            found
                                .entry(current.1.to_string())
                                .and_modify(|v| *v = count)
                                .or_insert(count);
                            break 'outer ();
                        }
                        current = mapping.get(&current.1).unwrap();
                    }
                    'L' => {
                        if current.0.ends_with('Z') {
                            println!("Found it (left): {count}");
                            found
                                .entry(current.0.to_string())
                                .and_modify(|v| *v = count)
                                .or_insert(count);
                            break 'outer ();
                        }
                        current = mapping.get(&current.0).unwrap();
                    }
                    _ => panic!("Unknown instruction"),
                }
            }
        }
    }

    println!("{:?}", found);

    let numbers = found.values().map(|v| *v as u64).collect::<Vec<_>>();
    let lcm = lcm(numbers);

    println!("{:?}", lcm);
    Ok(())
}

// https://www.andyloree.com/blog/2022/12/11/least-common-multiple-vect-rust/
fn lcm(numbers: Vec<u64>) -> u64 {
    let mut temp = numbers.clone();

    // check all the same
    loop {
        let mut same = true;

        for idx in 1..temp.len() {
            if temp[0] != temp[idx] {
                same = false;
                break;
            }
        }

        if same {
            return temp[0];
        }

        // Find lowest index
        match temp
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
        {
            Some(idx) => {
                temp[idx] = temp[idx] + numbers[idx];
            }
            None => panic!("Not possible"),
        }
    }
}
