pub(crate) mod X12Protocol;

use X12Protocol::{Instruction, X12Cpu};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "--ver" => {
            println!("X12 version 0.1.0 open beta");
            return;
        },
        "--help" => {
            println!("W.I.P");
            return;
        },
        _ => {},
    };

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
