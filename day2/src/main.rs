use std::error::Error;
use std::io::{self, Read};
use std::num::ParseIntError;

fn parse(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.split(',').map(|s| s.parse()).collect()
}

fn exec(memory: &mut [usize], opcode: usize, pc: &mut usize) {
    return match opcode {
        1 => { // add
            let noun = memory[*pc+1];
            let verb = memory[*pc+2];
            let dest = memory[*pc+3];
            memory[dest] = memory[noun] + memory[verb];
            *pc += 4;
        }
        2 => { // mul
            let noun = memory[*pc+1];
            let verb = memory[*pc+2];
            let dest = memory[*pc+3];
            memory[dest] = memory[noun] * memory[verb];
            *pc += 4;
        }
        _ => {
            panic!(format!("Fishy opcode found: {}", opcode));
        }
    }
}

fn run(memory: &mut [usize]) {
    let mut pc = 0;
    loop {
        match memory[pc] {
            99 => { return; } // halt
            op => exec(memory, op, &mut pc)
        }
    }
}

fn search_until<F>(memory: &[usize], predicate: F) -> (usize, usize) where F: Fn(&[usize]) -> bool {
    let mut state = memory.to_vec();
    for noun in 0.. {
        for verb in 0..noun {
            state[1] = noun;
            state[2] = verb;
            run(&mut state);
            if predicate(&state) {
                return (noun, verb)
            } else {
                state.copy_from_slice(memory);
            }
        }
    }
    unreachable!()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let memory = parse(input.trim_end())?;
    println!("Result: {:?}", search_until(&memory, |s| s[0] == 19690720));
    Ok(())
}
