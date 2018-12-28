use crate::position::Grid;
use crate::position::Position;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Clone)]
pub struct Unit {
    pub position: Position,
    u_type: Type,
    pub attack_power: u32,
    pub hp: i32,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Clone)]
pub enum Type {
    Empty,
    Wall,
    Cavern,
    Goblin,
    Elf,
}

impl Unit {
    pub fn new_temp(position: Position) -> Unit {
        Unit {
            position,
            u_type: Type::Empty,
            attack_power: 0,
            hp: 0,
        }
    }

    pub fn new_wall(position: Position) -> Unit {
        Unit {
            position,
            u_type: Type::Wall,
            attack_power: 0,
            hp: 0,
        }
    }

    pub fn new_cavern(position: Position) -> Unit {
        Unit {
            position,
            u_type: Type::Cavern,
            attack_power: 0,
            hp: 0,
        }
    }

    pub fn new_globin(position: Position) -> Unit {
        Unit {
            position,
            u_type: Type::Goblin,
            attack_power: 3,
            hp: 200,
        }
    }

    pub fn new_elf(position: Position) -> Unit {
        Unit {
            position,
            u_type: Type::Elf,
            attack_power: 3,
            hp: 200,
        }
    }

    pub fn new_elf_with_power(position: Position, attack_power: u32) -> Unit {
        Unit {
            position,
            u_type: Type::Elf,
            attack_power,
            hp: 200,
        }
    }

    pub fn attack(&self, other: &mut Unit) {
        if !self.is_enemy_to(other) {
            panic!("attacking strange unit {:?}", other);
        }

        if !self.is_adjacent_to(&vec![other]) {
            panic!("unit is not adjacent to {:?}", other);
        }

        other.hp -= self.attack_power as i32;
    }

    pub fn get_hit(&mut self, attack_power: u32) {
        self.hp -= attack_power as i32;
    }

    pub fn get_type(&self) -> &Type {
        &self.u_type
    }

    pub fn is_enemy_to(&self, other: &Unit) -> bool {
        if self.is_goblin() && other.is_elf() {
            true
        } else if self.is_elf() && other.is_goblin() {
            true
        } else {
            false
        }
    }

    pub fn is_temp(&self) -> bool {
        if let Type::Empty = self.u_type {
            true
        } else {
            false
        }
    }

    pub fn is_creature(&self) -> bool {
        match self.u_type {
            Type::Goblin | Type::Elf => true,
            _ => false,
        }
    }

    pub fn is_goblin(&self) -> bool {
        if self.is_creature() {
            if let Type::Goblin = self.u_type {
                return true;
            }
        }
        false
    }

    pub fn is_elf(&self) -> bool {
        if self.is_creature() {
            if let Type::Elf = self.u_type {
                return true;
            }
        }
        false
    }

    pub fn can_be_stand_on(&self) -> bool {
        if let Type::Cavern = self.u_type {
            true
        } else {
            false
        }
    }

    pub fn adjacent(&self) -> Option<Vec<Position>> {
        let positions = self.position.adjacent();
        if positions.len() > 0 {
            Some(positions)
        } else {
            None
        }
    }

    pub fn is_adjacent_to(&self, units: &Vec<&Unit>) -> bool {
        self.position
            .is_adjacent_to(units.iter().map(|x| x.position.clone()).collect::<Vec<_>>())
    }

    fn distance_to<T>(&self, unit: &Unit, map: &T) -> Option<u32>
    where
        T: Grid,
    {
        self.position.distance_to(&unit.position, map)
    }

    pub fn closest_reachable_units<'a, T>(
        &'a self,
        units: &Vec<&'a Unit>,
        map: &'a T,
    ) -> Option<&Unit>
    where
        T: Grid,
    {
        let mut reachable_units = Vec::new();
        for unit in units {
            if let Some(_) = self.distance_to(unit, map) {
                reachable_units.push(unit)
            } else {
                // println!(
                //     "cannot reach unit {:?} unit {:?}",
                //     unit,
                //     self.distance_to(unit, map)
                // );
            }
        }

        // sort by reading order
        reachable_units.sort_by_key(|u| &u.position);

        reachable_units.sort_by_key(|u| u.distance_to(self, map));

        if reachable_units.len() > 0 {
            Some(reachable_units[0])
        } else {
            None
        }
    }

    pub fn is_dead(&self) -> bool {
        if !self.is_creature() {
            panic!("You should not check a non creature is dead");
        }

        if self.hp <= 0 {
            true
        } else {
            false
        }
    }

    pub fn print_status(&self) {
        println!("{:?} - {:?} - {}", self.position, self.u_type, self.hp);
    }
}

#[test]
fn attack() {
    let elf = Unit::new_elf(Position::new(0, 0));
    let mut globin = Unit::new_globin(Position::new(0, 1));
    elf.attack(&mut globin);
    assert_eq!(globin.hp, 200 - 3);
}

#[test]
fn is_dead() {
    let elf = Unit {
        position: Position::new(0, 0),
        u_type: Type::Elf,
        attack_power: 0,
        hp: -1,
    };

    assert!(elf.is_dead());

    let elf = Unit {
        position: Position::new(0, 0),
        u_type: Type::Elf,
        attack_power: 0,
        hp: 0,
    };

    assert!(elf.is_dead());

    let elf = Unit {
        position: Position::new(0, 1),
        u_type: Type::Elf,
        attack_power: 0,
        hp: 1,
    };

    assert!(!elf.is_dead());
}

#[test]
#[should_panic]
fn attack_same_race() {
    let elf_0 = Unit::new_elf(Position::new(0, 0));
    let mut elf_1 = Unit::new_elf(Position::new(0, 1));
    elf_0.attack(&mut elf_1);
}

#[test]
#[should_panic]
fn attack_not_adjacent() {
    let elf = Unit::new_elf(Position::new(0, 0));
    let mut globin = Unit::new_elf(Position::new(1, 1));
    elf.attack(&mut globin);
}

#[test]
fn get_hit() {
    let attack_power: u32 = 5;
    let mut elf_0 = Unit::new_elf(Position::new(0, 0));
    elf_0.get_hit(attack_power);
    assert_eq!(elf_0.hp, 200 - attack_power as i32);
}
