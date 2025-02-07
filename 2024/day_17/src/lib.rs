const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

#[derive(Debug)]
pub struct Computer {
    pub registers: [u64; 3], // Registers A, B, and C
    pub program: Vec<u8>,    // List of 3bit numbers
    pub instruction_pointer: usize,
    pub output: Vec<u8>,
}

impl Computer {
    pub fn new(registers: [u64; 3], program: &[u8]) -> Self {
        Self {
            registers,
            program: program.to_vec(),
            instruction_pointer: 0,
            output: Vec::new(),
        }
    }

    pub fn combo(&self, operand: u64) -> u64 {
        match operand {
            //Combo operands 0 through 3 represent literal values 0 through 3.
            0..=3 => operand,
            //Combo operand 4 represents the value of register A.
            4 => self.registers[A],
            //Combo operand 5 represents the value of register B.
            5 => self.registers[B],
            //Combo operand 6 represents the value of register C.
            6 => self.registers[C],
            //Combo operand 7 is reserved and will not appear in valid programs
            _ => unreachable!("Operand is not recognized: {}", operand),
        }
    }

    /// The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
    /// The denominator is found by raising 2 to the power of the instruction's combo operand.
    /// (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
    /// The result of the division operation is truncated to an integer and then written to the A register.
    pub fn adv(&mut self, operand: u64) {
        let operand = self.combo(operand);
        self.registers[A] >>= operand;
    }

    /// The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the
    /// instruction's literal operand, then stores the result in register B.
    pub fn bxl(&mut self, operand: u64) {
        self.registers[B] ^= operand;
    }

    /// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
    /// (thereby keeping only its lowest 3 bits), then writes that value to the B register.
    pub fn bst(&mut self, operand: u64) {
        let operand = self.combo(operand);
        self.registers[B] = operand % 8;
    }

    /// The jnz instruction (opcode 3) does nothing if the A register is 0.
    /// However, if the A register is not zero, it jumps by setting the instruction pointer
    /// to the value of its literal operand; if this instruction jumps, the instruction pointer
    /// is not increased by 2 after this instruction.
    pub fn jnz(&mut self, operand: u64) {
        self.instruction_pointer = operand as usize;
    }

    /// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
    /// then stores the result in register B. (For legacy reasons, this instruction reads an
    /// operand but ignores it.)
    pub fn bxc(&mut self, _operand: u64) {
        self.registers[B] ^= self.registers[C];
    }

    /// The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
    /// then outputs that value. (If a program outputs multiple values, they are separated by commas.)
    pub fn out(&mut self, operand: u64) {
        let operand = self.combo(operand);
        self.output.push((operand % 8) as u8);
    }

    /// The bdv instruction (opcode 6) works exactly like the adv instruction except that
    /// the result is stored in the B register. (The numerator is still read from the A
    /// register.)
    pub fn bdv(&mut self, operand: u64) {
        let operand = self.combo(operand);
        self.registers[B] = self.registers[A] >> operand;
    }

    /// The cdv instruction (opcode 7) works exactly like the adv instruction
    /// except that the result is stored in the C register. (The numerator
    /// is still read from the A register.)
    pub fn cdv(&mut self, operand: u64) {
        let operand = self.combo(operand);
        self.registers[C] = self.registers[A] >> operand;
    }

    pub fn run(&mut self) -> Vec<u8> {
        while self.instruction_pointer < self.program.len() {
            let opcode = self.program[self.instruction_pointer];
            let operand = self.program[self.instruction_pointer + 1] as u64;
            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => {
                    if self.registers[A] > 0 {
                        self.jnz(operand);
                        continue;
                    }
                }
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => unreachable!("Unrecognized instruction"),
            }
            self.instruction_pointer += 2;
        }
        self.output.clone()
    }
}

pub static INPUT: &str = r#"
Register A: 64854237
Register B: 0
Register C: 0

Program: 2,4,1,1,7,5,1,5,4,0,5,5,0,3,3,0
"#;
