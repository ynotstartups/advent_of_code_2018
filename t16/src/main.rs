#![allow(non_snake_case, dead_code)]
use std::fs;

#[derive(Debug, PartialEq)]
struct Registers {
    values: [i32; 4],
}

impl Registers {
    pub fn new(values: [i32; 4]) -> Registers {
        Registers { values }
    }

    pub fn new_from_vec(values: Vec<i32>) -> Registers {
        assert_eq!(values.len(), 4);
        let values = [values[0], values[1], values[2], values[3]];
        Registers { values }
    }

    fn get(&self, register: i32) -> i32 {
        assert!(register <= 3);
        self.values[register as usize]
    }

    fn set_register_to_value(&self, register: i32, value: i32) -> Registers {
        assert!(register <= 3);
        let mut values = self.values.clone();
        values[register as usize] = value;
        Registers::new(values)
    }
}

#[derive(Debug, PartialEq)]
struct Instruction<'a> {
    instruction: [i32; 4],
    registers: &'a Registers,
}

impl<'a> Instruction<'a> {
    pub fn new(data: [i32; 4], registers: &'a Registers) -> Instruction<'a> {
        Instruction {
            instruction: data,
            registers: registers,
        }
    }

    pub fn new_from_vec(data: Vec<i32>, registers: &'a Registers) -> Instruction<'a> {
        assert_eq!(data.len(), 4);
        let data = [data[0], data[1], data[2], data[3]];
        Instruction::new(data, registers)
    }

    pub fn new_without_opcode(data: [i32; 3], registers: &'a Registers) -> Instruction {
        Instruction {
            instruction: [0, data[0], data[1], data[2]],
            registers: registers,
        }
    }

    fn get_register_A(&self) -> i32 {
        self.registers.get(self.instruction[1])
    }

    fn get_register_B(&self) -> i32 {
        self.registers.get(self.instruction[2])
    }

    fn get_value_A(&self) -> i32 {
        self.instruction[1]
    }
    fn get_value_B(&self) -> i32 {
        self.instruction[2]
    }

    fn store_register_C(&self, store_value: i32) -> Registers {
        self.registers
            .set_register_to_value(self.instruction[3], store_value)
    }

    fn store_register_C_bool(&self, boolean: bool) -> Registers {
        let value = if boolean { 1 } else { 0 };
        self.registers
            .set_register_to_value(self.instruction[3], value)
    }

    fn addr(&self) -> Registers {
        self.store_register_C(self.get_register_A() + self.get_register_B())
    }

    fn addi(&self) -> Registers {
        self.store_register_C(self.get_register_A() + self.get_value_B())
    }

    fn mulr(&self) -> Registers {
        self.store_register_C(self.get_register_A() * self.get_register_B())
    }

    fn muli(&self) -> Registers {
        self.store_register_C(self.get_register_A() * self.get_value_B())
    }

    fn banr(&self) -> Registers {
        self.store_register_C(self.get_register_A() & self.get_register_B())
    }

    fn bani(&self) -> Registers {
        self.store_register_C(self.get_register_A() & self.get_value_B())
    }

    fn borr(&self) -> Registers {
        self.store_register_C(self.get_register_A() | self.get_register_B())
    }

    fn bori(&self) -> Registers {
        self.store_register_C(self.get_register_A() | self.get_value_B())
    }

    fn setr(&self) -> Registers {
        self.store_register_C(self.get_register_A())
    }

    fn seti(&self) -> Registers {
        self.store_register_C(self.get_value_A())
    }

    fn gtir(&self) -> Registers {
        self.store_register_C_bool(self.get_value_A() > self.get_register_B())
    }

    fn gtri(&self) -> Registers {
        self.store_register_C_bool(self.get_register_A() > self.get_value_B())
    }

    fn gtrr(&self) -> Registers {
        self.store_register_C_bool(self.get_register_A() > self.get_register_B())
    }

    fn eqir(&self) -> Registers {
        self.store_register_C_bool(self.get_value_A() == self.get_register_B())
    }

    fn eqri(&self) -> Registers {
        self.store_register_C_bool(self.get_register_A() == self.get_value_B())
    }

    fn eqrr(&self) -> Registers {
        self.store_register_C_bool(self.get_register_A() == self.get_register_B())
    }

    fn num_behaves_like_three_opcodes(&self, output: &Registers) -> i32 {
        [
            self.addr(),
            self.addi(),
            self.mulr(),
            self.muli(),
            self.banr(),
            self.bani(),
            self.borr(),
            self.bori(),
            self.setr(),
            self.seti(),
            self.gtir(),
            self.gtri(),
            self.gtrr(),
            self.eqir(),
            self.eqri(),
            self.eqrr(),
        ]
        .iter()
        .map(|r| if r == output { 1 } else { 0 })
        .sum()
    }
}

struct System {}

impl System {
    fn part1(data: &String) -> u32 {
        let mut number_behaves_like_three = 0 as u32;
        let mut number_of_instructions = 0;

        let mut lines = data.lines();
        loop {
            let before = lines.next().unwrap();
            if before == "" {
                return number_behaves_like_three;
            }

            let instruction = lines.next().unwrap();
            let after = lines.next().unwrap();
            let blank = lines.next().unwrap();

            let before_register = System::to_register(before, 9);
            let instruction =
                Instruction::new_from_vec(System::to_i32_vec(instruction), &before_register);
            let after_register = System::to_register(after, 9);

            // println!("{:?}", before_register);
            println!(
                "{:?} {:?} {:?} {}",
                before_register, instruction, after_register, blank
            );

            if instruction.num_behaves_like_three_opcodes(&after_register) >= 3 {
                number_behaves_like_three += 1;
            }

            number_of_instructions += 1;
            println!("number of instruction {}", number_of_instructions);
        }
    }

    fn to_register(string: &str, start_index: usize) -> Registers {
        let mut register_data = Vec::new();
        for b in string.chars().skip(start_index).take(10) {
            if let Some(digit) = b.to_digit(10) {
                register_data.push(digit as i32);
            }
        }
        Registers::new_from_vec(register_data)
    }

    fn to_i32_vec(string: &str) -> Vec<i32> {
        string
            .split(' ')
            .map(|c| c.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    }
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");
    println!(
        "number of instruction behave like three opcodes {}",
        System::part1(&data)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_new_test() {
        assert_eq!(
            Registers::new([1, 2, 3, 4]),
            Registers {
                values: [1, 2, 3, 4]
            }
        );
    }

    #[test]
    fn instruction_new_test() {
        let registers = Registers::new([1, 1, 0, 0]);
        assert_eq!(
            Instruction::new([0, 0, 0, 0], &registers),
            Instruction {
                instruction: [0, 0, 0, 0],
                registers: &registers,
            }
        );
    }

    #[test]
    fn instruction_new_without_opcode_test() {
        let registers = Registers::new([1, 1, 0, 0]);
        assert_eq!(
            Instruction::new_without_opcode([0, 0, 0], &registers),
            Instruction {
                instruction: [0, 0, 0, 0],
                registers: &registers,
            }
        );
    }

    #[test]
    fn addr_test() {
        let registers = Registers::new([1, 1, 0, 0]);
        let instruction = Instruction::new_without_opcode([0, 1, 2], &registers);
        assert_eq!(instruction.addr(), Registers::new([1, 1, 2, 0]));
    }

    #[test]
    fn addi_test() {
        let registers = Registers::new([1, 0, 0, 0]);
        let instruction = Instruction::new_without_opcode([0, 2, 2], &registers);
        assert_eq!(instruction.addi(), Registers::new([1, 0, 3, 0]));
    }

    #[test]
    fn mulr_test() {
        let registers = Registers::new([1, 2, 0, 0]);
        let instruction = Instruction::new_without_opcode([0, 1, 2], &registers);
        assert_eq!(instruction.mulr(), Registers::new([1, 2, 2, 0]));
    }

    #[test]
    fn muli_test() {
        let registers = Registers::new([1, 0, 0, 0]);
        let instruction = Instruction::new_without_opcode([0, 2, 2], &registers);
        assert_eq!(instruction.muli(), Registers::new([1, 0, 2, 0]));
    }

    #[test]
    fn banr() {
        let registers = Registers::new([1, 1, 0, 0]);
        let instruction = Instruction::new_without_opcode([0, 1, 2], &registers);
        assert_eq!(instruction.banr(), Registers::new([1, 1, 1, 0]));

        let registers = Registers::new([1, 0, 0, 0]);
        let instruction = Instruction::new_without_opcode([0, 1, 2], &registers);
        assert_eq!(instruction.banr(), Registers::new([1, 0, 0, 0]));
    }

    #[test]
    fn bani() {
        let registers = Registers::new([1, 0, 0, 0]);
        let instruction = Instruction::new_without_opcode([0, 1, 2], &registers);
        assert_eq!(instruction.bani(), Registers::new([1, 0, 1, 0]));

        let registers = Registers::new([1, 0, 0, 0]);
        let instruction = Instruction::new_without_opcode([0, 0, 2], &registers);
        assert_eq!(instruction.bani(), Registers::new([1, 0, 0, 0]));
    }

    #[test]
    fn borr() {
        let registers = Registers::new([1, 1, 0, 0]);
        let instruction = Instruction::new_without_opcode([0, 1, 2], &registers);
        assert_eq!(instruction.borr(), Registers::new([1, 1, 1, 0]));

        let registers = Registers::new([1, 0, 1, 0]);
        let instruction = Instruction::new_without_opcode([0, 1, 2], &registers);
        assert_eq!(instruction.borr(), Registers::new([1, 0, 1, 0]));

        let registers = Registers::new([0, 0, 1, 0]);
        let instruction = Instruction::new_without_opcode([0, 1, 2], &registers);
        assert_eq!(instruction.borr(), Registers::new([0, 0, 0, 0]));
    }

    #[test]
    fn bori() {
        let registers = Registers::new([0, 1, 0, 0]);
        let instruction = Instruction::new_without_opcode([1, 1, 2], &registers);
        assert_eq!(instruction.bori(), Registers::new([0, 1, 1, 0]));

        let registers = Registers::new([0, 1, 0, 0]);
        let instruction = Instruction::new_without_opcode([1, 0, 2], &registers);
        assert_eq!(instruction.bori(), Registers::new([0, 1, 1, 0]));

        let registers = Registers::new([0, 0, 1, 0]);
        let instruction = Instruction::new_without_opcode([1, 0, 2], &registers);
        assert_eq!(instruction.bori(), Registers::new([0, 0, 0, 0]));
    }

    #[test]
    fn setr() {
        let registers = Registers::new([2, 0, 0, 0]);
        let instruction = Instruction::new_without_opcode([0, 0, 2], &registers);
        assert_eq!(instruction.setr(), Registers::new([2, 0, 2, 0]));
    }

    #[test]
    fn seti() {
        let registers = Registers::new([0, 0, 0, 0]);
        let instruction = Instruction::new_without_opcode([1, 0, 2], &registers);
        assert_eq!(instruction.seti(), Registers::new([0, 0, 1, 0]));
    }

    #[test]
    fn gtir() {
        let registers = Registers::new([1, 0, 0, 0]);
        let instruction = Instruction::new_without_opcode([2, 0, 2], &registers);
        assert_eq!(instruction.gtir(), Registers::new([1, 0, 1, 0]));

        let registers = Registers::new([1, 0, 1, 0]);
        let instruction = Instruction::new_without_opcode([0, 0, 2], &registers);
        assert_eq!(instruction.gtir(), Registers::new([1, 0, 0, 0]));
    }

    #[test]
    fn gtri() {
        let registers = Registers::new([1, 0, 0, 0]);
        let instruction = Instruction::new_without_opcode([0, 0, 2], &registers);
        assert_eq!(instruction.gtri(), Registers::new([1, 0, 1, 0]));

        let registers = Registers::new([1, 0, 1, 0]);
        let instruction = Instruction::new_without_opcode([0, 2, 2], &registers);
        assert_eq!(instruction.gtri(), Registers::new([1, 0, 0, 0]));
    }

    #[test]
    fn gtrr() {
        let registers = Registers::new([1, 0, 0, 0]);
        let instruction = Instruction::new_without_opcode([0, 1, 2], &registers);
        assert_eq!(instruction.gtrr(), Registers::new([1, 0, 1, 0]));

        let registers = Registers::new([0, 1, 1, 0]);
        let instruction = Instruction::new_without_opcode([0, 1, 2], &registers);
        assert_eq!(instruction.gtrr(), Registers::new([0, 1, 0, 0]));
    }

    #[test]
    fn eqir() {
        let registers = Registers::new([1, 2, 0, 4]);
        let instruction = Instruction::new_without_opcode([1, 0, 2], &registers);
        assert_eq!(instruction.eqir(), Registers::new([1, 2, 1, 4]));

        let registers = Registers::new([1, 2, 1, 4]);
        let instruction = Instruction::new_without_opcode([2, 0, 2], &registers);
        assert_eq!(instruction.eqir(), Registers::new([1, 2, 0, 4]));
    }

    #[test]
    fn eqri() {
        let registers = Registers::new([1, 0, 0, 0]);
        let instruction = Instruction::new_without_opcode([0, 1, 2], &registers);
        assert_eq!(instruction.eqri(), Registers::new([1, 0, 1, 0]));

        let registers = Registers::new([1, 0, 1, 0]);
        let instruction = Instruction::new_without_opcode([0, 0, 2], &registers);
        assert_eq!(instruction.eqri(), Registers::new([1, 0, 0, 0]));
    }

    #[test]
    fn eqrr() {
        let registers = Registers::new([1, 1, 0, 0]);
        let instruction = Instruction::new_without_opcode([0, 1, 2], &registers);
        assert_eq!(instruction.eqrr(), Registers::new([1, 1, 1, 0]));

        let registers = Registers::new([1, 0, 1, 0]);
        let instruction = Instruction::new_without_opcode([0, 1, 2], &registers);
        assert_eq!(instruction.eqrr(), Registers::new([1, 0, 0, 0]));
    }

    #[test]
    fn num_behaves_like_three_opcodes() {
        let registers = Registers::new([3, 2, 1, 1]);
        let instruction = Instruction::new_without_opcode([2, 1, 2], &registers);
        assert_eq!(
            instruction.num_behaves_like_three_opcodes(&Registers::new([3, 2, 2, 1])),
            3
        );
    }
}
