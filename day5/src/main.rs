use std::error::Error;
use std::io::{self, Read};
use std::num::ParseIntError;

struct ProgramCtx {
    mem: Vec<i32>,
    input: Box<dyn (FnMut() -> i32)>,
    output: Box<dyn (FnMut(i32) -> ())>
}

fn val(mem: &[i32], data: i32, mode: usize) -> i32 {
    if mode == 1 { data } else { mem[data as usize] }
}

fn exec(ctx: &mut ProgramCtx, pc: usize, opcode: usize, modes: [usize; 2]) -> usize {
    let ProgramCtx { ref mut mem, ref mut input, ref mut output, .. } = ctx;
    match opcode {
        1 => { // add
            let noun = mem[pc+1];
            let verb = mem[pc+2];
            let dest = mem[pc+3] as usize;
            mem[dest] = val(mem, noun, modes[0]) + val(mem, verb, modes[1]);
            pc + 4
        }
        2 => { // mul
            let noun = mem[pc+1];
            let verb = mem[pc+2];
            let dest = mem[pc+3] as usize;
            mem[dest] = val(mem, noun, modes[0]) * val(mem, verb, modes[1]);
            pc + 4
        }
        3 => { // input
            let dest = mem[pc+1] as usize;
            mem[dest] = (input)();
            pc + 2
        }
        4 => { // output
            let noun = mem[pc+1];
            (output)(val(mem, noun, modes[0]));
            pc + 2
        }
        5 => { // jump if true
            let noun = mem[pc+1];
            let verb = mem[pc+2];
            if val(mem, noun, modes[0]) != 0 { val(mem, verb, modes[1]) as usize } else { pc + 3 }
        }
        6 => { // jump if false
            let noun = mem[pc+1];
            let verb = mem[pc+2];
            if val(mem, noun, modes[0]) == 0 { val(mem, verb, modes[1]) as usize } else { pc + 3 }
        }
        7 => { // lt
            let noun = mem[pc+1];
            let verb = mem[pc+2];
            let dest = mem[pc+3] as usize;
            mem[dest] = if val(mem, noun, modes[0]) < val(mem, verb, modes[1]) { 1 } else { 0 };
            pc + 4
        }
        8 => { // eq
            let noun = mem[pc+1];
            let verb = mem[pc+2];
            let dest = mem[pc+3] as usize;
            mem[dest] = if val(mem, noun, modes[0]) == val(mem, verb, modes[1]) { 1 } else { 0 };
            pc + 4
        }
        _ => {
            panic!(format!("Fishy opcode found: {}", opcode));
        }
    }
}

fn parse_program(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.split(',').map(|s| s.parse()).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut ctx = ProgramCtx {
        mem: parse_program(input.trim_end())?,
        input: Box::new(|| { 1 }),
        output: Box::new(|x| { println!("{}", x) })
    };
    // println!("Memory: {:?}", ctx.mem);
    let mut pc = 0;
    loop {
        let instr = ctx.mem[pc] as usize;
        let modenum = instr / 100;
        let opcode = instr - (modenum * 100);
        let modes = [
            (modenum % 10),
            (modenum % 100) / 10
        ];
        match opcode {
            99 => { break; } // halt
            op => pc = exec(&mut ctx, pc, op, modes)
        }
    }
    Ok(())
}
