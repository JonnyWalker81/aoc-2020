use std::collections::HashMap;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    // let input = vec![0, 3, 6];
    let input = vec![1, 0, 15, 2, 10, 13];
    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &[i64]) -> Result<()> {
    let mut turns: HashMap<i64, i64> = HashMap::new();
    let end = 2020;

    let mut init = vec![0; input.len()];
    for (i, s) in input.iter().enumerate() {
        init[i] = *s;
    }

    let mut last = -1;
    for t in 0..end {
        let mut val = 0;
        if t < init.len() {
            val = init[t];
        } else {
            if let Some(n) = turns.get(&last) {
                val = (t as i64 - 1) - n;
            } else {
                val = 0;
            }
        }

        if last >= 0 {
            turns.insert(last, t as i64 - 1);
        }
        last = val;
    }

    println!("{}", last);
    Ok(())
}

// 30000000
fn part2(input: &[i64]) -> Result<()> {
    let mut turns: HashMap<i64, i64> = HashMap::new();
    let end = 30000000;

    let mut init = vec![0; input.len()];
    for (i, s) in input.iter().enumerate() {
        init[i] = *s;
    }

    let mut last = -1;
    for t in 0..end {
        let mut val = 0;
        if t < init.len() {
            val = init[t];
        } else {
            if let Some(n) = turns.get(&last) {
                val = (t as i64 - 1) - n;
            } else {
                val = 0;
            }
        }

        if last >= 0 {
            turns.insert(last, t as i64 - 1);
        }
        last = val;
    }

    println!("{}", last);
    Ok(())
}
