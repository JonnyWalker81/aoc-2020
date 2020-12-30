use std::collections::{HashMap, HashSet};
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
    let mut group = String::new();
    let mut groups = vec![];
    for l in input.lines() {
        if l.is_empty() {
            groups.push(group.to_string());
            group.clear();
        }

        group += l;
    }

    if !group.is_empty() {
        groups.push(group);
    }

    let mut sum = 0;
    for g in &groups {
        let mut seen = HashSet::new();
        for c in g.chars() {
            seen.insert(c);
        }

        sum += seen.len();
    }

    println!("{}", sum);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut group = vec![];
    let mut groups = vec![];
    for l in input.lines() {
        if l.is_empty() {
            groups.push(group.clone());
            group.clear();
            continue;
        }

        group.push(l);
    }

    if !group.is_empty() {
        groups.push(group);
    }

    let mut sum = 0;
    for g in &groups {
        let group_count = g.len();
        let mut seen = HashMap::new();
        for a in g {
            for c in a.chars() {
                *seen.entry(c).or_insert(0) += 1;
            }
        }

        for (_, v) in seen {
            if v == group_count {
                sum += 1;
            }
        }
    }

    println!("{}", sum);

    Ok(())
}
