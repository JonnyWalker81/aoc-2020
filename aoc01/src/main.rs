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

fn part1(input: &str) -> Result<()> {
    let values: Vec<i32> = input
        .lines()
        .map(|l| l.parse().expect("Expected a number"))
        .collect();

    let mut s = HashSet::new();
    for v in &values {
        s.insert(v);
    }

    for v in &values {
        let lookup = 2020 - v;
        if s.contains(&lookup) {
            let p = lookup * v;
            println!("{}", p);
            break;
        }
    }

    Ok(())
}

// for i := 0; i < len(array) - 2; i++ {
// 	left := i + 1
// 	right := len(array) - 1
// 		for left < right {

// 		sum := array[i] + array[left] + array[right]
// 		if sum == target {
// 			result = append(result, []int{array[i], array[left], array[right]})
// 			left++
// 			right--
// 		} else if sum > target {
// 			right--
// 		} else if sum < target {
// 			left++
// 		}
// 		}

// 	}

fn part2(input: &str) -> Result<()> {
    let values: Vec<i32> = input
        .lines()
        .map(|l| l.parse().expect("Expected a number"))
        .collect();

    for i in 0..values.len() {
        for j in 0..values.len() {
            for k in 0..values.len() {
                let sum = values[i] + values[j] + values[k];
                if sum == 2020 {
                    let p = values[i] * values[j] * values[k];
                    println!("{}", p);
                    break;
                }
            }
        }
    }

    // let mut s = HashSet::new();
    // for v in &values {
    //     s.insert(v);
    // }

    // for v in &values {
    //     let lookup = 2020 - v;
    //     if s.contains(&lookup) {
    //         let p = lookup * v;
    //         println!("{}", p);
    //         break;
    //     }
    // }

    // for i in 0..values.len() - 1 {
    //     let mut left = i;
    //     let mut right = values.len() - 1;
    //     let mut sum = 0;
    //     for j in left + 1..right {
    //         // if left >= right {
    //         //     break;
    //         // }

    //         sum = values[j] + values[left] + values[right];
    //         if sum == 2020 {
    //             let p = values[j] * values[left] * values[right];
    //             println!("{}", p);
    //         }
    //     }

    //     left += 1;
    //     right -= 1;
    // }

    Ok(())
}
