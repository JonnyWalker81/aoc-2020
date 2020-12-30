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
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|l| {
            let mut line = vec![];
            for c in l.chars() {
                line.push(c);
            }

            line
        })
        .collect();

    let mut row = 1;
    let mut col = 3;
    let mut trees = 0;
    loop {
        if row >= grid.len() {
            break;
        }
        // row = row % grid.len();
        col %= grid[row].len();

        let c = grid[row][col];
        if c == '#' {
            trees += 1;
        }

        row += 1;
        col += 3;
    }

    println!("{}", trees);
    Ok(())
}

struct Slope {
    row: usize,
    col: usize,
}

fn part2(input: &str) -> Result<()> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|l| {
            let mut line = vec![];
            for c in l.chars() {
                line.push(c);
            }

            line
        })
        .collect();

    let slopes = vec![
        Slope { row: 1, col: 1 },
        Slope { row: 1, col: 3 },
        Slope { row: 1, col: 5 },
        Slope { row: 1, col: 7 },
        Slope { row: 2, col: 1 },
    ];

    let mut counts = vec![];
    for s in slopes {
        let mut row = s.row;
        let mut col = s.col;
        let mut trees = 0;
        loop {
            if row >= grid.len() {
                break;
            }
            // row = row % grid.len();
            col %= grid[row].len();

            let c = grid[row][col];
            if c == '#' {
                trees += 1;
            }

            row += s.row;
            col += s.col;
        }

        counts.push(trees);
    }

    let p: usize = counts.iter().product();
    println!("{}", p);
    Ok(())
}
