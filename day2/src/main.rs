use std::error::Error;
use std::io;
use std::io::Read;
use std::num::ParseIntError;

fn parse(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.split(',').map(|s| s.parse()).collect()
}

fn execute(memory: &mut [usize]) {
    let mut pc = 0;
    loop {
        let opcode = memory[pc];
        match opcode {
            1 => { // add
                let noun = memory[pc+1];
                let verb = memory[pc+2];
                let dest = memory[pc+3];
                memory[dest] = memory[noun] + memory[verb];
                pc += 4;
            }
            2 => { // mul
                let noun = memory[pc+1];
                let verb = memory[pc+2];
                let dest = memory[pc+3];
                memory[dest] = memory[noun] * memory[verb];
                pc += 4;
            }
            99 => { // halt
                return;
            }
            _ => {
                panic!(format!("Fishy opcode found: {}", opcode));
            }
        }
    }
}

fn search_target(memory: &[usize], target: usize) -> (usize, usize) {
    for noun in 0.. {
        for verb in 0..noun {
            let mut state = memory.to_owned();
            state[1] = noun;
            state[2] = verb;
            execute(&mut state);
            if state[0] == target {
                return (noun, verb)
            }
        }
    }
    unreachable!()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let memory = parse(input.trim_end())?;
    println!("Result: {:?}", search_target(&memory, 19690720));
    Ok(())
}
