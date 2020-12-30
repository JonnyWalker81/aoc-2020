use std::collections::HashMap;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut adaptors: Vec<usize> = input
        .lines()
        .map(|l| l.parse().expect("Expected a number"))
        .collect();

    adaptors.insert(0, 0);
    adaptors.sort_unstable();

    let max = adaptors.iter().max().expect("expected max value.") + 3;
    adaptors.push(max);

    let mut diffs = HashMap::new();
    for i in 0..adaptors.len() {
        let cur = adaptors[i];
        if adaptors.iter().any(|a| *a == cur + 1) {
            let diff = 1;
            diffs.insert(cur, diff);
        } else if adaptors.iter().any(|a| *a == cur + 2) {
            let diff = 2;
            diffs.insert(cur, diff);
        } else if adaptors.iter().any(|a| *a == cur + 3) {
            let diff = 3;
            diffs.insert(cur, diff);
        }
    }

    let mut one_sum = 0;
    let mut three_sum = 0;

    for (_, v) in diffs {
        if v == 1 {
            one_sum += 1;
        } else if v == 3 {
            three_sum += 1;
        }
    }

    println!("{}", one_sum * three_sum);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut adaptors: Vec<usize> = input
        .lines()
        .map(|l| l.parse().expect("Expected a number"))
        .collect();

    adaptors.insert(0, 0);
    adaptors.sort_unstable();

    let max = adaptors.iter().max().expect("expected max value.") + 3;
    adaptors.push(max);

    let exists: HashMap<usize, usize> = adaptors.iter().enumerate().map(|(i, v)| (*v, i)).collect();
    let mut ways = vec![0; adaptors.len()];
    ways[adaptors.len() - 1] = 1;

    // println!("{:?}", exists);

    for i in (0..=adaptors.len() - 2).rev() {
        let mut sum: usize = 0;
        let v = adaptors[i];
        for d in 1..=3 {
            if let Some(p) = exists.get(&(v + d)) {
                sum += ways[*p];
            }
        }

        ways[i] = sum;
    }

    // println!("{:?}", ways);

    let mut ret = 0;
    for v in 1..=3 {
        if let Some(p) = exists.get(&v) {
            ret += ways[*p];
        }
    }

    println!("{}", ret);
    Ok(())
}
