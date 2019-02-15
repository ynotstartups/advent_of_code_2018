extern crate regex;

use regex::Regex;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct ClayPositions {
    positions: HashSet<Position>,
}

impl ClayPositions {
    fn new() -> ClayPositions {
        ClayPositions {
            positions: HashSet::new(),
        }
    }

    fn insert(&mut self, position: Position) {
        self.positions.insert(position);
    }
}

fn parse(clays: &str) -> ClayPositions {
    let re = Regex::new(r"\d+").unwrap();
    let mut cp = ClayPositions::new();

    for line in clays.lines() {
        if line.len() == 0 {
            continue;
        }

        let mut number_iter = re.captures_iter(line);
        let n0 = number_iter.next().unwrap()[0].parse::<u32>().unwrap();
        let n1 = number_iter.next().unwrap()[0].parse::<u32>().unwrap();
        let n2 = number_iter.next().unwrap()[0].parse::<u32>().unwrap();

        if line.starts_with('x') {
            let x = n0;
            for y in n1..n2 {
                let p = Position { x, y };
                cp.insert(p);
            }
        } else if line.starts_with('y') {
            let y = n0;
            for x in n1..n2 {
                let p = Position { x, y };
                cp.insert(p);
            }
        }
    }

    println!("ClayPositions {:?}", cp);

    cp
}

fn main() {
    let clays = r#"
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
"#;

    parse(clays);
}
