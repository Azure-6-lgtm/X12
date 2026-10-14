pub(crate) mod X12Protocol;

use X12Protocol::{Instruction, X12Cpu};

fn main() {
    let program = vec![
        Instruction::Add {
            source1: 0,
            source2: 1,
            destination: 2,
        },
        Instruction::Compare {
            source1: 1,
            source2: 2,
        },
        Instruction::Jl {
            target: 98,
        },
        Instruction::Halt,
    ];
    let mut cpu = X12Cpu::new();
    cpu.write_register(5, 0);
    cpu.write_register(3, 1);
    cpu.run(&program);

    println!("The final PC is {}", cpu.get_pc());
}
