use crate::position::Grid;
use crate::position::Position;
use crate::unit::Type;
use crate::unit::Unit;
use std::iter::Zip;

pub struct System {
    map: Vec<Vec<Unit>>,
}

enum Part2_status {
    ElfDie,
    NextTick,
    GameOver(i32),
}

impl Grid for System {
    fn is_availble_position(&self, position: &Position) -> bool {
        match self.get(&position) {
            Some(unit) => {
                return unit.can_be_stand_on();
            }
            None => false,
        }
    }
}

impl System {
    pub fn get_all_units(&self) -> Vec<&Unit> {
        let mut units = Vec::new();
        for unit in self.map.iter().flatten() {
            if unit.is_creature() {
                units.push(unit);
            }
        }

        units
    }

    fn get_units_avail_adjacents<'a>(&'a self, units: &'a Vec<&Unit>) -> Option<Vec<&'a Unit>> {
        let mut adjacents_units: Vec<&Unit> = Vec::new();
        for unit in units {
            for adjacent_positions in unit.adjacent() {
                for position in adjacent_positions {
                    if let Some(u) = self.get(&position) {
                        if u.can_be_stand_on() {
                            adjacents_units.push(u);
                        }
                    }
                }
            }
        }

        if adjacents_units.len() > 0 {
            Some(adjacents_units)
        } else {
            None
        }
    }

    pub fn get_elves(&self) -> Vec<&Unit> {
        let mut units = Vec::new();
        for unit in self.map.iter().flatten() {
            if unit.is_creature() && unit.is_elf() {
                units.push(unit);
            }
        }

        units
    }

    pub fn get_globins(&self) -> Vec<&Unit> {
        let mut units = Vec::new();
        for unit in self.map.iter().flatten() {
            if unit.is_creature() && unit.is_goblin() {
                units.push(unit);
            }
        }

        units
    }

    pub fn get_enemy_units(&self, unit: &Unit) -> Vec<&Unit> {
        if unit.is_goblin() {
            self.get_elves()
        } else {
            self.get_globins()
        }
    }

    pub fn get_adjacent_enemy_units(&self, unit: &Unit) -> Vec<&Unit> {
        let mut adjacent_enemy_units: Vec<&Unit> = Vec::new();
        for adjacent_positions in unit.adjacent() {
            for position in adjacent_positions {
                if let Some(adjacent_unit) = self.get(&position) {
                    if unit.is_enemy_to(adjacent_unit) {
                        adjacent_enemy_units.push(adjacent_unit);
                    }
                }
            }
        }

        adjacent_enemy_units
    }

    fn check_gameover(&self) -> Option<i32> {
        if self.get_elves().len() == 0 {
            let mut total_hp = 0;
            for globin in self.get_globins() {
                total_hp += globin.hp;
            }
            return Some(total_hp);
        } else if self.get_globins().len() == 0 {
            let mut total_hp = 0;
            for elf in self.get_elves() {
                total_hp += elf.hp;
            }
            return Some(total_hp);
        }

        None
    }

    fn remove(&mut self, position: &Position) {
        self.set(Unit::new_cavern(position.clone()), position.clone());
    }

    fn remove_if_dead(&mut self, position: &Position) -> bool {
        if let Some(unit) = self.get_ceature_mut(position) {
            if unit.is_dead() {
                let is_elf = unit.is_elf();
                let unit_position = unit.position.clone();
                self.remove(&unit_position);

                return is_elf;
            }
        }

        false
    }

    fn is_creature_on_position(&self, position: &Position) -> bool {
        if let Some(unit) = self.get(&position) {
            if unit.is_creature() {
                return true;
            }
        }

        false
    }

    pub fn get(&self, position: &Position) -> Option<&Unit> {
        if position.x >= self.map.len() as u32 || position.y >= self.map[0].len() as u32 {
            None
        } else {
            Some(&self.map[position.x as usize][position.y as usize])
        }
    }

    pub fn get_ceature_mut(&mut self, position: &Position) -> Option<&mut Unit> {
        let unit = &mut self.map[position.x as usize][position.y as usize];
        if unit.is_creature() {
            Some(unit)
        } else {
            None
        }
    }

    pub fn set(&mut self, unit: Unit, position: Position) {
        if position.x >= self.map.len() as u32 || position.y >= self.map[0].len() as u32 {
            panic!("if correct position")
        }

        self.map[position.x as usize][position.y as usize] = unit;
    }

    fn swap_unit(&mut self, from: Unit, to: Unit) {
        let from_position = from.position.clone();
        let to_position = to.position.clone();

        let mut from = from.clone();
        from.position = to_position.clone();

        let mut to = to.clone();
        to.position = from_position.clone();

        self.set(from.clone(), to_position.clone());
        self.set(to.clone(), from_position.clone());
    }

    fn move_creature(&mut self, position: Position) -> Option<Position> {
        let mut unit = self.get(&position).unwrap();
        let enemy_units = self.get_enemy_units(unit);

        if !unit.is_adjacent_to(&enemy_units) {
            if let Some(mut availble_units) = self.get_units_avail_adjacents(&enemy_units) {
                if let Some(closest_unit) = unit.closest_reachable_units(&mut availble_units, self)
                {
                    if let Some(adjacent_units) = self.get_units_avail_adjacents(&vec![&unit]) {
                        if let Some(move_to) =
                            closest_unit.closest_reachable_units(&adjacent_units, self)
                        {
                            let move_to_position = move_to.position.clone();

                            self.swap_unit(unit.clone(), move_to.clone());

                            return Some(move_to_position.clone());
                        }
                    }
                }
            }
        }

        None
    }

    fn creature_attack(&mut self, position: Position) -> bool {
        let unit = self.get(&position).unwrap();
        let enemy_units = self.get_enemy_units(&unit);

        if enemy_units.len() == 0 {
            panic!("full round");
        }

        if !unit.is_adjacent_to(&enemy_units) {
            return false;
        }

        let mut enemy_units = self.get_adjacent_enemy_units(&unit);

        enemy_units.sort_by_key(|u| &u.position);
        System::assert_reading_position(&enemy_units);

        enemy_units.sort_by_key(|u| &u.hp);
        let enemy_unit_position = &enemy_units[0].position.clone();

        assert!(unit.is_adjacent_to(&vec![enemy_units[0]]));

        let attack_power = unit.attack_power;

        let enemy_unit_mut = self.get_ceature_mut(enemy_unit_position).unwrap();
        enemy_unit_mut.get_hit(attack_power);

        self.remove_if_dead(&enemy_unit_position)

        // if enemy_unit_mut.is_dead() && enemy_unit_mut.is_elf() {
        //     true
        // } else {
        //     false
        // }
    }

    pub fn part1_tick(&mut self) -> Option<i32> {
        let mut all_units = self.get_all_units();
        all_units.sort_by_key(|u| &u.position);
        System::assert_reading_position(&all_units);

        let mut clone_unit_positions = Vec::new();
        for unit in all_units {
            clone_unit_positions.push(unit.position.clone());
        }

        for position in clone_unit_positions {
            if !self.is_creature_on_position(&position) {
                continue;
            }

            let new_position = self.move_creature(position.clone()).unwrap_or(position);
            if let Some(total_hp) = self.check_gameover() {
                return Some(total_hp);
            }
            self.creature_attack(new_position);
        }

        // self.check_invariant();
        None
    }

    pub fn part2_tick(&mut self) -> Part2_status {
        let mut all_units = self.get_all_units();
        all_units.sort_by_key(|u| &u.position);
        System::assert_reading_position(&all_units);

        let mut clone_unit_positions = Vec::new();
        for unit in all_units {
            clone_unit_positions.push(unit.position.clone());
        }

        for position in clone_unit_positions {
            if !self.is_creature_on_position(&position) {
                continue;
            }

            let new_position = self.move_creature(position.clone()).unwrap_or(position);
            if let Some(total_hp) = self.check_gameover() {
                return Part2_status::GameOver(total_hp);
            }
            let a_elf_die = self.creature_attack(new_position);
            if a_elf_die {
                return Part2_status::ElfDie;
            }
        }

        return Part2_status::NextTick;
    }

    pub fn part1_with_turn_number(&mut self) -> i32 {
        let mut turn = 0;
        loop {
            println!("turn number {}", turn);
            if let Some(total_hp) = self.part1_tick() {
                return total_hp * turn;
            }
            turn += 1;
            self.print_map();
        }
    }

    pub fn part2_with_turn_number(data: String) -> i32 {
        let mut power = 4;
        loop {
            println!("current power {}", power);
            let mut system = System::init_part2(data.to_string(), power);
            let mut rounds = 0;
            loop {
                match system.part2_tick() {
                    Part2_status::NextTick => {}
                    Part2_status::ElfDie => break,
                    Part2_status::GameOver(total_hp) => {
                        return total_hp * rounds as i32;
                    }
                }
                rounds += 1;
            }
            power += 1;
        }
    }

    pub fn init(data: String) -> System {
        System::init_part2(data, 3)
    }

    pub fn init_part2(data: String, elf_attack_power: u32) -> System {
        let mut map = Vec::new();

        for (row, line) in data.lines().enumerate() {
            let mut units = Vec::new();
            for (col, each_char) in line.chars().enumerate() {
                let position = Position::new(row as u32, col as u32);
                let mut unit = Unit::new_temp(position.clone());

                if each_char == '#' {
                    unit = Unit::new_wall(position);
                } else if each_char == '.' {
                    unit = Unit::new_cavern(position);
                } else if each_char == 'E' {
                    unit = Unit::new_elf_with_power(position, elf_attack_power);
                // TODO how to add to unit as a reference
                } else if each_char == 'G' {
                    unit = Unit::new_globin(position);
                } else if each_char == ' ' {
                } else {
                    panic!("Unknown char {}", each_char);
                }

                units.push(unit);
            }

            map.push(units);
        }

        map.iter().flatten().map(|u| {
            if u.is_temp() {
                panic!("should never be temp")
            }
        });

        System { map }
    }

    pub fn print_map(&self) {
        for row in &self.map {
            for col in row {
                match col.get_type() {
                    Type::Goblin => print!("G"),
                    Type::Elf => print!("E"),
                    Type::Cavern => print!("."),
                    Type::Wall => print!("#"),
                    Type::Empty => print!("!"),
                }
            }
            println!();
        }
    }

    fn check_invariant(&self) {
        for (row_number, row) in self.map.iter().enumerate() {
            for (col_number, col) in row.iter().enumerate() {
                let position = Position::new(row_number as u32, col_number as u32);
                let unit = self.get(&position).unwrap();
                if unit.position != position {
                    panic!("Incorrect Invariant");
                }
                if unit.is_creature() && unit.is_dead() {
                    panic!("has a dead unit");
                }
            }
        }
    }

    fn assert_reading_position(units: &Vec<&Unit>) {
        let mut positions = units
            .iter()
            .map(|u| u.position.clone())
            .collect::<Vec<Position>>();

        positions.sort();

        for (unit, position) in units.iter().zip(positions.iter()) {
            assert_eq!(
                unit.position.clone(),
                position.clone(),
                "Not in reading position {:?}",
                units
            );
        }
    }
}

#[test]
fn init() {
    let data = r#"#######
#.G.E.#
#E.G.E#
#.G.E.#
#######
    "#;

    let system = System::init(data.to_string());

    assert_eq!(system.get_globins().len(), 3);
    assert_eq!(system.get_elves().len(), 4);

    let data = r#"#######
#E..G.#
#...#.#
#.G.#G#
#######
    "#;

    let system = System::init(data.to_string());

    assert_eq!(system.get_globins().len(), 3);
    assert_eq!(system.get_elves().len(), 1);
}

#[test]
fn trait_grid() {
    let data = r#"#######
#E..G.#
#...#.#
#.G.#G#
#######
    "#;

    let system = System::init(data.to_string());

    assert!(!system.is_availble_position(&Position::new(0, 0)));
    assert!(system.is_availble_position(&Position::new(1, 2)));
    assert!(!system.is_availble_position(&Position::new(2, 4)));
}

#[test]
fn part1() {
    let data = r#"#######
#E..G.#
#...#.#
#.G.#G#
#######
    "#;

    let mut system = System::init(data.to_string());
    system.part1_tick();

    let unit = system.get(&Position::new(1, 2)).unwrap();
    assert_eq!(*unit.get_type(), Type::Elf);

    let unit = system.get(&Position::new(2, 2)).unwrap();
    assert_eq!(*unit.get_type(), Type::Goblin);

    let unit = system.get(&Position::new(3, 5)).unwrap();
    assert_eq!(*unit.get_type(), Type::Goblin);
}

#[test]
fn get_set() {
    let mut system = System {
        map: vec![vec![Unit::new_temp(Position::new(0, 0))]],
    };

    let unit = system.get(&Position::new(0, 0)).unwrap();
    assert_eq!(*unit.get_type(), Type::Empty);

    system.set(Unit::new_wall(Position::new(0, 1)), Position::new(0, 0));

    let unit = system.get(&Position::new(0, 0)).unwrap();

    assert_eq!(*unit.get_type(), Type::Wall);
}

#[test]
fn move_test() {
    let mut system = System {
        map: vec![vec![
            Unit::new_temp(Position::new(0, 0)),
            Unit::new_wall(Position::new(0, 1)),
        ]],
    };

    let unit_from = system.get(&Position::new(0, 0)).unwrap();
    let unit_to = system.get(&Position::new(0, 1)).unwrap();

    system.swap_unit(unit_from.clone(), unit_to.clone());

    let unit_0 = system.get(&Position::new(0, 0)).unwrap();
    let unit_1 = system.get(&Position::new(0, 1)).unwrap();

    assert_eq!(*unit_0.get_type(), Type::Wall);
    assert_eq!(unit_0.position, Position::new(0, 0));

    assert_eq!(*unit_1.get_type(), Type::Empty);
    assert_eq!(unit_1.position, Position::new(0, 1));
}

#[test]
fn move_test_2() {
    let data = r#"####
#.G#
#..#
#..#
#.E#
####
    "#;
    let mut system = System::init(data.to_string());

    system.part1_tick();
    system.print_map();

    let unit_0 = system.get(&Position::new(2, 2)).unwrap();
    let unit_1 = system.get(&Position::new(3, 2)).unwrap();

    assert_eq!(*unit_0.get_type(), Type::Goblin);
    assert_eq!(*unit_1.get_type(), Type::Elf);
    assert_eq!(unit_0.hp, 200 - 3, "it should move and then attack");
}

#[test]
fn remove_test() {
    let position = Position::new(0, 0);
    let mut system = System {
        map: vec![vec![Unit::new_elf(position.clone())]],
    };

    system.remove(&position);

    let unit_0 = system.get(&position.clone()).unwrap();
    assert_eq!(*unit_0.get_type(), Type::Cavern);
}

#[test]
#[should_panic]
fn assert_reading_position() {
    let elf_0 = Unit::new_elf(Position::new(0, 1));
    let elf_1 = Unit::new_elf(Position::new(0, 0));

    let units = vec![&elf_0, &elf_1];
    System::assert_reading_position(&units);
}

#[test]
#[should_panic]
fn check_invariant() {
    let mut system = System {
        map: vec![vec![Unit::new_elf(Position::new(0, 1))]],
    };

    system.check_invariant();
}

#[test]
fn example_0() {
    let data = r#"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
    "#;
    let mut system = System::init(data.to_string());
    assert_eq!(system.part1_with_turn_number(), 36334);
}

#[test]
fn example_1() {
    let data = r#"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
    "#;
    let mut system = System::init(data.to_string());
    assert_eq!(system.part1_with_turn_number(), 39514);
    assert_eq!(System::part2_with_turn_number(data.to_string()), 31284);
}

#[test]
fn example_2() {
    let data = r#"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
    "#;
    let mut system = System::init(data.to_string());
    assert_eq!(system.part1_with_turn_number(), 27755);
    assert_eq!(System::part2_with_turn_number(data.to_string()), 3478);
}

#[test]
fn example_3() {
    let data = r#"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
    "#;
    let mut system = System::init(data.to_string());
    assert_eq!(system.part1_with_turn_number(), 28944);
    assert_eq!(System::part2_with_turn_number(data.to_string()), 6474);
}

#[test]
fn example_4() {
    use std::fs;

    let data = r#"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
    "#;
    let mut system = System::init(data.to_string());
    assert_eq!(system.part1_with_turn_number(), 18740);

    let data =
        fs::read_to_string("./example_input_0.txt").expect("Something went wrong reading the file");
    let mut system = System::init(data.to_string());
    assert_eq!(system.part1_with_turn_number(), 18740);
    assert_eq!(System::part2_with_turn_number(data.to_string()), 1140);
}
