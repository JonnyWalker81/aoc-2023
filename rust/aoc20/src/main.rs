use std::{
    collections::{HashMap, HashSet, VecDeque},
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
struct Module {
    mod_type: ModuleType,
    outputs: Vec<String>,
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcaster,
}

fn part1(input: &str) -> Result<()> {
    let mut conjunctions: HashMap<String, HashMap<String, bool>> = HashMap::new();
    let mut modules: HashMap<String, Module> = input
        .trim()
        .lines()
        .map(|l| {
            let (lhs, rhs) = l.split_once(" -> ").unwrap();
            let rhs = rhs.split(", ").map(|s| s.to_string()).collect();
            if lhs.starts_with("%") {
                let name = lhs[1..].to_string();
                let m = Module {
                    mod_type: ModuleType::FlipFlop(false),
                    outputs: rhs,
                };
                (name, m)
            } else if lhs.starts_with("&") {
                let name = lhs[1..].to_string();
                conjunctions.insert(name.clone(), HashMap::new());
                let m = Module {
                    mod_type: ModuleType::Conjunction(HashMap::new()),
                    outputs: rhs,
                };
                (name, m)
            } else {
                let name = lhs.to_string();
                let m = Module {
                    mod_type: ModuleType::Broadcaster,
                    outputs: rhs,
                };
                (name, m)
            }
        })
        .collect();

    println!("{:?}", conjunctions);

    for (name, module) in &modules {
        for out in &module.outputs {
            if let Some(conj) = conjunctions.get_mut(out) {
                conj.insert(name.clone(), false);
            }
        }
    }
    println!("after: {:?}", conjunctions);

    for (name, module) in modules.iter_mut() {
        if let Some(conj) = conjunctions.get(name) {
            module.mod_type = ModuleType::Conjunction(conj.clone());
        }
    }
    println!("{:?}", modules);

    let mut counts = [0, 0];
    for _ in 0..1000 {
        let (h, l) = pulse(&mut modules, &Vec::new(), false);
        counts[0] += h;
        counts[1] += l;
    }
    println!("{} {}", counts[0], counts[1]);
    println!("{}", counts[0] * counts[1]);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut conjunctions: HashMap<String, HashMap<String, bool>> = HashMap::new();
    let mut modules: HashMap<String, Module> = input
        .trim()
        .lines()
        .map(|l| {
            let (lhs, rhs) = l.split_once(" -> ").unwrap();
            let rhs = rhs.split(", ").map(|s| s.to_string()).collect();
            if lhs.starts_with("%") {
                let name = lhs[1..].to_string();
                let m = Module {
                    mod_type: ModuleType::FlipFlop(false),
                    outputs: rhs,
                };
                (name, m)
            } else if lhs.starts_with("&") {
                let name = lhs[1..].to_string();
                conjunctions.insert(name.clone(), HashMap::new());
                let m = Module {
                    mod_type: ModuleType::Conjunction(HashMap::new()),
                    outputs: rhs,
                };
                (name, m)
            } else {
                let name = lhs.to_string();
                let m = Module {
                    mod_type: ModuleType::Broadcaster,
                    outputs: rhs,
                };
                (name, m)
            }
        })
        .collect();

    println!("{:?}", conjunctions);

    for (name, module) in &modules {
        for out in &module.outputs {
            if let Some(conj) = conjunctions.get_mut(out) {
                conj.insert(name.clone(), false);
            }
        }
    }
    println!("after: {:?}", conjunctions);

    for (name, module) in modules.iter_mut() {
        if let Some(conj) = conjunctions.get(name) {
            module.mod_type = ModuleType::Conjunction(conj.clone());
        }
    }
    println!("{:?}", modules);

    // let mut count = 0;
    // loop {
    //     count += 1;
    //     let (h, l) = pulse(&mut modules, count, true);
    //     if h == 0 && l == 0 {
    //         break;
    //     }
    // }

    // println!("{}", count);

    // let vals = vec![3733u64, 3911, 4019, 4093];
    // let lcm = vals.lcm().unwrap();
    // println!("{}", lcm);
    let mut source = None;
    for (name, Module { outputs, .. }) in modules.iter() {
        if outputs.contains(&"rx".to_string()) {
            source = Some(name.clone());
            break;
        }
    }

    let Some(source) = source else {
        panic!("no source found.");
    };

    println!("{source:?} -> rx");

    let mut prepre = Vec::new();
    for (name, Module { outputs, .. }) in modules.iter() {
        if outputs.contains(&source) {
            prepre.push(name.clone());
        }
    }

    println!("{:?} = {:?}", source, prepre);

    let mut found = 0;
    let mut loop_count = 0;
    let mut cycles = vec![0u64; prepre.len()];
    loop {
        loop_count += 1;
        if let (0, c) = pulse(&mut modules, &prepre, true) {
            println!("{} {:?}", loop_count, c);
            cycles[c - 1 as usize] = loop_count;
            found += 1;
            if found == prepre.len() {
                break;
            }
        }
    }
    println!("{:?}", cycles.lcm().unwrap());

    Ok(())
}

fn pulse(modules: &mut HashMap<String, Module>, keys: &[String], track_rx: bool) -> (usize, usize) {
    let mut queue = VecDeque::from_iter([("button".to_string(), "broadcaster".to_string(), false)]);
    let mut count = [0, 0];
    let mut this_cycle = 0;

    while let Some((sender, dest, pulse)) = queue.pop_front() {
        count[pulse as usize] += 1;
        if let Some(Module { mod_type, outputs }) = modules.get_mut(&dest) {
            match mod_type {
                ModuleType::FlipFlop(s) => {
                    if !pulse {
                        *s = !*s;
                        for out in outputs {
                            queue.push_back((dest.clone(), out.clone(), *s));
                        }
                    }
                }
                ModuleType::Conjunction(states) => {
                    states.insert(sender.clone(), pulse);
                    let out = !states.values().all(|v| *v);
                    if track_rx && !keys.is_empty() && out {
                        if let Some(p) = keys.iter().position(|k| k == &dest) {
                            this_cycle = p + 1;
                        }
                    }

                    for o in outputs {
                        queue.push_back((dest.clone(), o.clone(), out));
                    }
                }
                ModuleType::Broadcaster => {
                    for out in outputs {
                        queue.push_back((dest.clone(), out.clone(), false));
                    }
                }
            }
        }
    }

    if this_cycle != 0 {
        return (0, this_cycle);
    } else {
        (count[0], count[1])
    }
}
// fn lcm(numbers: Vec<u64>) -> u64 {
//     let mut temp = numbers.clone();

//     // check all the same
//     loop {
//         let mut same = true;

//         for idx in 1..temp.len() {
//             if temp[0] != temp[idx] {
//                 same = false;
//                 break;
//             }
//         }

//         if same {
//             return temp[0];
//         }

//         // Find lowest index
//         match temp
//             .iter()
//             .enumerate()
//             .min_by(|(_, a), (_, b)| a.cmp(b))
//             .map(|(index, _)| index)
//         {
//             Some(idx) => {
//                 temp[idx] = temp[idx] + numbers[idx];
//             }
//             None => panic!("Not possible"),
//         }
//     }
// }

pub trait Mathemagic<T> {
    type Output;
    fn gcd(self) -> Option<Self::Output>;
    fn lcm(self) -> Option<Self::Output>;
}

impl<T> Mathemagic<T> for &[T]
where
    T: Copy
        + std::default::Default
        + std::cmp::PartialOrd
        + std::ops::Rem<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    type Output = T;

    fn gcd(self) -> Option<Self::Output> {
        self.iter().copied().reduce(gcd)
    }

    fn lcm(self) -> Option<Self::Output> {
        self.iter().copied().reduce(lcm)
    }
}
pub fn gcd<T>(x: T, y: T) -> T
where
    T: Copy
        + std::default::Default
        + std::cmp::PartialOrd
        + std::ops::Rem<Output = T>
        + std::ops::Mul<Output = T>,
{
    // let zero: T = 0_u8.try_into().unwrap();
    if y == T::default() {
        x
    } else {
        gcd(y, x % y)
    }
}

pub fn lcm<T>(x: T, y: T) -> T
where
    T: Copy
        + std::default::Default
        + std::cmp::PartialOrd
        + std::ops::Rem<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    x * y / gcd(x, y)
}
