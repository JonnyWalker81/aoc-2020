use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Bag {
    name: String,
    contains: i32,
}

fn get_bags(input: &str) -> HashMap<String, HashSet<Bag>> {
    let mut bags: HashMap<String, HashSet<Bag>> = HashMap::new();
    for l in input.lines() {
        let words: Vec<&str> = l.split_whitespace().collect();
        let bag_name = &words[0..2].join(" ");
        let contains = match words[4].parse() {
            Ok(n) => n,
            Err(_) => 0,
        };

        bags.insert(bag_name.clone(), HashSet::new());
        if contains > 0 {
            let index = 4;
            for i in (index..words.len()).step_by(4) {
                let c: i32 = words[i].parse().expect("expected number");
                let b = &words[i + 1..i + 3].join(" ");
                bags.entry(bag_name.clone()).and_modify(|e| {
                    e.insert(Bag {
                        name: b.to_string(),
                        contains: c,
                    });
                });
            }
        }
    }

    bags
}

fn part1(input: &str) -> Result<()> {
    // println!("{:?}", bags);
    let bags = get_bags(input);

    let mut count = 0;
    for (k, _) in &bags {
        if k != "shiny gold" {
            let mut queue: VecDeque<&str> = VecDeque::new();
            queue.push_back(k);
            loop {
                if queue.is_empty() {
                    break;
                }

                let name = queue.pop_front().expect("expected name");
                if name == "shiny gold" {
                    count += 1;
                    break;
                }
                let contains = &bags[name];
                for n in contains {
                    queue.push_back(&n.name);
                }
            }
        }
    }

    println!("{}", count);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let bags = get_bags(input);
    let total = traverse("shiny gold", &bags);
    println!("{}", total);

    Ok(())
}

fn traverse(key: &str, bags: &HashMap<String, HashSet<Bag>>) -> i32 {
    let contains = &bags[key];
    let mut total = 0;
    for b in contains {
        total += b.contains + (b.contains * traverse(&b.name, bags));
    }

    total
}
