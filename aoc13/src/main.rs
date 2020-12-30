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
    let mut lines = input.lines();
    let time: i32 = lines
        .next()
        .expect("expected time line.")
        .parse()
        .expect("expected number");
    let bus_ids: Vec<i32> = lines
        .next()
        .expect("expected bus ids")
        .split(',')
        .filter(|l| *l != "x")
        .map(|v| v.parse().expect("expected number."))
        .collect();

    let mut start = time;
    let mut id = 0;
    'outter: loop {
        for b in &bus_ids {
            let is_departing = start % b;
            if is_departing == 0 {
                id = *b;
                break 'outter;
            }
        }
        start += 1;
    }

    println!("{}", (start - time) * id);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut lines = input.lines();
    let _time: i32 = lines
        .next()
        .expect("expected time line.")
        .parse()
        .expect("expected number");

    let bus_ids: Vec<i64> = lines
        .next()
        .expect("expected bus ids")
        .split(',')
        .map(|v| v.parse().unwrap_or(-1))
        .collect();

    let mut min_value: i64 = 0;
    let mut running_product: i64 = 1;
    for (i, id) in bus_ids.iter().enumerate() {
        if *id == -1 {
            continue;
        }

        while (min_value + i as i64) % id != 0 {
            min_value += running_product;
        }
        running_product *= id;
    }

    println!("{}", min_value);

    Ok(())
}
