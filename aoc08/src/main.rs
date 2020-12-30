use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop,
}

// #[derive(Clone, Debug)]
// struct Instruction {
//     op: Op,
//     argument: i32,
// }

fn get_ops(input: &str) -> Vec<Op> {
    let instructions: Vec<Op> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let op = match parts[0] {
                "acc" => {
                    let arg = parts[1].parse().expect("expected number");
                    // println!("Parsing Acc arg: {}", arg);
                    Op::Acc(arg)
                }
                "jmp" => {
                    let arg = parts[1].parse().expect("expected number");
                    // println!("Parsing Jump arg: {}", arg);
                    Op::Jmp(arg)
                }
                "nop" => Op::Nop,
                _ => Op::Nop,
            };

            op
        })
        .collect();

    instructions
}

fn part1(input: &str) -> Result<()> {
    let instructions = get_ops(input);
    let mut pc = 0;
    let mut acc = 0;
    let mut seen: HashSet<usize> = HashSet::new();
    loop {
        let op = &instructions[pc];
        if seen.contains(&pc) {
            break;
        } else {
            seen.insert(pc);
        }

        match op {
            Op::Acc(arg) => {
                acc += arg;
                pc += 1;
            }
            Op::Jmp(arg) => {
                let mut p = pc as i32;
                p += *arg;
                pc = p as usize;
            }
            Op::Nop => pc += 1,
        }
    }

    println!("{}", acc);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let ops = get_ops(input);
    let mut acc = 0;
    'outer: for (i, o) in ops.iter().enumerate() {
        let mut sim = ops.clone();
        if let Op::Jmp(_) = o {
            sim[i] = Op::Nop;
        } else if let Op::Nop = o {
            sim[i] = Op::Jmp(0);
        } else {
            continue;
        }

        acc = 0;
        let mut pc = 0;
        let mut seen: HashSet<usize> = HashSet::new();
        loop {
            if pc >= ops.len() {
                break 'outer;
            }

            let op = &sim[pc];
            if seen.contains(&pc) {
                break;
            } else {
                seen.insert(pc);
            }

            match op {
                Op::Acc(arg) => {
                    acc += arg;
                    pc += 1;
                }
                Op::Jmp(arg) => {
                    let mut p = pc as i32;
                    p += *arg;
                    pc = p as usize;
                }
                Op::Nop => pc += 1,
            }
        }
    }

    println!("{}", acc);

    Ok(())
}
