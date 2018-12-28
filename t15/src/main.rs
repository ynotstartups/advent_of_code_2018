#[allow(dead_code)]
mod position;
mod system;
mod unit;

use crate::system::System;
use std::fs;
// allow to sort by reading order

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");
    // let data =
    //     fs::read_to_string("./example_input_0.txt").expect("Something went wrong reading the file");
    // let mut system = System::init(data);
    // println!("part1 score {}", system.part1_with_turn_number());

    // let mut system = System::init_part2(data);
    println!(
        "part2 score {}",
        System::part2_with_turn_number(data.to_string())
    );
}
