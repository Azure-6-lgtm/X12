#[derive(Debug, Clone)]
pub struct CpuFlags {
    pub zero: bool,
    pub greater: bool,
    pub lesser: bool,
}

#[derive(Debug, Clone)]
pub struct X12Cpu {
    registers: [u16; 8],
    ram: [u16; 256],
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
    Load {
        address: usize,
        destination: usize,
    },
    Store {
        source: usize,
        address: usize,
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
            ram: [0; 256],
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
                let num1 = match self.read_register(source1) {
                    Some(value) => value,
                    None => {
                        println!("X12 Error: Invalid register");
                        return;
                    }
                };
                let num2 = match self.read_register(source2) {
                    Some(value) => value,
                    None => {
                        println!("X12 Error: Invalid register");
                        return;
                    }
                };

                let sum = num1 + num2;

                self.write_register(sum, destination);
            }

            Instruction::Sub {
                source1,
                source2,
                destination,
            } => {
                let num1 = match self.read_register(source1) {
                    Some(value) => value,
                    None => {
                        println!("X12 Error: Invalid register");
                        return;
                    }
                };
                let num2 = match self.read_register(source2) {
                    Some(value) => value,
                    None => {
                        println!("X12 Error: Invalid register");
                        return;
                    }
                };

                let difference = num1 - num2;

                self.write_register(difference, destination);
            }

            Instruction::Multiply {
                source1,
                source2,
                destination,
            } => {
                let num1 = match self.read_register(source1) {
                    Some(value) => value,
                    None => {
                        println!("X12 Error: Invalid register");
                        return;
                    }
                };
                let num2 = match self.read_register(source2) {
                    Some(value) => value,
                    None => {
                        println!("X12 Error: Invalid register");
                        return;
                    }
                };

                let product = num1 * num2;

                self.write_register(product, destination);
            }

            Instruction::Divide {
                source1,
                source2,
                destination,
            } => {
                let num1 = match self.read_register(source1) {
                    Some(value) => value,
                    None => {
                        println!("X12 Error: Invalid register");
                        return;
                    }
                };
                let num2 = match self.read_register(source2) {
                    Some(value) => value,
                    None => {
                        println!("X12 Error: Invalid register");
                        return;
                    }
                };

                let dividend = num1 / num2;

                self.write_register(dividend, destination);
            }

            Instruction::Load {
                address,
                destination,
            } => {
                if address < 256 && destination < 8 {
                    self.registers[destination] = self.ram[address];
                }
            }

            Instruction::Store { source, address } => {
                if address < 256 && source < 8 {
                    self.ram[address] = self.registers[source];
                }
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

            Instruction::Jnz { target } => {
                if !self.flags.zero {
                    self.pc = target;
                }
            }

            Instruction::Jg { target } => {
                if self.flags.greater {
                    self.pc = target;
                }
            }

            Instruction::Jl { target } => {
                if self.flags.lesser {
                    self.pc = target;
                }
            }

            Instruction::Halt => {
                self.halted = true;
            }
        };
    }

    pub fn run(&mut self, program: &[Instruction], max_steps: usize) {
        let mut steps = 0;

        loop {
            if self.halted {
                println!("X12 CPU has been halted, Exiting");
                break;
            }

            if steps >= max_steps {
                println!("X12 Aborting, Max steps reached");
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

            steps += 1;

            if is_halt {
                println!("X12 CPU has been halted, Exiting");
                break;
            }
        }
    }

    pub fn compare_registers(&mut self, source1: usize, source2: usize) {
        let num1 = match self.read_register(source1) {
			Some(Value) => {Value},
			None => {
    			println!("X12 Error: Invalid register");
    			return;
		    },
        };

        let num2 = match self.read_register(source2) {
			Some(Value) => {Value},
			None => {
				println!("X12 Error: Invalid register");
				return;
			},
        };

        self.flags.zero = num1 == num2;
        self.flags.greater = num1 > num2;
        self.flags.lesser = num1 < num2;
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

    pub fn read_register(&self, index: usize) -> Option<u16> {
        if index < 8 {
            Some(self.registers[index])
        } else {
            None
        }
    }

    pub fn read_ram(&self, index: usize) -> Option<u16> {
        if index < 256 {
            Some(self.ram[index])
        } else {
            None
        }
    }

    pub fn write_register(&mut self, value: u16, index: usize) {
        if index < 8 {
            self.registers[index] = value;
        }
    }

    pub fn get_pc(&self) -> u16 {
        self.pc.try_into().unwrap()
    }
    pub fn increment_pc(&mut self) {
        self.pc += 1;
    }
}
