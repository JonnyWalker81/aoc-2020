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

#[derive(Debug, Clone)]
struct Password {
    range: (i32, i32),
    req: char,
    password: String,
}

fn part1(input: &str) -> Result<()> {
    let passwords: Vec<Password> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let range: Vec<&str> = parts[0].split('-').collect();
            let start = range[0].parse().expect("Should be a number");
            let end = range[1].parse().expect("Should be a number");
            let s = parts[1].trim_matches(':');
            let c = s.chars().nth(0).expect("should be a char");
            let password = parts[2].trim();
            Password {
                range: (start, end),
                req: c,
                password: password.to_string(),
            }
        })
        .collect();

    let mut valid = vec![];
    for p in passwords {
        let mut count = 0;
        for c in p.password.chars() {
            if c == p.req {
                count += 1;
            }
        }

        if count >= p.range.0 && count <= p.range.1 {
            valid.push(p.clone());
        }
    }

    println!("{}", valid.len());
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let passwords: Vec<Password> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let range: Vec<&str> = parts[0].split('-').collect();
            let start = range[0].parse().expect("Should be a number");
            let end = range[1].parse().expect("Should be a number");
            let s = parts[1].trim_matches(':');
            let c = s.chars().next().expect("should be a char");
            let password = parts[2].trim();
            Password {
                range: (start, end),
                req: c,
                password: password.to_string(),
            }
        })
        .collect();

    let mut valid = vec![];
    for p in passwords {
        let mut count = 0;
        for (i, c) in p.password.chars().enumerate() {
            if c == p.req && ((i + 1) as i32 == p.range.0 || (i + 1) as i32 == p.range.1) {
                count += 1;
            }
        }

        if count == 1 {
            valid.push(p)
        }
    }

    println!("{}", valid.len());
    Ok(())
}
