#[derive(Debug, Clone)]
pub struct X12Cpu {
    registers: [u16; 8],
    halted: bool,
    pc: usize,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Add {
        source1: usize,
        source2: usize,
        destination: usize,
    },
    Sub {
        source1: usize,
        source2: usize,
        destination: usize,
    },
    Multiply {
        source1: usize,
        source2: usize,
        destination: usize,
    },
    Divide {
        source1: usize,
        source2: usize,
        destination: usize,
    },
    Jmp {
        target: usize,
    },
    Halt,
}

impl X12Cpu {
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
            halted: false,
            pc: 0,
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        if self.halted {
            println!("X12 Protocol is currently halted");
            return;
        }
        match instruction {
            Instruction::Add {
                source1,
                source2,
                destination,
            } => {
                let num1 = self.read_register(source1);
                let num2 = self.read_register(source2);

                let sum = num1 + num2;

                self.write_register(sum, destination);
            }

            Instruction::Sub {
                source1,
                source2,
                destination,
            } => {
                let num1 = self.read_register(source1);
                let num2 = self.read_register(source2);

                let difference = num1 - num2;

                self.write_register(difference, destination);
            }

            Instruction::Multiply {
                source1,
                source2,
                destination,
            } => {
                let num1 = self.read_register(source1);
                let num2 = self.read_register(source2);

                let product = num1 * num2;

                self.write_register(product, destination);
            }

            Instruction::Divide {
                source1,
                source2,
                destination,
            } => {
                let num1 = self.read_register(source1);
                let num2 = self.read_register(source2);

                let dividend = num1 / num2;

                self.write_register(dividend, destination);
            }

            Instruction::Jmp { target } => {
                self.pc = target;
            }

            Instruction::Halt => {
                self.halted = true;
            }
        };
    }

    pub fn run(&mut self, program: &[Instruction]) {
        loop {
            if self.halted {
                println!("X12 CPU has been halted, Exiting");
                break;
            }

            let current_instruction = match self.fetch(program) {
                Some(instruction) => instruction,
                None => {
                    println!("X12 program completed");
                    break;
                }
            };

            let is_halt = matches!(&current_instruction, Instruction::Halt);
            let is_jmp = matches!(&current_instruction, Instruction::Jmp { .. });

            self.execute(current_instruction);
            if !is_jmp {
                self.increment_pc();
            }

            if is_halt {
                println!("X12 CPU has been halted, Exiting");
                break;
            }
        }
    }

    pub fn fetch<'a>(&self, program: &'a [Instruction]) -> Option<Instruction> {
        program.get(self.pc).cloned()
    }

    pub fn restart(&mut self) {
        println!("Restarting X12 protocol");
        self.halted = false;
        self.pc = 0;
    }

    pub fn read_register(&self, index: usize) -> u16 {
        self.registers[index]
    }

    pub fn write_register(&mut self, value: u16, index: usize) {
        self.registers[index] = value;
    }

    pub fn get_pc(&self) -> u16 {
        self.pc.try_into().unwrap()
    }
    pub fn increment_pc(&mut self) {
        self.pc += 1;
    }
}
