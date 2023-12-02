use std::io::{self, Read};

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
    let sum = lines
        .iter()
        .map(|l| {
            let first = l.chars().find(|c| c.is_digit(10)).unwrap();
            let last = l.chars().rfind(|c| c.is_digit(10)).unwrap();
            let num_str = format!("{}{}", first, last);

            u32::from_str_radix(&num_str, 10).unwrap()
        })
        .sum::<u32>();

    println!("{}", sum);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let sum = lines
        .iter()
        .map(|l| {
            let first = find_digit(&l, false);
            let last = find_digit(&l, true);

            let num_str = format!("{}{}", first, last);
            println!("{}", num_str);
            u32::from_str_radix(&num_str, 10).unwrap()
        })
        .sum::<u32>();

    println!("{}", sum);
    Ok(())
}

fn find_digit(s: &str, rev: bool) -> char {
    let lookups: Vec<&str> = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];

    if rev {
        let mut max = (usize::MIN, "");
        for l in lookups {
            if let Some(idx) = s.rfind(l) {
                if max.0 <= idx {
                    max.0 = max.0.max(idx);
                    max.1 = l;
                }
            }
        }

        return to_char(max.1);
    } else {
        let mut min = (usize::MAX, "");
        for l in lookups {
            if let Some(idx) = s.find(l) {
                if min.0 >= idx {
                    min.0 = min.0.min(idx);
                    min.1 = l;
                }
            }
        }

        return to_char(min.1);
    }
}

fn to_char(s: &str) -> char {
    match s {
        "1" => '1',
        "2" => '2',
        "3" => '3',
        "4" => '4',
        "5" => '5',
        "6" => '6',
        "7" => '7',
        "8" => '8',
        "9" => '9',
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => panic!("should not get here: min"),
    }
}
