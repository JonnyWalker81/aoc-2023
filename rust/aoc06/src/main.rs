use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    // let mut input = String::new();
    // io::stdin().read_to_string(&mut input)?;

    part1()?;
    part2()?;

    Ok(())
}

// Time:       58     81     96     76
// Distance:   434   1041   2219   1218

fn part1() -> Result<()> {
    // let input = vec![(7, 9), (15, 40), (30, 200)];
    let input = vec![(58, 434), (81, 1041), (96, 2219), (76, 1218)];

    let mut counts = vec![];
    for r in input {
        let mut count = 0;
        for m in 0..r.0 {
            if (r.0 - m) * m > r.1 {
                count += 1;
            }
        }
        counts.push(count);
    }

    let total = counts.iter().product::<u32>();
    println!("{total}");
    Ok(())
}

fn part2() -> Result<()> {
    // let input = vec![(71530, 940200)];
    let input: Vec<(usize, usize)> = vec![(58819676, 434104122191218)];

    let mut counts = vec![];
    for r in input {
        let mut count = 0;
        for m in 0..r.0 {
            if (r.0 - m) * m > r.1 {
                count += 1;
            }
        }
        counts.push(count);
    }

    let total = counts.iter().product::<u32>();
    println!("{total}");
    Ok(())
}
