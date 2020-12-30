use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn get_values(input: &str) -> Vec<i32> {
    let passes: Vec<&str> = input.lines().collect();

    let mut results: Vec<i32> = vec![];
    for p in &passes {
        let mut s = String::new();
        for c in p.chars().take(7) {
            if c == 'F' {
                s.push('0');
            } else {
                s.push('1');
            }
        }
        let row = i32::from_str_radix(&s, 2).expect("expeted a binary number");

        s.clear();
        for c in p.chars().skip(7) {
            if c == 'R' {
                s.push('1');
            } else {
                s.push('0');
            }
        }

        let col = i32::from_str_radix(&s, 2).expect("expected binary col.");
        results.push(row * 8 + col);
    }

    results
}

fn part1(input: &str) -> Result<()> {
    let passes = get_values(input);
    let max = passes.iter().max();

    println!("Max: {:?}", max);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut passes = get_values(input);

    passes.sort_unstable();

    for i in 0..passes.len() - 1 {
        if passes[i + 1] - passes[i] > 1 {
            println!("{}", passes[i] + 1);
            break;
        }
    }

    Ok(())
}
