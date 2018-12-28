use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Clone)]
enum Indicator {
    Uninit,
    Obstacle,
    Distance(u32),
}

// x is row y is col
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Position {
        Position { x, y }
    }
    pub fn adjacent(&self) -> Vec<Position> {
        let mut pas = Vec::new();

        if self.x != 0 {
            pas.push(Position::new(self.x - 1, self.y));
        }

        pas.push(Position::new(self.x + 1, self.y));

        if self.y != 0 {
            pas.push(Position::new(self.x, self.y - 1));
        }

        pas.push(Position::new(self.x, self.y + 1));

        pas.sort();

        pas
    }

    pub fn is_adjacent_to(&self, others: Vec<Position>) -> bool {
        for j in others {
            for i in self.adjacent() {
                if i == j {
                    return true;
                }
            }
        }

        false
    }

    // not considering obstacle
    fn closest_distance_to(&self, other: &Position) -> u32 {
        let distance =
            (self.x as i32 - other.x as i32).abs() + (self.y as i32 - other.y as i32).abs();

        distance as u32
    }

    pub fn distance_to<T: Grid>(&self, other: &Position, map: &T) -> Option<u32> {
        struct NextTask {
            position: Position,
            current_distance: u32,
        };

        let mut deque = VecDeque::new();
        deque.push_back(NextTask {
            position: self.clone(),
            current_distance: 0,
        });

        let mut visited_positions = HashSet::new();

        while deque.len() != 0 {
            let task = deque.pop_front().unwrap();
            let position = task.position;
            let current_distance = task.current_distance;

            if visited_positions.contains(&position) {
                continue;
            } else {
                visited_positions.insert(position.clone());
            }

            if *other == position {
                return Some(current_distance);
            }

            if position == self.clone() || map.is_availble_position(&position) {
                for next_from in position.adjacent().iter() {
                    deque.push_back(NextTask {
                        position: next_from.clone(),
                        current_distance: current_distance + 1,
                    })
                }
            } else {
                continue;
            }
        }

        return None;
    }

    pub fn closest_to<T: Grid>(&self, positions: &mut Vec<Position>, map: &T) -> Position {
        positions.sort_by_key(|p| self.distance_to(p, map));
        positions[0].clone()
    }
}

pub trait Grid {
    fn is_availble_position(&self, position: &Position) -> bool;
}

#[test]
fn sort_order() {
    let p1 = Position { x: 0, y: 0 };
    let p2 = Position { x: 0, y: 1 };
    let p3 = Position { x: 1, y: 0 };
    let p4 = Position { x: 1, y: 1 };

    let mut ps = [&p4, &p3, &p2, &p1];
    ps.sort();

    assert_eq!(*ps[0], p1);
    assert_eq!(*ps[1], p2);
    assert_eq!(*ps[2], p3);
    assert_eq!(*ps[3], p4);
}

#[test]
fn adjacent() {
    let p1 = Position { x: 1, y: 1 };

    let p_a_top = Position { x: 0, y: 1 };
    let p_a_left = Position { x: 1, y: 0 };
    let p_a_right = Position { x: 1, y: 2 };
    let p_a_bottom = Position { x: 2, y: 1 };

    let p_as = p1.adjacent();

    assert_eq!(vec![p_a_top, p_a_left, p_a_right, p_a_bottom], p_as);
}

#[test]
fn closest_distance_to() {
    let p0 = Position { x: 0, y: 0 };
    let p1 = Position { x: 2, y: 2 };

    assert_eq!(p0.closest_distance_to(&p1), 4);

    let p0 = Position { x: 1, y: 1 };
    let p1 = Position { x: 0, y: 0 };

    assert_eq!(p0.closest_distance_to(&p1), 2);
}

#[test]
fn distance_to() {
    struct Map {
        map: Vec<Vec<Indicator>>,
    }

    impl Grid for Map {
        fn is_availble_position(&self, position: &Position) -> bool {
            if position.x >= self.map.len() as u32 || position.y >= self.map[0].len() as u32 {
                return false;
            }

            let indicator = &self.map[position.x as usize][position.y as usize];

            if let Indicator::Uninit = indicator {
                true
            } else {
                false
            }
        }
    }

    let map = Map {
        map: vec![
            vec![Indicator::Uninit, Indicator::Uninit],
            vec![Indicator::Uninit, Indicator::Uninit],
        ],
    };

    assert_eq!(
        Position::new(0, 0).distance_to(&Position::new(1, 1), &map),
        Some(2)
    );

    assert_eq!(
        Position::new(0, 0).distance_to(&Position::new(0, 1), &map),
        Some(1)
    );

    // test algorithm supports current position is Obstacle
    let map = Map {
        map: vec![
            vec![Indicator::Obstacle, Indicator::Uninit],
            vec![Indicator::Uninit, Indicator::Uninit],
        ],
    };

    assert_eq!(
        Position::new(0, 0).distance_to(&Position::new(1, 1), &map),
        Some(2)
    );

    let map = Map {
        map: vec![
            vec![Indicator::Uninit, Indicator::Obstacle],
            vec![Indicator::Obstacle, Indicator::Uninit],
        ],
    };

    assert_eq!(
        Position::new(0, 0).distance_to(&Position::new(1, 1), &map),
        None
    );

    let map = Map {
        map: vec![
            vec![
                Indicator::Uninit,
                Indicator::Uninit,
                Indicator::Uninit,
                Indicator::Obstacle,
                Indicator::Uninit,
            ],
            vec![
                Indicator::Uninit,
                Indicator::Uninit,
                Indicator::Uninit,
                Indicator::Obstacle,
                Indicator::Uninit,
            ],
            vec![
                Indicator::Uninit,
                Indicator::Uninit,
                Indicator::Uninit,
                Indicator::Obstacle,
                Indicator::Uninit,
            ],
        ],
    };

    // #######
    // #f.t#.#
    // #...#.#
    // #...#.#
    // #######
    assert_eq!(
        Position::new(0, 0).distance_to(&Position::new(0, 2), &map),
        Some(2)
    );

    // #######
    // #f..#.#
    // #...#t#
    // #...#.#
    // #######
    assert_eq!(
        Position::new(0, 0).distance_to(&Position::new(1, 4), &map),
        None
    );
}

#[test]
fn is_adjacent_to() {
    assert!(Position::new(0, 0).is_adjacent_to(vec![Position::new(1, 0)]));
    assert!(!Position::new(0, 0).is_adjacent_to(vec![Position::new(1, 1)]));
    assert!(!Position::new(0, 0).is_adjacent_to(vec![Position::new(1, 1), Position::new(0, 0)]));
    assert!(Position::new(0, 0).is_adjacent_to(vec![Position::new(1, 1), Position::new(0, 1)]));
}
