use regex::Regex;
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Mask(String),
    Mem(i64, i64),
}

fn parse(input: &str) -> Result<Vec<Instruction>> {
    let mask = Regex::new(r"^mask\s=\s([1|0|X]+)$").unwrap();
    let mem = Regex::new(r"^mem\[(\d+)\]\s=\s(\d+)$").unwrap();
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| {
            if mask.is_match(l) {
                let caps = mask.captures(l).unwrap();
                // println!("Mask...{:?}", &caps[1]);
                Instruction::Mask(caps[1].into())
            } else if mem.is_match(l) {
                let caps = mem.captures(l).unwrap();
                // println!("Mem: {} = {}", &caps[1], &caps[2]);
                Instruction::Mem(
                    caps[1].parse().expect("expected addr number"),
                    caps[2].parse().expect("expected value number"),
                )
            } else {
                panic!("Should not get here...");
            }
        })
        .collect();

    Ok(instructions)
}

fn part1(input: &str) -> Result<()> {
    let instructions = parse(input)?;
    let mut current_mask = String::new();
    let mut memory: HashMap<i64, i64> = HashMap::new();
    for i in &instructions {
        match i {
            Instruction::Mask(m) => current_mask = m.to_string(),
            Instruction::Mem(addr, val) => {
                // println!("{} -> {}", current_mask, current_mask.len());
                let s = format!("{:#038b}", val);
                let v = String::from(&s[2..]);
                // println!("{} -> {}", v, v.len());
                let mut masked_value = String::new();
                for d in 0..current_mask.len() {
                    let m = current_mask.chars().nth(d).unwrap();
                    let mv = v.chars().nth(d).unwrap();
                    let r = if m == 'X' {
                        mv
                    } else if m == '1' {
                        '1'
                    } else if m == '0' {
                        '0'
                    } else {
                        panic!("should not get here...{}", m);
                    };

                    masked_value.push(r);
                }

                let mb = i64::from_str_radix(&masked_value, 2).unwrap();
                // println!("{}", mb);
                memory.insert(*addr, mb);
            }
        }
    }

    let sum: i64 = memory.values().sum();

    println!("{:?}", sum);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let instructions = parse(input)?;

    let mut current_mask = String::new();
    let mut memory: HashMap<i64, i64> = HashMap::new();

    for i in &instructions {
        match i {
            Instruction::Mask(m) => current_mask = m.to_string(),
            Instruction::Mem(addr, val) => {
                let s = format!("{:#038b}", addr);
                let v = String::from(&s[2..]);
                let mut new_mask = String::new();
                for d in 0..current_mask.len() {
                    let m = current_mask.chars().nth(d).unwrap();
                    let mv = v.chars().nth(d).unwrap();
                    let r = if m == 'X' {
                        'X'
                    } else if m == '1' {
                        '1'
                    } else if m == '0' {
                        mv
                    } else {
                        panic!("should not get here...{}", m);
                    };

                    new_mask.push(r);
                }
                let addrs = generate(&new_mask);
                for a in &addrs {
                    let mb = i64::from_str_radix(a, 2).unwrap();
                    memory.insert(mb, *val);
                }
            }
        }
    }

    let sum: i64 = memory.values().sum();

    println!("{:?}", sum);
    Ok(())
}

fn generate(addr: &str) -> Vec<String> {
    if !addr.contains('X') {
        return vec![addr.to_string()];
    }

    let mut addrs = vec![];
    let xidx = addr.find('X').unwrap();
    let s0 = format!("{}0{}", &addr[0..xidx], &addr[xidx + 1..]);
    let s1 = format!("{}1{}", &addr[0..xidx], &addr[xidx + 1..]);
    let addrs0 = generate(&s0);
    let addrs1 = generate(&s1);
    addrs.extend(addrs0);
    addrs.extend(addrs1);
    addrs
}
