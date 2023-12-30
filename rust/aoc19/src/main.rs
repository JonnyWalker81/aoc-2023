use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Read},
    ops::Range,
};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    condition: Option<Condition>,
    value: Option<u64>,
    result: Option<String>,
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
enum Condition {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rating {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

impl Rating {
    fn from_str(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("invalid rating"),
        }
    }
}

impl From<String> for Rating {
    fn from(s: String) -> Self {
        Self::from_str(&s)
    }
}

#[derive(Debug, Clone)]
struct Ratings {
    ratings: HashMap<String, u64>,
}

fn part1(input: &str) -> Result<()> {
    // qqz{s>2770:qs,m<1801:hdj,R}
    let (rules_input, parts_input) = input.trim().split_once("\n\n").unwrap();

    let workflows: HashMap<String, Workflow> = rules_input
        .trim()
        .lines()
        .map(|l| {
            println!("parsing: {}", l);
            let (name, mut rest) = l.split_once("{").unwrap();
            rest = rest.trim_end_matches("}");
            let rule_parts: Vec<&str> = rest.split(",").collect();
            let mut rules = vec![];
            for r in rule_parts {
                println!("  rule: {}", r);
                if r.contains(":") {
                    let (condition, result) = r.split_once(":").unwrap();
                    if condition.contains("<") {
                        let (wf, value) = condition.split_once("<").unwrap();
                        let rule = Rule {
                            name: wf.to_string(),
                            condition: Some(Condition::LessThan),
                            value: Some(value.parse::<u64>().unwrap()),
                            result: Some(result.to_string()),
                        };
                        rules.push(rule);
                    } else if condition.contains(">") {
                        let (wf, value) = condition.split_once(">").unwrap();
                        let rule = Rule {
                            name: wf.to_string(),
                            condition: Some(Condition::GreaterThan),
                            value: Some(value.parse::<u64>().unwrap()),
                            result: Some(result.to_string()),
                        };
                        rules.push(rule);
                    }
                } else {
                    let rule = Rule {
                        name: r.to_string(),
                        condition: None,
                        value: None,
                        result: Some(r.to_string()),
                    };
                    rules.push(rule);
                }
            }

            (name.to_string(), Workflow { rules })
        })
        .collect();

    println!("{:?}", workflows);

    let parts: Vec<Ratings> = parts_input
        .trim()
        .lines()
        .map(|mut l| {
            l = l.trim_start_matches("{");
            l = l.trim_end_matches("}");
            let vals = l.split(",");
            let mut ratings = HashMap::new();
            for v in vals {
                let (name, value) = v.split_once("=").unwrap();
                ratings.insert(name.to_string(), value.parse::<u64>().unwrap());
            }
            Ratings { ratings }
        })
        .collect();

    // let mut current = workflows.get("in").unwrap().clone();

    let mut accepted = vec![];
    // for p in parts.iter().take(3) {
    for p in parts {
        let mut current = "in".to_string();
        'outer: loop {
            if let Some(workflow) = workflows.get(&current) {
                println!("current: {}", current);
                println!("  {:?}", workflow);
                for r in &workflow.rules {
                    println!("    rule: {:?}", r);
                    if r.result.is_some()
                        && r.condition.is_none()
                        && r.result.as_ref().unwrap() == "R"
                    {
                        break 'outer;
                    } else if r.result.is_some()
                        && r.condition.is_none()
                        && r.result.as_ref().unwrap() == "A"
                    {
                        accepted.push(p.clone());
                        break 'outer;
                    } else if r.condition.is_none() {
                        current = r.result.as_ref().unwrap().clone();
                        println!("  label: {}", current);
                        break;
                    } else {
                        println!("  condition: {:?} ({})", r.condition, r.name);
                        match r.condition {
                            Some(Condition::LessThan) => {
                                if let Some(v) = r.value {
                                    if let Some(rating) = p.ratings.get(&r.name) {
                                        if rating < &v {
                                            if r.result.is_some()
                                                && r.result.as_ref().unwrap() == "R"
                                            {
                                                break 'outer;
                                            } else if r.result.is_some()
                                                && r.result.as_ref().unwrap() == "A"
                                            {
                                                accepted.push(p.clone());
                                                break 'outer;
                                            }
                                            current = r.result.as_ref().unwrap().clone();
                                            break;
                                        }
                                    }
                                }
                            }
                            Some(Condition::GreaterThan) => {
                                if let Some(v) = r.value {
                                    if let Some(rating) = p.ratings.get(&r.name) {
                                        println!("{} > {}", rating, v);
                                        if rating > &v {
                                            if r.result.is_some()
                                                && r.result.as_ref().unwrap() == "R"
                                            {
                                                break 'outer;
                                            } else if r.result.is_some()
                                                && r.result.as_ref().unwrap() == "A"
                                            {
                                                accepted.push(p.clone());
                                                break 'outer;
                                            }

                                            current = r.result.as_ref().unwrap().clone();
                                            break;
                                        }
                                    }
                                }
                            }
                            None => {
                                panic!("Unknown condition");
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", accepted);
    let mut sum = 0;
    for a in accepted {
        for (k, v) in a.ratings.iter() {
            println!("{}: {}", k, v);
            sum += v;
        }
    }
    println!("{}", sum);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    // qqz{s>2770:qs,m<1801:hdj,R}
    let (rules_input, parts_input) = input.trim().split_once("\n\n").unwrap();

    let workflows: HashMap<String, Workflow> = rules_input
        .trim()
        .lines()
        .map(|l| {
            println!("parsing: {}", l);
            let (name, mut rest) = l.split_once("{").unwrap();
            rest = rest.trim_end_matches("}");
            let rule_parts: Vec<&str> = rest.split(",").collect();
            let mut rules = vec![];
            for r in rule_parts {
                println!("  rule: {}", r);
                if r.contains(":") {
                    let (condition, result) = r.split_once(":").unwrap();
                    if condition.contains("<") {
                        let (wf, value) = condition.split_once("<").unwrap();
                        let rule = Rule {
                            name: wf.to_string(),
                            condition: Some(Condition::LessThan),
                            value: Some(value.parse::<u64>().unwrap()),
                            result: Some(result.to_string()),
                        };
                        rules.push(rule);
                    } else if condition.contains(">") {
                        let (wf, value) = condition.split_once(">").unwrap();
                        let rule = Rule {
                            name: wf.to_string(),
                            condition: Some(Condition::GreaterThan),
                            value: Some(value.parse::<u64>().unwrap()),
                            result: Some(result.to_string()),
                        };
                        rules.push(rule);
                    }
                } else {
                    let rule = Rule {
                        name: r.to_string(),
                        condition: None,
                        value: None,
                        result: Some(r.to_string()),
                    };
                    rules.push(rule);
                }
            }

            (name.to_string(), Workflow { rules })
        })
        .collect();

    println!("{:?}", workflows);

    let parts: Vec<Ratings> = parts_input
        .trim()
        .lines()
        .map(|mut l| {
            l = l.trim_start_matches("{");
            l = l.trim_end_matches("}");
            let vals = l.split(",");
            let mut ratings = HashMap::new();
            for v in vals {
                let (name, value) = v.split_once("=").unwrap();
                ratings.insert(name.to_string(), value.parse::<u64>().unwrap());
            }
            Ratings { ratings }
        })
        .collect();

    // let mut current = workflows.get("in").unwrap().clone();

    // let mut accepted = vec![];
    // let mut ranges = vec![1..4000usize; 4];
    let ranges = [1..4000usize, 1..4000, 1..4000, 1..4000];
    // for p in parts.iter().take(3) {

    let current = workflows.get("in").unwrap().clone();
    let sum = compute(&workflows, &current.rules, ranges);

    // println!("{:?}", accepted);
    // let mut sum = 0;
    // for a in accepted {
    //     for (k, v) in a.ratings.iter() {
    //         println!("{}: {}", k, v);
    //         sum += v;
    //     }
    // }
    println!("{}", sum);

    Ok(())
}

fn compute(
    workflows: &HashMap<String, Workflow>,
    rules: &[Rule],
    mut ranges: [Range<usize>; 4],
) -> usize {
    let mut total = 0;
    for rule in rules {
        let mut deeper = ranges.clone();
        match rule.condition {
            Some(Condition::LessThan) => {
                let rating: Rating = rule.name.clone().into();
                println!(
                    "compute rule: {}({}) < {}",
                    rule.name,
                    rating as usize,
                    rule.value.unwrap()
                );
                deeper[rating as usize].end = rule.value.unwrap() as usize - 1;
                ranges[rating as usize].start = rule.value.unwrap() as usize;
            }
            Some(Condition::GreaterThan) => {
                let rating: Rating = rule.name.clone().into();
                deeper[rating as usize].start = rule.value.unwrap() as usize + 1;
                ranges[rating as usize].end = rule.value.unwrap() as usize;
            }
            None => {
                if let Some(v) = rule.result.as_ref() {
                    if v == "A" {
                        total += deeper.iter().map(|r| r.len() + 1).product::<usize>();
                    } else if v != "R" {
                        let wf = workflows.get(v).unwrap();
                        total += compute(workflows, &wf.rules, deeper);
                    }
                }
            }
        }
    }
    total
}
