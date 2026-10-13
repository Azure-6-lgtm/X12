pub(crate) mod X12Protocol;

use X12Protocol::{Instruction, X12Cpu};

fn main() {
    let program = [
        Instruction::Compare {
            source1: 0,
            source2: 1,
        }, // PC 0
        Instruction::Jz { target: 3 }, // PC 1: jump to 3
        Instruction::Add {
            source1: 0,
            source2: 1,
            destination: 2,
        }, // PC 2: should be skipped
        Instruction::Halt,
    ];

    let mut cpu = X12Cpu::new();
    cpu.write_register(5, 0);
    cpu.write_register(3, 1);

    cpu.run(&program);

    println!("R2: {}", cpu.read_register(2));
    println!("Flags: {:?}", cpu.get_flags());
    println!("PC: {}", cpu.get_pc());
}
