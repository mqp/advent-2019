use std::error::Error;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::collections::VecDeque;

type Word = i64;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum VMState {
    Ready,
    Halted,
    Waiting
}

#[derive(Debug)]
struct VM {
    pc: usize,
    rb: Word,
    mem: Vec<Word>,
    state: VMState,
    inputs: VecDeque<Word>,
    outputs: VecDeque<Word>
}

fn tokenize(input: &str) -> Result<Vec<Word>, ParseIntError> {
    input.split(',').map(|s| s.parse()).collect()
}

impl VM {

    pub fn create(program: &str, initial_inputs: &[Word], mem_size_words: usize) -> Result<Self, ParseIntError> {
        let contents = tokenize(program.trim_end())?;
        let mut mem = vec![0; mem_size_words];
        mem[0..contents.len()].copy_from_slice(&contents);
        let mut inputs = VecDeque::new();
        inputs.extend(initial_inputs);
        Ok(Self { mem, inputs, outputs: VecDeque::new(), pc: 0, rb: 0, state: VMState::Ready })
    }

    fn mode_for(i: usize, mode: usize) -> usize {
        mode % 10_usize.pow((i+1) as u32) / 10_usize.pow(i as u32)
    }

    fn w_parm(&self, i: usize, modes: usize) -> usize {
        let p = self.mem[self.pc + i+1];
        match VM::mode_for(i, modes) {
            0 => p as usize,
            1 => panic!("Immediate mode for write parameters is invalid."),
            2 => (p + self.rb) as usize,
            _ => panic!(format!("Fishy mode: {}", modes))
        }
    }

    fn r_parm(&self, i: usize, modes: usize) -> Word {
        let p = self.mem[self.pc + i+1];
        match VM::mode_for(i, modes) {
            0 => self.mem[p as usize],
            1 => p,
            2 => self.mem[(p + self.rb) as usize],
            _ => panic!(format!("Fishy mode: {}", modes))
        }
    }

    pub fn run(self: &mut Self) {
        loop {
            let instr = self.mem[self.pc] as usize;
            let modes = instr / 100;
            let opcode = instr - (modes * 100);
            match opcode {
                1 => { // add
                    let a = self.r_parm(0, modes);
                    let b = self.r_parm(1, modes);
                    let c = self.w_parm(2, modes);
                    self.mem[c] = a + b;
                    self.pc += 4;
                }
                2 => { // mul
                    let a = self.r_parm(0, modes);
                    let b = self.r_parm(1, modes);
                    let c = self.w_parm(2, modes);
                    self.mem[c] = a * b;
                    self.pc += 4;
                }
                3 => { // input
                    if let Some(n) = self.inputs.pop_front() {
                        let dest = self.w_parm(0, modes);
                        self.mem[dest] = n;
                        self.pc += 2;
                    } else {
                        self.state = VMState::Waiting;
                        return;
                    }
                }
                4 => { // output
                    let a = self.r_parm(0, modes);
                    self.outputs.push_back(a);
                    self.pc += 2;
                }
                5 => { // jump if true
                    let a = self.r_parm(0, modes);
                    let b = self.r_parm(1, modes);
                    self.pc = if a != 0 { b as usize } else { self.pc + 3 };
                }
                6 => { // jump if false
                    let a = self.r_parm(0, modes);
                    let b = self.r_parm(1, modes);
                    self.pc = if a == 0 { b as usize } else { self.pc + 3 };
                }
                7 => { // lt
                    let a = self.r_parm(0, modes);
                    let b = self.r_parm(1, modes);
                    let c = self.w_parm(2, modes);
                    self.mem[c] = if a < b { 1 } else { 0 };
                    self.pc += 4;
                }
                8 => { // eq
                    let a = self.r_parm(0, modes);
                    let b = self.r_parm(1, modes);
                    let c = self.w_parm(2, modes);
                    self.mem[c] = if a == b { 1 } else { 0 };
                    self.pc += 4;
                }
                9 => { // relative base offset
                    let a = self.r_parm(0, modes);
                    self.rb += a;
                    self.pc += 2;
                }
                99 => {
                    self.state = VMState::Halted;
                    return;
                }
                _ => {
                    panic!(format!("Fishy opcode found: {}", opcode));
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut vm = VM::create(&input, &[2], 10000)?;
    vm.run();
    println!("Outputs: {:?}", vm.outputs);
    Ok(())
}
