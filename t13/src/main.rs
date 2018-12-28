use std::cmp::Ordering;
use std::fs;

enum CartTurn {
    Left,
    Straight,
    Right,
}
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Position {
    fn up(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn down(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn left(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn right(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }
}

struct Cart {
    position: Position,
    direction: Direction,
    next_turn: CartTurn,
}

impl Cart {
    fn next(&mut self) {
        match &self.direction {
            Direction::Up => self.position = self.position.up(),
            Direction::Down => self.position = self.position.down(),
            Direction::Left => self.position = self.position.left(),
            Direction::Right => self.position = self.position.right(),
        }
    }

    fn turn_left(&mut self) {
        match &self.direction {
            Direction::Up => self.direction = Direction::Left,
            Direction::Down => self.direction = Direction::Right,
            Direction::Left => self.direction = Direction::Down,
            Direction::Right => self.direction = Direction::Up,
        }
        self.next_turn = CartTurn::Straight;
    }

    fn turn_straight(&mut self) {
        self.next_turn = CartTurn::Right;
    }

    fn turn_right(&mut self) {
        match &self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
        }
        self.next_turn = CartTurn::Left;
    }

    fn update_with_patch(&mut self, current_patch: &Patch) {
        match current_patch.path {
            Path::Empty => panic!("cart on path {:?}", current_patch.position),
            Path::Path => {}
            Path::Intersection => match self.next_turn {
                CartTurn::Left => {
                    self.turn_left();
                }
                CartTurn::Straight => {
                    self.turn_straight();
                }
                CartTurn::Right => {
                    self.turn_right();
                }
            },
            Path::PathTurn(simplified_turn) => match simplified_turn {
                SimplifiedTurn::TopLeftOrBottomRight => match self.direction {
                    Direction::Right => self.direction = Direction::Up,
                    Direction::Down => self.direction = Direction::Left,
                    Direction::Up => self.direction = Direction::Right,
                    Direction::Left => self.direction = Direction::Down,
                },
                SimplifiedTurn::TopRightOrBottomLeft => match self.direction {
                    Direction::Up => self.direction = Direction::Left,
                    Direction::Left => self.direction = Direction::Up,
                    Direction::Right => self.direction = Direction::Down,
                    Direction::Down => self.direction = Direction::Right,
                },
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum SimplifiedTurn {
    TopLeftOrBottomRight,
    TopRightOrBottomLeft,
}

#[derive(Debug, Copy, Clone)]
enum Path {
    Empty,
    Path,
    Intersection,
    PathTurn(SimplifiedTurn),
}

struct System {
    // vector of rails
    // vector of cart
    carts: Vec<Cart>,
    patch_matrix: Vec<Vec<Patch>>,
}

// individual char in the system string
#[derive(Debug)]
struct Patch {
    position: Position,
    visited: bool,
    path: Path,
}

impl System {
    fn tick(&mut self) {
        self.carts
            .sort_by(|a, b| match a.position.x.cmp(&b.position.x) {
                Ordering::Equal => a.position.y.cmp(&b.position.y),
                other => other,
            });

        println!("len of cart start of tick {}", self.carts.len());
        for cart in &self.carts {
            println!("start of tick position {:?}", cart.position);
        }

        if self.carts.len() == 1 {
            for cart in &self.carts {
                panic!("last cart position {:?}", cart.position);
            }
        }

        let mut cart_index = 0;
        loop {
            let cart = &mut self.carts[cart_index];
            cart.next();
            let current_patch = &self.patch_matrix[cart.position.x][cart.position.y];
            cart.update_with_patch(current_patch);

            let mut remove_index = Vec::new();

            for (index, cart) in &mut self.carts.iter().enumerate() {
                for (o_index, other_cart) in &mut self.carts.iter().enumerate() {
                    if index != o_index && cart.position == other_cart.position {
                        // panic!("find the same cart position {:?}", cart.position);
                        remove_index.push(index);
                        remove_index.push(o_index);
                    }
                }
            }
            remove_index.sort();
            remove_index.dedup();
            remove_index.reverse();

            if remove_index.len() == 0 {
                cart_index += 1
            }

            for index in remove_index {
                self.carts.remove(index);
                println!("removing index {}", index);
                if index < cart_index {
                    cart_index -= 1;
                }
            }

            if cart_index >= self.carts.len() {
                break;
            }
        }
    }

    fn parse(&mut self, input_data: String) {
        // let mut input_data_char_vec: Vec<Vec<Patch>> = Vec::new();

        let input_data_vec: Vec<&str> = input_data.lines().collect();

        for (row_index, line) in input_data_vec.iter().enumerate() {
            let mut patchs: Vec<Patch> = Vec::new();
            let mut path = Path::Empty;
            for (col_index, this_char) in line.chars().enumerate() {
                // println!("{} {}", row_index, col_index);
                let position = Position {
                    x: row_index,
                    y: col_index,
                };

                if this_char == '/' {
                    path = Path::PathTurn(SimplifiedTurn::TopLeftOrBottomRight);
                } else if this_char == '\\' {
                    path = Path::PathTurn(SimplifiedTurn::TopRightOrBottomLeft);
                } else if this_char == '^' {
                    self.carts.push({
                        Cart {
                            position,
                            direction: Direction::Up,
                            next_turn: CartTurn::Left,
                        }
                    });
                    path = Path::Path;
                } else if this_char == '>' {
                    self.carts.push({
                        Cart {
                            position,
                            direction: Direction::Right,
                            next_turn: CartTurn::Left,
                        }
                    });
                    path = Path::Path;
                } else if this_char == '<' {
                    self.carts.push({
                        Cart {
                            position,
                            direction: Direction::Left,
                            next_turn: CartTurn::Left,
                        }
                    });
                    path = Path::Path;
                } else if this_char == 'v' {
                    self.carts.push({
                        Cart {
                            position,
                            direction: Direction::Down,
                            next_turn: CartTurn::Left,
                        }
                    });
                    path = Path::Path;
                } else if this_char == '+' {
                    path = Path::Intersection;
                } else if this_char == '-' || this_char == '|' {
                    path = Path::Path;
                } else if this_char == ' ' {
                    path = Path::Empty;
                } else {
                    println!("this_char {}", this_char);
                    assert_eq!(true, false);
                }

                patchs.push(Patch {
                    path: path,
                    position: position,
                    visited: false,
                });
            }
            self.patch_matrix.push(patchs);
        }

        println!("patch system len {}", self.patch_matrix.len());
        println!("cart num {}", self.carts.len());
    }
}

fn main() {
    let input_data =
        fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    println!("With text:\n{}", input_data);

    let mut system = System {
        carts: Vec::new(),
        patch_matrix: Vec::new(),
    };

    system.parse(input_data);

    loop {
        system.tick();
    }
}
