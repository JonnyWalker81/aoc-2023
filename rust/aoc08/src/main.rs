use std::{
    collections::HashMap,
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

    // println!("{:?}", mapping);

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

    // println!("{:?}", all_as);

    let mut current: HashMap<String, (String, String)> = HashMap::new();
    let mut count = 0;

    let mut found: HashMap<String, String> = HashMap::new();
    for a in &all_as {
        let c = mapping.get(*a).unwrap();
        let cur = (c.0.to_string(), c.1.to_string());
        current
            .entry(a.to_string())
            .and_modify(|v| *v = cur.into())
            .or_insert(cur.into());
        found.entry(a.to_string()).or_insert(cur.0.to_string());
    }

    loop {
        // let arr = all_as.clone();
        // println!("{:?}", current);
        // break;

        for i in instructions.chars() {
            for a in &all_as {
                count += 1;
                match i {
                    'R' => {
                        // if current.get(&a).unwrap().1.ends_with('Z') {
                        if let Some((left, right)) = current.get(&a.to_string()).clone() {
                            if right.ends_with('Z') {
                                found
                                    .entry(right.to_string())
                                    .and_modify(|v| *v = right.to_string());
                                break;
                            }
                            // println!("Found it: {count}");
                            // break 'outer ();
                            current.entry(a.to_string()).and_modify(|vv| {
                                let l = left.clone();
                                let r = right.clone();
                                *vv = (l, r);
                            });
                        }
                        // current = mapping.get(current.1).unwrap();
                    }
                    'L' => {
                        // if current.get(&a).unwrap().0.ends_with('Z') {
                        if let Some((left, _)) = current.get(&a.to_string()).clone() {
                            if left.ends_with('Z') {
                                // found.insert(a.to_string(), v.to_string());
                                found
                                    .entry(left.to_string())
                                    .and_modify(|v| *v = left.to_string());
                                break;
                            }

                            current.entry(a.to_string()).and_modify(|vv| {
                                let t = mapping.get(&left.to_string()).unwrap();
                                let l = t.0.to_string();
                                let r = t.1.to_string();
                                *vv = (l, r);
                            });
                        }
                        // println!("Found it: {count}");
                        // }

                        // current
                        //     .entry(&a)
                        //     .and_modify(|v| *v = mapping.get(v.0).unwrap());
                        // current = mapping.get(current.0).unwrap();
                    }
                    _ => panic!("Unknown instruction"),
                }
            }
        }

        println!("{:?}", found);
        if found.values().all(|v| v.ends_with('Z')) {
            break;
        }
    }

    println!("{:?}", count);
    Ok(())
}
