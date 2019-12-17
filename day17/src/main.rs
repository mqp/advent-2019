use std::error::Error;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::collections::{VecDeque};

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

    fn mode_for(i: usize, mode: Word) -> Word {
        match i {
            0 => (mode % 10),
            1 => (mode % 100) / 10,
            2 => (mode % 1000) / 100,
            _ => panic!("Only three parameter modes are supported.")
        }
    }

    fn parm(&self, i: usize, modes: Word) -> usize {
        let p = self.pc + i + 1;
        match VM::mode_for(i, modes) {
            1 => p,
            0 => self.mem[p] as usize,
            2 => (self.mem[p] + self.rb) as usize,
            _ => panic!(format!("Fishy mode: {}", modes))
        }
    }

    pub fn run(self: &mut Self) {
        loop {
            let instr = self.mem[self.pc];
            let opcode = instr % 100;
            let modes = instr / 100;
            match opcode {
                1 => { // add
                    let a = self.parm(0, modes);
                    let b = self.parm(1, modes);
                    let c = self.parm(2, modes);
                    self.mem[c] = self.mem[a] + self.mem[b];
                    self.pc += 4;
                }
                2 => { // mul
                    let a = self.parm(0, modes);
                    let b = self.parm(1, modes);
                    let c = self.parm(2, modes);
                    self.mem[c] = self.mem[a] * self.mem[b];
                    self.pc += 4;
                }
                3 => { // input
                    if let Some(n) = self.inputs.pop_front() {
                        let dest = self.parm(0, modes);
                        self.mem[dest] = n;
                        self.pc += 2;
                    } else {
                        self.state = VMState::Waiting;
                        return;
                    }
                }
                4 => { // output
                    let a = self.parm(0, modes);
                    self.outputs.push_back(self.mem[a]);
                    self.pc += 2;
                }
                5 => { // jump if true
                    let a = self.parm(0, modes);
                    let b = self.parm(1, modes);
                    self.pc = if self.mem[a] != 0 { self.mem[b] as usize } else { self.pc + 3 };
                }
                6 => { // jump if false
                    let a = self.parm(0, modes);
                    let b = self.parm(1, modes);
                    self.pc = if self.mem[a] == 0 { self.mem[b] as usize } else { self.pc + 3 };
                }
                7 => { // lt
                    let a = self.parm(0, modes);
                    let b = self.parm(1, modes);
                    let c = self.parm(2, modes);
                    self.mem[c] = if self.mem[a] < self.mem[b] { 1 } else { 0 };
                    self.pc += 4;
                }
                8 => { // eq
                    let a = self.parm(0, modes);
                    let b = self.parm(1, modes);
                    let c = self.parm(2, modes);
                    self.mem[c] = if self.mem[a] == self.mem[b] { 1 } else { 0 };
                    self.pc += 4;
                }
                9 => { // relative base offset
                    let a = self.parm(0, modes);
                    self.rb += self.mem[a];
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

const MAIN_FUNCTION: &'static [u8] = b"A,C,A,B,A,B,C,B,B,C";
const SUBROUTINE_A: &'static [u8] = b"L,4,L,4,L,10,R,4";
const SUBROUTINE_B: &'static [u8] = b"R,4,L,10,R,10";
const SUBROUTINE_C: &'static [u8] = b"R,4,L,4,L,4,R,8,R,10";

fn provide_ascii(vm: &mut VM, s: &[u8]) {
    let mut words: Vec<_> = s.iter().map(|&c| c as Word).collect();
    words.push('\n' as Word);
    vm.inputs.extend(&words);
}

fn get_printable_output(vm: &mut VM) -> Result<String, Box<dyn Error>> {
    Ok(String::from_utf8(vm.outputs.drain(..).map(|c| c as u8).collect::<Vec<_>>())?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut vm = VM::create(&input, &[], 10000)?;
    vm.mem[0] = 2;
    vm.run();
    provide_ascii(&mut vm, MAIN_FUNCTION);
    provide_ascii(&mut vm, SUBROUTINE_A);
    provide_ascii(&mut vm, SUBROUTINE_B);
    provide_ascii(&mut vm, SUBROUTINE_C);
    provide_ascii(&mut vm, b"n");
    vm.run();
    let output = vm.outputs.pop_back();
    println!("{}", get_printable_output(&mut vm)?);
    println!("{:?}", output);
    Ok(())
}
