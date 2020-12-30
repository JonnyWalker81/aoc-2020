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

#[derive(Debug, Clone, Eq, PartialEq)]
enum State {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    LowerLeft,
    LowerRight,
}

fn part1(input: &str) -> Result<()> {
    let mut grid: Vec<Vec<State>> = input
        .lines()
        .map(|l| {
            // let p: Vec<&str> = l.split_whitespace().collect();
            let mut row = vec![];
            for s in l.chars() {
                let state = match s {
                    '.' => State::Floor,
                    'L' => State::Empty,
                    '#' => State::Occupied,
                    _ => panic!("Should not get here: {}", s),
                };
                row.push(state);
            }
            row
        })
        .collect();

    let dirs = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::UpLeft,
        Direction::UpRight,
        Direction::LowerLeft,
        Direction::LowerRight,
    ];

    let mut loop_count = 0;
    'outer: loop {
        // if loop_count >= 3 {
        //     break;
        // }

        loop_count += 1;
        let mut snapshot = grid.clone();
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                // let mut empty_count = 0;
                let mut occupied_count = 0;
                for d in &dirs {
                    let s = state(
                        &grid,
                        row as i32,
                        col as i32,
                        grid.len(),
                        grid[row].len(),
                        d,
                    );

                    // if s == State::Empty {
                    //     empty_count += 1;
                    // } else
                    if s == State::Occupied {
                        occupied_count += 1;
                    }
                }
                let cur = &grid[row][col];
                if *cur == State::Empty && occupied_count == 0 {
                    snapshot[row][col] = State::Occupied;
                } else if *cur == State::Occupied && occupied_count >= 4 {
                    snapshot[row][col] = State::Empty;
                }
            }
        }

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if snapshot[row][col] != grid[row][col] {
                    // println!("Here...");
                    grid = snapshot;
                    continue 'outer;
                }
            }
        }

        // println!("End...");
        break;
    }

    let mut count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == State::Occupied {
                count += 1;
            }
        }
    }

    // println!("{:#?}", grid);
    println!("{:#?}", count);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut grid: Vec<Vec<State>> = input
        .lines()
        .map(|l| {
            // let p: Vec<&str> = l.split_whitespace().collect();
            let mut row = vec![];
            for s in l.chars() {
                let state = match s {
                    '.' => State::Floor,
                    'L' => State::Empty,
                    '#' => State::Occupied,
                    _ => panic!("Should not get here: {}", s),
                };
                row.push(state);
            }
            row
        })
        .collect();

    let dirs = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::UpLeft,
        Direction::UpRight,
        Direction::LowerLeft,
        Direction::LowerRight,
    ];

    let mut loop_count = 0;
    'outer: loop {
        if loop_count >= 1 {
            break;
        }

        loop_count += 1;
        let mut snapshot = grid.clone();
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                // let mut empty_count = 0;
                let mut occupied_count = 0;
                for d in &dirs {
                    let s = state2(
                        &grid,
                        row as i32,
                        col as i32,
                        grid.len(),
                        grid[row].len(),
                        d,
                    );

                    // if s == State::Empty {
                    //     empty_count += 1;
                    // } else
                    if s == State::Occupied {
                        occupied_count += 1;
                    }
                }
                let cur = &grid[row][col];
                if *cur == State::Empty && occupied_count == 0 {
                    snapshot[row][col] = State::Occupied;
                } else if *cur == State::Occupied && occupied_count >= 4 {
                    snapshot[row][col] = State::Empty;
                }
            }
        }

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if snapshot[row][col] != grid[row][col] {
                    // println!("Here...");
                    grid = snapshot;
                    continue 'outer;
                }
            }
        }

        // println!("End...");
        break;
    }

    let mut count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == State::Occupied {
                count += 1;
            }
        }
    }

    println!("{:#?}", grid);
    println!("{:#?}", count);
    Ok(())
}

fn state(
    grid: &[Vec<State>],
    mut row: i32,
    mut col: i32,
    rows: usize,
    cols: usize,
    dir: &Direction,
) -> State {
    match dir {
        Direction::Up => {
            row -= 1;
            if row < 0 {
                return State::Floor;
            }
        }
        Direction::Down => {
            row += 1;
            if row >= rows as i32 {
                return State::Floor;
            }
        }
        Direction::Left => {
            col -= 1;
            if col < 0 {
                return State::Floor;
            }
        }
        Direction::Right => {
            col += 1;
            if col >= cols as i32 {
                return State::Floor;
            }
        }
        Direction::UpLeft => {
            col -= 1;
            row -= 1;
            if col < 0 {
                return State::Floor;
            }

            if row < 0 {
                return State::Floor;
            }
        }
        Direction::UpRight => {
            col += 1;
            row -= 1;
            if col >= cols as i32 {
                return State::Floor;
            }

            if row < 0 {
                return State::Floor;
            }
        }
        Direction::LowerLeft => {
            col -= 1;
            row += 1;
            if col < 0 {
                return State::Floor;
            }

            if row >= rows as i32 {
                return State::Floor;
            }
        }
        Direction::LowerRight => {
            col += 1;
            row += 1;
            if col >= cols as i32 {
                return State::Floor;
            }

            if row >= rows as i32 {
                return State::Floor;
            }
        }
    }

    grid[row as usize][col as usize].clone()
}

fn state2(
    grid: &[Vec<State>],
    mut row: i32,
    mut col: i32,
    rows: usize,
    cols: usize,
    dir: &Direction,
) -> State {
    loop {
        match dir {
            Direction::Up => {
                row -= 1;
                if row < 0 {
                    return State::Floor;
                }
            }
            Direction::Down => {
                row += 1;
                if row >= rows as i32 {
                    return State::Floor;
                }
            }
            Direction::Left => {
                col -= 1;
                if col < 0 {
                    return State::Floor;
                }
            }
            Direction::Right => {
                col += 1;
                if col >= cols as i32 {
                    return State::Floor;
                }
            }
            Direction::UpLeft => {
                col -= 1;
                row -= 1;
                if col < 0 {
                    return State::Floor;
                }

                if row < 0 {
                    return State::Floor;
                }
            }
            Direction::UpRight => {
                col += 1;
                row -= 1;
                if col >= cols as i32 {
                    return State::Floor;
                }

                if row < 0 {
                    return State::Floor;
                }
            }
            Direction::LowerLeft => {
                col -= 1;
                row += 1;
                if col < 0 {
                    return State::Floor;
                }

                if row >= rows as i32 {
                    return State::Floor;
                }
            }
            Direction::LowerRight => {
                col += 1;
                row += 1;
                if col >= cols as i32 {
                    return State::Floor;
                }

                if row >= rows as i32 {
                    return State::Floor;
                }
            }
        }

        let c = &grid[row as usize][col as usize];

        if *c != State::Floor {
            return c.clone();
        }

        if row >= rows as i32 && col >= cols as i32 {
            return State::Floor;
        }
    }
}
