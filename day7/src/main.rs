use std::error::Error;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::collections::VecDeque;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum VMState {
    Ready,
    Halted,
    Waiting
}

#[derive(Debug)]
struct VM {
    pc: usize,
    mem: Vec<i64>,
    state: VMState,
    inputs: VecDeque<i64>,
    outputs: VecDeque<i64>
}

fn val(mem: &[i64], data: i64, mode: usize) -> i64 {
    if mode == 1 { data } else { mem[data as usize] }
}

impl VM {

    fn create(program: &[i64]) -> Self {
        Self {
            pc: 0,
            mem: program.to_owned(),
            state: VMState::Ready,
            inputs: VecDeque::new(),
            outputs: VecDeque::new()
        }
    }

    fn run(self: &mut Self) {
        let VM { ref mut pc, ref mut mem, ref mut state, ref mut inputs, ref mut outputs } = self;
        loop {
            let instr = mem[*pc] as usize;
            let modenum = instr / 100;
            let opcode = instr - (modenum * 100);
            let modes = [
                (modenum % 10),
                (modenum % 100) / 10
            ];
            match opcode {
                1 => { // add
                    let noun = mem[*pc+1];
                    let verb = mem[*pc+2];
                    let dest = mem[*pc+3] as usize;
                    mem[dest] = val(mem, noun, modes[0]) + val(mem, verb, modes[1]);
                    *pc += 4;
                }
                2 => { // mul
                    let noun = mem[*pc+1];
                    let verb = mem[*pc+2];
                    let dest = mem[*pc+3] as usize;
                    mem[dest] = val(mem, noun, modes[0]) * val(mem, verb, modes[1]);
                    *pc += 4;
                }
                3 => { // input
                    if let Some(val) = inputs.pop_front() {
                        let dest = mem[*pc+1] as usize;
                        mem[dest] = val;
                        *pc += 2;
                    } else {
                        *state = VMState::Waiting;
                        return;
                    }
                }
                4 => { // output
                    let noun = mem[*pc+1];
                    outputs.push_back(val(mem, noun, modes[0]));
                    *pc += 2;
                }
                5 => { // jump if true
                    let noun = mem[*pc+1];
                    let verb = mem[*pc+2];
                    *pc = if val(mem, noun, modes[0]) != 0 { val(mem, verb, modes[1]) as usize } else { *pc+3 };
                }
                6 => { // jump if false
                    let noun = mem[*pc+1];
                    let verb = mem[*pc+2];
                    *pc = if val(mem, noun, modes[0]) == 0 { val(mem, verb, modes[1]) as usize } else { *pc+3 };
                }
                7 => { // lt
                    let noun = mem[*pc+1];
                    let verb = mem[*pc+2];
                    let dest = mem[*pc+3] as usize;
                    mem[dest] = if val(mem, noun, modes[0]) < val(mem, verb, modes[1]) { 1 } else { 0 };
                    *pc += 4;
                }
                8 => { // eq
                    let noun = mem[*pc+1];
                    let verb = mem[*pc+2];
                    let dest = mem[*pc+3] as usize;
                    mem[dest] = if val(mem, noun, modes[0]) == val(mem, verb, modes[1]) { 1 } else { 0 };
                    *pc += 4;
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

fn parse_program(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split(',').map(|s| s.parse()).collect()
}

fn part1(program: &[i64]) {
    let mut best_inputs = Vec::new();
    let mut best_thrust = 0;
    let candidates = vec![0, 1, 2, 3, 4].into_iter().permutations(5);
    for phases in candidates {
        let mut vms: Vec<_> = phases.iter().map(|&p| {
            let mut vm = VM::create(program);
            vm.inputs.push_back(p);
            vm
        }).collect();
        vms[0].inputs.push_back(0);
        vms[0].run();
        for i in 1..5 {
            let output = vms[i-1].outputs[0];
            vms[i].inputs.push_back(output);
            vms[i].run();
        }
        let output = vms[4].outputs.pop_front().unwrap();
        if output > best_thrust {
            best_thrust = output;
            best_inputs = phases;
        }
    }
    println!("Result: {:?} -> {}", best_inputs, best_thrust);
}

fn part2(program: &[i64]) {
    let mut best_inputs = Vec::new();
    let mut best_thrust = 0;
    let candidates = vec![5, 6, 7, 8, 9].into_iter().permutations(5);
    for phases in candidates {
        let mut vms: Vec<_> = phases.iter().map(|&p| {
            let mut vm = VM::create(program);
            vm.inputs.push_back(p);
            vm
        }).collect();
        vms[0].inputs.push_back(0);
        let mut halted = 0;
        while halted != 5 {
            for i in 0..5 {
                let vm = &mut vms[i];
                vm.run();
                if vm.state == VMState::Halted {
                    halted += 1;
                }
                if halted != 5 {
                    if let Some(val) = vm.outputs.pop_front() {
                        vms[(i+1)%5].inputs.push_back(val);
                    }
                }
            }
        }
        let output = vms[4].outputs.pop_front().unwrap();
        if output > best_thrust {
            best_thrust = output;
            best_inputs = phases;
        }
    }
    println!("Result: {:?} -> {}", best_inputs, best_thrust);
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let program = parse_program(input.trim_end())?;
    println!("Memory: {:?}", program);
    part1(&program);
    part2(&program);
    Ok(())
}
