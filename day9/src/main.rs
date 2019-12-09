use std::error::Error;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum VMState {
    Ready,
    Halted,
    Waiting
}

#[derive(Debug)]
struct VM {
    pc: usize,
    rb: i128,
    mem: Vec<i128>,
    state: VMState,
    inputs: VecDeque<i128>,
    outputs: VecDeque<i128>
}

fn addr(base: i128, rb: i128, mode: usize) -> usize {
    match mode {
        0 => base as usize,
        1 => panic!("Immediate mode for write parameters is invalid."),
        2 => (base + rb) as usize,
        _ => panic!(format!("Fishy mode: {}", mode))
    }
}

fn val(mem: &[i128], data: i128, rb: i128, mode: usize) -> i128 {
    match mode {
        0 => mem[addr(data, rb, mode)],
        1 => data,
        2 => mem[addr(data, rb, mode)],
        _ => panic!(format!("Fishy mode: {}", mode))
    }
}

impl VM {

    fn create(program: &[i128], mem_size_128s: usize) -> Self {
        let mut mem = vec![0; mem_size_128s];
        mem[0..program.len()].copy_from_slice(program);
        Self {
            mem,
            pc: 0,
            rb: 0,
            state: VMState::Ready,
            inputs: VecDeque::new(),
            outputs: VecDeque::new()
        }
    }

    fn run(self: &mut Self) {
        let VM { ref mut pc, ref mut rb, ref mut mem, ref mut state, ref mut inputs, ref mut outputs } = self;
        loop {
            let instr = mem[*pc] as usize;
            let modenum = instr / 100;
            let opcode = instr - (modenum * 100);
            let modes = [
                (modenum % 10),
                (modenum % 100) / 10,
                (modenum % 1000) / 100
            ];
            match opcode {
                1 => { // add
                    let noun = mem[*pc+1];
                    let verb = mem[*pc+2];
                    let dest = mem[*pc+3];
                    mem[addr(dest, *rb, modes[2])] = val(mem, noun, *rb, modes[0]) + val(mem, verb, *rb, modes[1]);
                    *pc += 4;
                }
                2 => { // mul
                    let noun = mem[*pc+1];
                    let verb = mem[*pc+2];
                    let dest = mem[*pc+3];
                    mem[addr(dest, *rb, modes[2])] = val(mem, noun, *rb, modes[0]) * val(mem, verb, *rb, modes[1]);
                    *pc += 4;
                }
                3 => { // input
                    if let Some(n) = inputs.pop_front() {
                        let dest = mem[*pc+1];
                        mem[addr(dest, *rb, modes[0])] = n;
                        *pc += 2;
                    } else {
                        *state = VMState::Waiting;
                        return;
                    }
                }
                4 => { // output
                    let noun = mem[*pc+1];
                    outputs.push_back(val(mem, noun, *rb, modes[0]));
                    *pc += 2;
                }
                5 => { // jump if true
                    let noun = mem[*pc+1];
                    let verb = mem[*pc+2];
                    *pc = if val(mem, noun, *rb, modes[0]) != 0 { val(mem, verb, *rb, modes[1]) as usize } else { *pc+3 };
                }
                6 => { // jump if false
                    let noun = mem[*pc+1];
                    let verb = mem[*pc+2];
                    *pc = if val(mem, noun, *rb, modes[0]) == 0 { val(mem, verb, *rb, modes[1]) as usize } else { *pc+3 };
                }
                7 => { // lt
                    let noun = mem[*pc+1];
                    let verb = mem[*pc+2];
                    let dest = mem[*pc+3];
                    mem[addr(dest, *rb, modes[2])] = if val(mem, noun, *rb, modes[0]) < val(mem, verb, *rb, modes[1]) { 1 } else { 0 };
                    *pc += 4;
                }
                8 => { // eq
                    let noun = mem[*pc+1];
                    let verb = mem[*pc+2];
                    let dest = mem[*pc+3];
                    mem[addr(dest, *rb, modes[2])] = if val(mem, noun, *rb, modes[0]) == val(mem, verb, *rb, modes[1]) { 1 } else { 0 };
                    *pc += 4;
                }
                9 => { // relative base offset
                    let noun = mem[*pc+1];

                    *rb += val(mem, noun, *rb, modes[0]);
                    *pc += 2;
                }
                99 => {
                    *state = VMState::Halted;
                    return;
                }
                _ => {
                    panic!(format!("Fishy opcode found: {}", opcode));
                }
            }
        }
    }
}

fn parse_program(input: &str) -> Result<Vec<i128>, ParseIntError> {
    input.split(',').map(|s| s.parse()).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let program = parse_program(input.trim_end())?;
    println!("Memory: {:?}", program);
    let mut vm = VM::create(&program, 10000);
    vm.inputs.push_back(2);
    vm.run();
    println!("Outputs: {:?}", vm.outputs);
    Ok(())
}
