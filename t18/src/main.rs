use std::collections::HashMap;
use std::fmt;
use std::fs;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Type {
    OpenGround,
    Trees,
    Lumberyards,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::OpenGround => write!(f, "."),
            Type::Trees => write!(f, "|"),
            Type::Lumberyards => write!(f, "#"),
        }
    }
}

fn char_to_type(character: char) -> Type {
    use Type::*;

    if character == '.' {
        OpenGround
    } else if character == '#' {
        Lumberyards
    } else if character == '|' {
        Trees
    } else {
        panic!("unknown type")
    }
}

#[derive(Debug)]
struct Types {
    neighbours_types: Vec<Type>,
}

impl Types {
    fn more_than_three_trees(self) -> bool {
        return self
            .neighbours_types
            .iter()
            .filter(|&t| *t == Type::Trees)
            .count()
            >= 3;
    }

    fn more_than_three_lumberyard(self) -> bool {
        return self
            .neighbours_types
            .iter()
            .filter(|&t| *t == Type::Lumberyards)
            .count()
            >= 3;
    }

    fn remain_lumberyard(self) -> bool {
        return self
            .neighbours_types
            .iter()
            .filter(|&t| *t == Type::Lumberyards)
            .count()
            >= 1
            && self
                .neighbours_types
                .iter()
                .filter(|&t| *t == Type::Trees)
                .count()
                >= 1;
    }
}

fn next_life(current_type: Type, neighbours_types: Types) -> Type {
    use Type::*;

    match current_type {
        OpenGround => {
            if neighbours_types.more_than_three_trees() {
                Type::Trees
            } else {
                Type::OpenGround
            }
        }
        Trees => {
            if neighbours_types.more_than_three_lumberyard() {
                Type::Lumberyards
            } else {
                Type::Trees
            }
        }
        Lumberyards => {
            if neighbours_types.remain_lumberyard() {
                Type::Lumberyards
            } else {
                Type::OpenGround
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn neighbour_positions(&self) -> [Position; 8] {
        return [
            Position {
                x: self.x - 1,
                y: self.y - 1,
            },
            Position {
                x: self.x - 1,
                y: self.y,
            },
            Position {
                x: self.x - 1,
                y: self.y + 1,
            },
            Position {
                x: self.x,
                y: self.y - 1,
            },
            Position {
                x: self.x,
                y: self.y + 1,
            },
            Position {
                x: self.x + 1,
                y: self.y - 1,
            },
            Position {
                x: self.x + 1,
                y: self.y,
            },
            Position {
                x: self.x + 1,
                y: self.y + 1,
            },
        ];
    }
}

fn main() {
    let data = r#".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
"#;

    let data = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    let mut map = HashMap::new();

    let mut x = 0;
    let max_x = data.lines().count();
    let max_y = data.lines().next().unwrap().chars().count();

    for line in data.lines() {
        let mut y = 0;
        for character in line.chars() {
            map.insert(Position { x: x, y: y }, char_to_type(character));
            y += 1;
        }
        x += 1;
    }

    let mut previous_map = map;

    for i in 1..100000000 {
        let mut next_map = HashMap::new();
        for (k, v) in &previous_map {
            let neighbours_types = Types {
                neighbours_types: k
                    .neighbour_positions()
                    .into_iter()
                    .filter_map(|position| previous_map.get(position))
                    .map(|position| position.clone())
                    .collect::<Vec<_>>(),
            };

            let next_type = next_life(*v, neighbours_types);
            next_map.insert((*k).clone(), next_type);
        }

        // for x in 0..max_x {
        //     for y in 0..max_y {
        //         print!(
        //             "{}",
        //             next_map
        //                 .get(&Position {
        //                     x: x as i32,
        //                     y: y as i32
        //                 })
        //                 .unwrap()
        //         );
        //     }
        //     println!();
        // }

        previous_map = next_map;

        // println!();

        let number_of_trees = previous_map.values().filter(|v| **v == Type::Trees).count();
        let number_of_lumberyard = previous_map
            .values()
            .filter(|v| **v == Type::Lumberyards)
            .count();

        println!(
            "index {} results {}",
            i,
            number_of_trees * number_of_lumberyard
        );
    }
}
