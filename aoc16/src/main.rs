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
struct Range {
    field: String,
    start: i32,
    end: i32,
}

fn part1(input: &str) -> Result<()> {
    let mut ranges = vec![];
    let mut lines = input.lines();
    loop {
        if let Some(l) = lines.next() {
            if l.is_empty() {
                break;
            }

            let parts: Vec<&str> = l.split(": ").collect();
            if parts.len() > 0 {
                let rs: Vec<&str> = parts[1].split(" or ").collect();
                for r in rs {
                    let pr: Vec<&str> = r.split('-').collect();
                    let start: i32 = pr[0].parse().expect("expected number.");
                    let end: i32 = pr[1].parse().expect("expected number.");
                    ranges.push(Range {
                        start,
                        end,
                        field: "".to_string(),
                    });
                }
            }
        } else {
            break;
        }
    }

    lines.next();
    let _your_tickets = lines.next().unwrap();

    lines.next();
    lines.next();

    let mut nearby_tickets = vec![];

    loop {
        if let Some(l) = lines.next() {
            if l.is_empty() {
                break;
            }

            l.split(',')
                .map(|s| s.trim().parse().expect("expected number."))
                .for_each(|n: i32| {
                    nearby_tickets.push(n);
                });
        } else {
            break;
        }
    }

    let mut invalid: Vec<i32> = vec![];
    for t in &nearby_tickets {
        let mut found = false;
        for r in &ranges {
            if t >= &r.start && t <= &r.end {
                found = true;
            }
        }

        if !found {
            invalid.push(*t);
        }
    }

    println!("{:?}", invalid.iter().sum::<i32>());
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut ranges: HashMap<String, Vec<Range>> = HashMap::new();
    let mut lines = input.lines();
    let mut input_fields = vec![];
    loop {
        if let Some(l) = lines.next() {
            if l.is_empty() {
                break;
            }

            let parts: Vec<&str> = l.split(": ").collect();
            let f = parts[0].to_string();
            input_fields.push(f.clone());
            if parts.len() > 0 {
                let rs: Vec<&str> = parts[1].split(" or ").collect();
                for r in rs {
                    let pr: Vec<&str> = r.split('-').collect();
                    let start: i32 = pr[0].parse().expect("expected number.");
                    let end: i32 = pr[1].parse().expect("expected number.");
                    let rng = Range {
                        start,
                        end,
                        field: f.clone(),
                    };
                    ranges.entry(f.clone()).or_insert(vec![]).push(rng);
                }
            }
        } else {
            break;
        }
    }

    lines.next();
    let your_tickets: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.trim().parse().expect("expected number"))
        .collect();

    lines.next();
    lines.next();

    let mut nearby_tickets: Vec<Vec<i32>> = vec![];

    loop {
        if let Some(l) = lines.next() {
            if l.is_empty() {
                break;
            }

            let vals: Vec<i32> = l
                .split(',')
                .map(|s| s.trim().parse().expect("expected number."))
                .collect();
            nearby_tickets.push(vals);
        } else {
            break;
        }
    }

    let mut invalid: Vec<i32> = vec![];
    let mut valid: Vec<Vec<i32>> = vec![];
    for n in &nearby_tickets {
        let mut valid_ticket = true;
        for c in n {
            let mut found = false;
            for (_k, v) in &ranges {
                if valid_value(&v, *c) {
                    found = true;
                }
            }

            if !found {
                invalid.push(*c);
                valid_ticket = false;
            }
        }
        if valid_ticket {
            valid.push(n.clone());
        }
    }

    let mut value_pos = HashMap::new();
    for v in &valid {
        for (i, t) in v.iter().enumerate() {
            value_pos.entry(i).or_insert(vec![]).push(t);
        }
    }

    let mut candidates: HashMap<String, Vec<i32>> = HashMap::new();
    for (k, v) in &value_pos {
        'inner: for (rk, rv) in &ranges {
            for c in v {
                if !valid_value(&rv, **c) {
                    continue 'inner;
                }
            }

            candidates
                .entry(rk.to_string())
                .or_insert(vec![])
                .push(*k as i32);
        }
    }

    let mut fields: HashMap<String, i32> = HashMap::new();
    loop {
        let keys: Vec<String> = candidates.keys().cloned().collect();
        let mut changed = false;
        for name in &keys {
            let mappings = &candidates[name];
            if mappings.len() != 1 {
                continue;
            }

            let mut pos = -1;
            for k in mappings {
                pos = *k;
            }

            fields.insert(name.to_string(), pos);

            candidates.remove(name);
            for (_, v) in &mut candidates {
                let index = v.iter().position(|x| *x == pos).unwrap();
                v.remove(index);
            }
            changed = true;
            break;
        }

        if !changed {
            break;
        }
    }

    let mut product: usize = 1;
    for (f, idx) in &fields {
        if f.len() >= 9 && &f[0..9] == "departure" {
            product *= your_tickets[*idx as usize] as usize;
        }
    }

    println!("{}", product);
    Ok(())
}

fn valid_value(ranges: &[Range], c: i32) -> bool {
    for r in ranges {
        if c >= r.start && c <= r.end {
            return true;
        }
    }

    false
}
