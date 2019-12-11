use std::error::Error;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::collections::{HashMap, VecDeque};

type Word = i64;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum PaintColor {
    White,
    Black
}

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

    fn mode_for(i: u32, mode: u32) -> u32 {
        mode % 10_u32.pow(i + 1) / 10_u32.pow(i)
    }

    fn w_parm(&self, i: u32, modes: u32) -> usize {
        let p = self.mem[self.pc + (i as usize) + 1];
        match VM::mode_for(i, modes) {
            0 => p as usize,
            1 => panic!("Immediate mode for write parameters is invalid."),
            2 => (p + self.rb) as usize,
            _ => panic!(format!("Fishy mode: {}", modes))
        }
    }

    fn r_parm(&self, i: u32, modes: u32) -> Word {
        let p = self.mem[self.pc + (i as usize) + 1];
        match VM::mode_for(i, modes) {
            0 => self.mem[p as usize],
            1 => p,
            2 => self.mem[(p + self.rb) as usize],
            _ => panic!(format!("Fishy mode: {}", modes))
        }
    }

    pub fn run(self: &mut Self) {
        loop {
            let instr = self.mem[self.pc] as u32;
            let opcode = instr % 100;
            let modes = instr / 100;
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

fn rotate_left(direction: (i64, i64)) -> (i64, i64) {
    match direction {
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        _ => panic!(format!("Invalid direction: {:?}", direction))
    }
}

fn rotate_right(direction: (i64, i64)) -> (i64, i64) {
    match direction {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => panic!(format!("Invalid direction: {:?}", direction))
    }
}

fn add(location: (i64, i64), direction: (i64, i64)) -> (i64, i64) {
    let (x, y) = location;
    let (dx, dy) = direction;
    (x+dx, y+dy)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut vm = VM::create(&input, &[], 10000)?;
    let mut painted = HashMap::new();
    painted.insert((0, 0), 1);
    let mut location = (0, 0);
    let mut vector = (0, -1);
    while vm.state != VMState::Halted {
        let color = painted.get(&location).unwrap_or(&0);
        vm.inputs.push_back(*color);
        vm.run();
        let paint_color = vm.outputs.pop_front().expect("No color provided!");
        let rotation = vm.outputs.pop_front().expect("No rotation provided!");
        println!("Painting: {:?}, {}", location, paint_color);
        painted.insert(location, paint_color);
        vector = if rotation == 0 { rotate_left(vector) } else { rotate_right(vector) };
        location = add(location, vector);
    }
    for row in 0..10 {
        for col in 0..50 {
            if *painted.get(&(col, row)).unwrap_or(&0) == 0 {
                print!(".");
            } else {
                print!("O");
            }
        }
        println!();
    }
    println!("Painted tiles: {:?}", painted.keys().count());
    Ok(())
}
