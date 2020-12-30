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
enum Action {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn part1(input: &str) -> Result<()> {
    let actions: Vec<Action> = input
        .lines()
        .map(|l| {
            let a = l.chars().next().expect("expected char");
            let v = &l[1..];
            let val: i32 = v.parse().expect("expected number");
            match a {
                'N' => Action::N(val),
                'S' => Action::S(val),
                'E' => Action::E(val),
                'W' => Action::W(val),
                'L' => Action::L(val),
                'R' => Action::R(val),
                'F' => Action::F(val),
                _ => panic!("should not get here."),
            }
        })
        .collect();

    let mut ship = Point { x: 0, y: 0 };
    let mut heading = Action::E(0);
    for a in &actions {
        mov(a, &mut heading, &mut ship);
    }

    let mut manhattan = 0;
    if ship.x >= 0 {
        manhattan += ship.x;
    } else {
        manhattan -= ship.x;
    }

    if ship.y >= 0 {
        manhattan += ship.y;
    } else {
        manhattan -= ship.y;
    }

    // println!("{:?}", ship);
    println!("{:?}", manhattan);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let actions: Vec<Action> = input
        .lines()
        .map(|l| {
            let a = l.chars().next().expect("expected char");
            let v = &l[1..];
            let val: i32 = v.parse().expect("expected number");
            match a {
                'N' => Action::N(val),
                'S' => Action::S(val),
                'E' => Action::E(val),
                'W' => Action::W(val),
                'L' => Action::L(val),
                'R' => Action::R(val),
                'F' => Action::F(val),
                _ => panic!("should not get here."),
            }
        })
        .collect();

    let mut ship = Point { x: 0, y: 0 };
    let mut waypoint = Point { x: 10, y: 1 };
    for a in &actions {
        mov2(a, &mut ship, &mut waypoint);
    }

    let mut manhattan = 0;
    if ship.x >= 0 {
        manhattan += ship.x;
    } else {
        manhattan -= ship.x;
    }

    if ship.y >= 0 {
        manhattan += ship.y;
    } else {
        manhattan -= ship.y;
    }

    // println!("{:?}", ship);
    println!("{:?}", manhattan);
    Ok(())
}

fn mov(a: &Action, heading: &mut Action, ship: &mut Point) {
    match a {
        Action::N(v) => move_dir(&mut Action::N(0), ship, *v),
        Action::S(v) => move_dir(&mut Action::S(0), ship, *v),
        Action::E(v) => move_dir(&mut Action::E(0), ship, *v),
        Action::W(v) => move_dir(&mut Action::W(0), ship, *v),
        Action::L(_) => {
            turn(a, heading);
        }
        Action::R(_) => {
            turn(a, heading);
        }
        Action::F(v) => {
            move_dir(heading, ship, *v);
        }
    }
}

fn mov2(a: &Action, ship: &mut Point, waypoint: &mut Point) {
    match a {
        Action::N(v) => move_dir(&mut Action::N(0), waypoint, *v),
        Action::S(v) => move_dir(&mut Action::S(0), waypoint, *v),
        Action::E(v) => move_dir(&mut Action::E(0), waypoint, *v),
        Action::W(v) => move_dir(&mut Action::W(0), waypoint, *v),
        Action::L(_) => {
            turn2(a, waypoint);
        }
        Action::R(_) => {
            turn2(a, waypoint);
        }
        Action::F(v) => {
            move_ship(ship, waypoint, *v);
        }
    }
}

fn turn(a: &Action, heading: &mut Action) {
    let current = match heading {
        Action::N(_) => 1,
        Action::S(_) => 3,
        Action::E(_) => 0,
        Action::W(_) => 2,
        _ => panic!("should not get here...{:?}", heading),
    };

    let n = match a {
        Action::L(v) => (current + (v / 90)).abs() % 4,
        Action::R(v) => ((current - (v / 90)) + 4).abs() % 4,
        _ => panic!("should not get here."),
    };

    *heading = match n {
        0 => Action::E(0),
        1 => Action::N(0),
        2 => Action::W(0),
        3 => Action::S(0),
        _ => panic!("should not get here."),
    }
}

fn turn2(a: &Action, waypoint: &mut Point) {
    let rad = match a {
        Action::L(v) => ((*v) as f32).to_radians(),
        Action::R(v) => ((-(*v)) as f32).to_radians(),
        _ => panic!("should not get here."),
    };

    let x = waypoint.x * rad.cos() as i32 - waypoint.y * rad.sin() as i32;
    let y = waypoint.y * rad.cos() as i32 + waypoint.x * rad.sin() as i32;
    *waypoint = Point {
        x: x as i32,
        y: y as i32,
    };
}

fn move_dir(heading: &mut Action, ship: &mut Point, v: i32) {
    match heading {
        Action::N(_) => ship.y += v,
        Action::S(_) => ship.y -= v,
        Action::E(_) => ship.x += v,
        Action::W(_) => ship.x -= v,
        _ => panic!("should only be directions here"),
    }
}

// fn move_dir2(heading: &mut Action, ship: &mut Point, v: i32) {
//     match heading {
//         Action::N(_) => ship.y += ship.y * v,
//         Action::S(_) => ship.y -= ship.y * v,
//         Action::E(_) => ship.x += ship.x * v,
//         Action::W(_) => ship.x -= ship.x * v,
//         _ => panic!("should only be directions here"),
//     }
// }

fn move_ship(ship: &mut Point, waypoint: &Point, v: i32) {
    ship.x += waypoint.x * v;
    ship.y += waypoint.y * v;
}
