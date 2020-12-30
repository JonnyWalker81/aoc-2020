use std::collections::HashMap;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let adaptors: Vec<i32> = input
        .lines()
        .map(|l| l.parse().expect("Expected a number"))
        .collect();

    let max = adaptors.iter().max().expect("expected max value.") + 3;

    let mut current = 0;
    let mut diffs = HashMap::new();

    Ok(())
}
