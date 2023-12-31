use crate::instruction::Opcode;

pub struct VM {
    /// Array that simulates hardware registers
    pub registers: [i32; 32],
    /// Program counter that keeps track of the currently executing instruction
    pc: usize,
    /// The bytecode that the virtual machine will execute
    pub program: Vec<u8>,
    /// Contains the remainder of modulo division ops
    remainder: u32,
    /// Contains the result of the last comparison operation
    equal_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;

        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
            },
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            },
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            },
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            },
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            },
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            },
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize; // Set the program counter to the value in the register
            },
            Opcode::JMPF => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc += target as usize; // Increment the program counter by the value in the register
            },
            Opcode::JMPB => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc -= target as usize; // Decrement the program counter by the value in the register
            },
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 == register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            },
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 != register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();
            },
            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 > register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();
            },
            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 < register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();
            },
            Opcode::GTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 >= register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();
            },
            Opcode::LTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 <= register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();
            },
            Opcode::JEQ => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                if self.equal_flag {
                    self.pc = target as usize;
                }
            },
            _ => {
                println!("Unknown instruction");
                return true;
            }
        }
        false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }

    pub fn add_byte(&mut self, b: u8) {
        self.program.push(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_vm() -> VM {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;
        test_vm
    }

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![0, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![1, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 15)
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![2, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], -5);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![3, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 50);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![4, 1, 0, 2];
        test_vm.registers[1] = 16;
        test_vm.run();
        assert_eq!(test_vm.registers[2], 3);
        assert_eq!(test_vm.remainder, 1);
    }

    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 4;
        test_vm.program = vec![6, 0, 0, 0, 8, 0, 0, 0, 0];
        test_vm.pc = 4;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 2);
    }

    #[test]
    fn test_opcode_eq() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_neq() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 8;
        test_vm.registers[1] = 10;
        test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 8;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_gt() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 8;
        test_vm.program = vec![11, 0, 1, 0, 11, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 12;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_lt() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 8;
        test_vm.registers[1] = 10;
        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 4;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_gtq() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0, 13, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_ltq() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0, 14, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_opcode_jeq() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![15, 0, 0, 0, 17, 0, 0, 0, 17, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }

}