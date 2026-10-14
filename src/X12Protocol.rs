#[derive(Debug, Clone)]
pub struct CpuFlags {
    pub zero: bool,
    pub greater: bool,
    pub lesser: bool,
}

#[derive(Debug, Clone)]
pub struct X12Cpu {
    registers: [u16; 8],
    halted: bool,
    pc: usize,
    flags: CpuFlags,
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
    Compare {
        source1: usize,
        source2: usize,
    },
    Jz {
        target: usize,
    },
    Jnz {
        target: usize,
    },
    Jg {
        target: usize,
    },
    Jl {
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
            flags: CpuFlags {
                zero: false,
                greater: false,
                lesser: false,
            },
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

            Instruction::Compare { source1, source2 } => {
                self.compare_registers(source1, source2);
            }

            Instruction::Jmp { target } => {
                self.pc = target;
            }

            Instruction::Jz { target } => {
                if self.flags.zero {
                    self.pc = target;
                }
            }

            Instruction::Jnz {target } => {
                if !self.flags.zero {
                    self.pc = target;
                }
            },

            Instruction::Jg  {target} => {
                if self.flags.greater {
                    self.pc = target;
                }
            },

            Instruction::Jl {target} => {
                if self.flags.lesser {
                    self.pc = target;
                }
            },

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

            let jumped = match &current_instruction {
                Instruction::Jmp { .. } => true,
                Instruction::Jz { .. } => self.flags.zero,
                Instruction::Jnz { .. } => !self.flags.zero,
                Instruction::Jg { .. } => self.flags.greater,
                Instruction::Jl { .. } => self.flags.lesser,
                _ => false,
            };

            self.execute(current_instruction);

            if !jumped {
                self.increment_pc();
            }

            if is_halt {
                println!("X12 CPU has been halted, Exiting");
                break;
            }
        }
    }

    pub fn compare_registers(&mut self, source1: usize, source2: usize) {
        let a = self.read_register(source1);
        let b = self.read_register(source2);

        self.flags.zero = a == b;
        self.flags.greater = a > b;
        self.flags.lesser = a < b;
    }

    pub fn fetch<'a>(&self, program: &'a [Instruction]) -> Option<Instruction> {
        program.get(self.pc).cloned()
    }

    pub fn restart(&mut self) {
        println!("Restarting X12 protocol");
        self.halted = false;
        self.pc = 0;
    }

    pub fn get_flags(&self) -> &CpuFlags {
        &self.flags
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
