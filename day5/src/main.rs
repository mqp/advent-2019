use std::error::Error;
use std::io::{self, Read};
use std::num::ParseIntError;

fn parse_program(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.split(',').map(|s| s.parse()).collect()
}

fn val(memory: &[i32], data: i32, mode: usize) -> i32 {
    if mode == 1 { data } else { memory[data as usize] }
}

fn exec(memory: &mut [i32], input: i32, opcode: usize, modes: usize, pc: &mut usize) {
    let m0 = modes % 10;
    let m1 = (modes % 100) / 10;
    return match opcode {
        1 => { // add
            let noun = memory[*pc+1];
            let verb = memory[*pc+2];
            let dest = memory[*pc+3] as usize;
            memory[dest] = val(memory, noun, m0) + val(memory, verb, m1);
            *pc += 4;
        }
        2 => { // mul
            let noun = memory[*pc+1];
            let verb = memory[*pc+2];
            let dest = memory[*pc+3] as usize;
            memory[dest] = val(memory, noun, m0) * val(memory, verb, m1);
            *pc += 4;
        }
        3 => { // input
            let dest = memory[*pc+1] as usize;
            memory[dest] = input;
            *pc += 2;
        }
        4 => { // output
            let noun = memory[*pc+1];
            println!("{}", val(memory, noun, m0));
            *pc += 2;
        }
        5 => { // jump if true
            let noun = memory[*pc+1];
            let verb = memory[*pc+2];
            if val(memory, noun, m0) != 0 {
                *pc = val(memory, verb, m1) as usize;
            } else {
                *pc += 3;
            }
        }
        6 => { // jump if false
            let noun = memory[*pc+1];
            let verb = memory[*pc+2];
            if val(memory, noun, m0) == 0 {
                *pc = val(memory, verb, m1) as usize;
            } else {
                *pc += 3;
            }
        }
        7 => { // lt
            let noun = memory[*pc+1];
            let verb = memory[*pc+2];
            let dest = memory[*pc+3] as usize;
            memory[dest] = if val(memory, noun, m0) < val(memory, verb, m1) { 1 } else { 0 };
            *pc += 4;
        }
        8 => { // eq
            let noun = memory[*pc+1];
            let verb = memory[*pc+2];
            let dest = memory[*pc+3] as usize;
            memory[dest] = if val(memory, noun, m0) == val(memory, verb, m1) { 1 } else { 0 };
            *pc += 4;
        }
        _ => {
            panic!(format!("Fishy opcode found: {}", opcode));
        }
    }
}

fn run(memory: &mut [i32], input: i32) {
    println!("Memory: {:?}", memory);
    let mut pc = 0;
    loop {
        let instr = memory[pc] as usize;
        let modenum = instr / 100;
        let opcode = instr - (modenum * 100);
        match opcode {
            99 => { return; } // halt
            op => exec(memory, input, op, modenum, &mut pc)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut memory = parse_program(input.trim_end())?;
    run(&mut memory, 5);
    Ok(())
}
