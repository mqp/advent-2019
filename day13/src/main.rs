use std::error::Error;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::cmp::Ordering;
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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum TileKind {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4
}

impl TileKind {
    fn from_word(i: Word) -> Result<Self, Box<dyn Error>> {
        match i {
            0 => Ok(Self::Empty),
            1 => Ok(Self::Wall),
            2 => Ok(Self::Block),
            3 => Ok(Self::Paddle),
            4 => Ok(Self::Ball),
            _ => Err(From::from(format!("Invalid tile: {}", i)))
        }
    }
}

struct GameState {
    score: Word,
    paddle: (Word, Word),
    ball: (Word, Word)
}

fn apply_updates(state: &mut GameState, outputs: &mut VecDeque<Word>) -> Result<(), Box<dyn Error>> {
    let mut vals = outputs.drain(..);
    while let Some(x) = vals.next() {
        let y = vals.next().ok_or("Outputs must come in triples!")?;
        let n = vals.next().ok_or("Outputs must come in triples!")?;
        if x == -1 && y == 0 {
            state.score = n;
        } else {
            // we actually don't give a fuck about anything but the ball and paddle
            let tile = TileKind::from_word(n)?;
            if tile == TileKind::Ball {
                state.ball = (x, y);
            } else if tile == TileKind::Paddle {
                state.paddle = (x, y);
            }
        }
    }
    Ok(())
}

fn determine_direction(state: &GameState) -> Word {
    let (px, _py) = state.paddle;
    let (bx, _by) = state.ball;
    match px.cmp(&bx) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut vm = VM::create(&input, &[], 10000)?;
    vm.mem[0] = 2;
    let mut state = GameState { score: 0, paddle: (0, 0), ball: (0, 0) };
    loop {
        vm.run();
        apply_updates(&mut state, &mut vm.outputs)?;
        if vm.state == VMState::Halted {
            break;
        } else {
            let dir = determine_direction(&state);
            vm.inputs.push_back(dir);
        }
    }
    println!("Game over. Final score: {}", state.score);
    Ok(())
}
