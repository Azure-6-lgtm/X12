pub(crate) mod X12Protocol;

use X12Protocol::{Instruction, X12Cpu};

fn main() {
    let mut cpu = X12Cpu::new();

    cpu.write_register(10, 0);
    cpu.write_register(3, 1);

    let program = vec![
        // Index 0: R2 = 10 + 3 = 13
        Instruction::Add {
            source1: 0,
            source2: 1,
            destination: 2,
        },
        // Index 1: Skip the subtraction at index 2
        Instruction::Jmp { target: 3 },
        // Index 2: This should never execute
        Instruction::Sub {
            source1: 0,
            source2: 1,
            destination: 3,
        },
        // Index 3: R4 = 10 * 3 = 30
        Instruction::Multiply {
            source1: 0,
            source2: 1,
            destination: 4,
        },
        Instruction::Halt,
    ];

    cpu.run(&program);

    println!("R2: {}", cpu.read_register(2));
    println!("R3: {}", cpu.read_register(3));
    println!("R4: {}", cpu.read_register(4));
    println!("PC: {}", cpu.get_pc());
}
