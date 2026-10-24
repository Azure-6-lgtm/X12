pub(crate) mod X12Protocol;

use X12Protocol::{Instruction, X12Cpu};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("--ver") => {
            println!("X12 version 0.1.0 open beta");
            return;
        }
        Some("--help") => {
            println!("W.I.P");
            return;
        }
        _ => {}
    };

    let program = vec![
        Instruction::Add {
            source1: 0,
            source2: 1,
            destination: 2,
        },
        Instruction::Store {
            address: 0,
            source: 2,
        },
        Instruction::Load {
            address: 0,
            destination: 3,
        },
        Instruction::Compare {
			source1: 0,
			source2: 1,
        },
    ];
    let mut cpu = X12Cpu::new();
    cpu.write_register(5, 0);
    cpu.write_register(3, 1);
    cpu.run(&program, 187);
    let flags = cpu.get_flags();
    let R3 = match cpu.read_register(3) {
        Some(Value) => Value,
        None => 0,
    };
    let M0 = match cpu.read_ram(0) {
        Some(Value) => Value,
        None => 0,
    };

    println!("The final PC is {}", cpu.get_pc());
    println!("The value of ram at address 0 is {:#?}", M0);
    println!("The Value of regsiter 3 is {:#?} ", R3);
    println!("The flags of the cpu: {flags:#?}");
}
