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
struct Hand {
    hand: String,
    rank: usize,
}

fn part1(input: &str) -> Result<()> {
    let mut hands: Vec<Hand> = input
        .trim()
        .lines()
        .map(|line| {
            let (hand, rank) = line.split_once(" ").unwrap();
            Hand {
                hand: hand.to_string(),
                rank: rank.parse::<usize>().unwrap(),
            }
        })
        .collect();

    hands.sort_by(|a, b| {
        let a_hand = get_hand_value(&a.hand);
        let b_hand = get_hand_value(&b.hand);
        if a_hand == b_hand {
            for i in 0..a.hand.len() {
                if a.hand.chars().nth(i).unwrap() != b.hand.chars().nth(i).unwrap() {
                    let a_value = card_value(a.hand.chars().nth(i).unwrap());
                    let b_value = card_value(b.hand.chars().nth(i).unwrap());
                    return a_value.cmp(&b_value);
                }
            }
        }
        a_hand.cmp(&b_hand)
    });

    // println!("{:?}", hands);

    let sum = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.rank * (i + 1)));
    println!("{}", sum);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut hands: Vec<Hand> = input
        .trim()
        .lines()
        .map(|line| {
            let (hand, rank) = line.split_once(" ").unwrap();
            Hand {
                hand: hand.to_string(),
                rank: rank.parse::<usize>().unwrap(),
            }
        })
        .collect();

    hands.sort_by(|a, b| {
        let a_hand = get_hand_value2(&a.hand);
        let b_hand = get_hand_value2(&b.hand);
        if a_hand == b_hand {
            if a_hand == HandValue::FiveOfAKind {
                println!("{} - {}", a.hand, b.hand);
                // return a.rank.cmp(&b.rank);
            }
            for i in 0..a.hand.len() {
                if a.hand.chars().nth(i).unwrap() != b.hand.chars().nth(i).unwrap() {
                    let a_value = card_value2(a.hand.chars().nth(i).unwrap());
                    let b_value = card_value2(b.hand.chars().nth(i).unwrap());
                    return a_value.cmp(&b_value);
                }
            }
        }
        a_hand.cmp(&b_hand)
    });

    // println!("{:#?}", hands);
    // for h in hands.iter() {
    //     println!("{} - {:?}", h.hand, get_hand_value2(&h.hand));
    // }

    let sum = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.rank * (i + 1)));
    println!("{}", sum);
    Ok(())
}

fn card_value(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }
}

fn card_value2(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }
}

fn value_to_card(value: u32) -> char {
    match value {
        14 => 'A',
        13 => 'K',
        12 => 'Q',
        1 => 'J',
        10 => 'T',
        _ => value.to_string().chars().nth(0).unwrap(),
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum HandValue {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

fn get_hand_value(hand: &str) -> HandValue {
    let mut values = HashMap::new();
    for c in hand.chars() {
        values.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    if values.values().any(|v| *v == 5) {
        return HandValue::FiveOfAKind;
    }

    if values.values().any(|v| *v == 4) {
        return HandValue::FourOfAKind;
    }

    if values.values().any(|v| *v == 3) && values.values().any(|v| *v == 2) {
        return HandValue::FullHouse;
    }

    if values.values().any(|v| *v == 3) {
        return HandValue::ThreeOfAKind;
    }

    let pairs = values.values().filter(|v| **v == 2).count();
    if pairs == 2 {
        return HandValue::TwoPair;
    }

    if pairs == 1 {
        return HandValue::OnePair;
    }

    HandValue::HighCard
}

fn get_hand_value2(hand: &str) -> HandValue {
    let mut values = HashMap::new();
    for c in hand.chars() {
        values.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    // let highest_value = values.iter().map(|(k, _)| card_value2(*k)).max().unwrap();
    // let highest_card = value_to_card(highest_value);

    let joker_count = values.get(&'J').unwrap_or(&0).clone();
    // values.entry(highest_card).and_modify(|v| *v += joker_count);
    // values.remove(&'J');

    // let sum = values.values().sum::<u32>();
    // if sum < 5 || *values.get(&'J').unwrap_or(&0) == 4 {
    //     println!(
    //         "{highest_card} - {highest_card} - {joker_count} - {hand} -> sum: {}",
    //         sum
    //     );
    // }

    if joker_count == 5 {
        println!("empty values, all Jokers");
        return HandValue::FiveOfAKind;
    }

    if joker_count == 4 {
        return HandValue::FiveOfAKind;
    }

    if joker_count == 3 {
        return HandValue::FourOfAKind;
    }

    if joker_count == 2 {
        if values.values().filter(|v| **v == 2).count() == 1 {
            return HandValue::FourOfAKind;
        }

        if values.values().filter(|v| **v == 3).count() == 1 {
            return HandValue::FiveOfAKind;
        }

        return HandValue::ThreeOfAKind;
    }

    if joker_count == 1 {
        if values.values().filter(|v| **v == 2).count() == 1 {
            return HandValue::ThreeOfAKind;
        }

        if values.values().filter(|v| **v == 3).count() == 1 {
            return HandValue::FullHouse;
        }

        if values.values().all(|v| *v == 1) {
            return HandValue::OnePair;
        }
    }

    if values.values().any(|v| *v == 5) {
        return HandValue::FiveOfAKind;
    }

    if values.values().any(|v| *v == 4) {
        return HandValue::FourOfAKind;
    }

    if values.values().any(|v| *v == 3) && values.values().any(|v| *v == 2) {
        return HandValue::FullHouse;
    }

    if values.values().any(|v| *v == 3) {
        return HandValue::ThreeOfAKind;
    }

    let pairs = values.values().filter(|v| **v == 2).count();
    if pairs == 2 {
        return HandValue::TwoPair;
    }

    if pairs == 1 {
        return HandValue::OnePair;
    }

    HandValue::HighCard
}
