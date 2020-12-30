use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let weakness = part1(&input)?;
    part2(&input, weakness)?;

    Ok(())
}

fn find_two(values: &[i64], target: i64) -> (i64, i64) {
    let mut s = HashSet::new();
    for v in values {
        s.insert(v);
    }

    let last = values.last().unwrap();

    // for i in 0..values.len() {
    // let cur = values[i];
    for v in values {
        let lookup = target - v;
        if s.contains(&lookup) {
            let p = lookup * v;
            // println!("{}", p);
            return (p, *v);
        }
    }
    // }

    (-1, -1)
}

fn part1(input: &str) -> Result<i64> {
    let values: Vec<i64> = input
        .lines()
        .map(|l| l.parse().expect("Expected a number"))
        .collect();

    for i in 0..values.len() - 25 {
        let (num1, num2) = find_two(&values[i..i + 25], values[i + 25]);
        if num1 == -1 && num2 == -1 {
            let v = values[i + 25];
            println!("{}", v);
            return Ok(v);
        }
    }

    Ok(0)
}

fn part2(input: &str, weakness: i64) -> Result<()> {
    let values: Vec<i64> = input
        .lines()
        .map(|l| l.parse().expect("Expected a number"))
        .collect();

    for i in 0..values.len() {
        let v = values[i];
        let mut rolling = v;
        let mut high = v;
        let mut low = v;

        let mut n = 1;
        loop {
            let current = values[n + i];
            rolling += current;
            if high < current {
                high = current;
            }
            if low > current {
                low = current;
            }

            if rolling == weakness {
                println!("{}", high + low);
                return Ok(());
            }

            if rolling > weakness {
                break;
            }

            n += 1;
        }
    }

    Ok(())
}
