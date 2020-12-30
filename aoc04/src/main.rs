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

fn collect_passports(input: &str) -> Vec<HashMap<String, String>> {
    let mut passport_inputs = vec![];
    let mut current = String::new();
    for s in input.lines() {
        if s.is_empty() {
            passport_inputs.push(current.clone());
            current.clear();
        }

        current += s;
        current.push(' ');
    }

    if !current.is_empty() {
        passport_inputs.push(current);
    }

    // println!("{:?}", passport_inputs);

    let mut passports: Vec<HashMap<String, String>> = vec![];
    for p in &passport_inputs {
        let parts: Vec<&str> = p.split_whitespace().collect();
        let mut map = HashMap::new();
        for prt in parts {
            let data: Vec<&str> = prt.split(':').collect();
            map.insert(data[0].to_string(), data[1].to_string());
        }
        passports.push(map);
    }

    // println!("{:#?}", passports);
    // println!("{}", passports.len());

    passports
}

fn part1(input: &str) -> Result<()> {
    let passports = collect_passports(input);
    let mut valid = 0;
    for p in passports {
        if p.len() == 8 {
            valid += 1;
        } else {
            if p.len() == 7 {
                if !p.contains_key("cid") {
                    valid += 1;
                }
            }
        }
    }

    println!("{}", valid);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let passports = collect_passports(input);
    let mut filtered = vec![];
    for p in &passports {
        if p.len() == 8 {
            filtered.push(p)
        } else {
            if p.len() == 7 {
                if !p.contains_key("cid") {
                    filtered.push(p);
                }
            }
        }
    }

    let mut valid = vec![];
    for p in filtered {
        let mut is_valid = true;
        if let Some(byr) = p.get("byr") {
            let year: i32 = byr.parse().expect("Expected number");
            if !(year >= 1920 && year <= 2002) {
                println!("byr: {}", year);
                is_valid = false;
            }
        }

        if let Some(iyr) = p.get("iyr") {
            let year: i32 = iyr.parse().expect("Expected number");
            if !(year >= 2010 && year <= 2020) {
                println!("iyr: {}", year);
                is_valid = false;
            }
        }

        if let Some(eyr) = p.get("eyr") {
            let year: i32 = eyr.parse().expect("Expected number");
            if !(year >= 2020 && year <= 2030) {
                println!("eyr: {}", year);
                is_valid = false;
            }
        }

        if let Some(hgt) = p.get("hgt") {
            let mut s = String::new();
            for c in hgt.chars() {
                if c.is_digit(10) {
                    s.push(c);
                } else {
                    break;
                }
            }
            let height: i32 = s.parse().expect("Expected number");
            if hgt.ends_with("cm") {
                if !(height >= 150 && height <= 193) {
                    println!("height: {}, {}", height, hgt);
                    is_valid = false;
                }
            } else if hgt.ends_with("in") {
                if !(height >= 59 && height <= 76) {
                    println!("height: {}, {}", height, hgt);
                    is_valid = false;
                }
            } else {
                is_valid = false;
            }
        }

        if let Some(hcl) = p.get("hcl") {
            if hcl.starts_with("#") {
                let mut v = String::new();
                for c in hcl.chars().skip(1) {
                    if c.is_ascii_hexdigit() {
                        v.push(c);
                    } else {
                        break;
                    }
                }

                if v.len() != 6 {
                    println!("hcl: {}", v);
                    is_valid = false;
                }
            } else {
                println!("hcl: {}", hcl);
                is_valid = false;
            }
        }

        if let Some(ecl) = p.get("ecl") {
            // amb blu brn gry grn hzl oth
            match ecl.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
                _ => {
                    is_valid = false;
                    println!("ecl: {}", ecl);
                }
            }
        }

        if let Some(pid) = p.get("pid") {
            let mut n = String::new();
            for c in pid.chars() {
                if c.is_digit(10) {
                    n.push(c);
                } else {
                    break;
                }
            }

            if n.len() != 9 {
                println!("pid: {}", n);
                is_valid = false;
            }
        }

        println!();

        if is_valid {
            valid.push(p);
        }
    }

    println!("------");
    println!("{:#?}", valid);

    println!("{:?}", valid.len());
    Ok(())
}
