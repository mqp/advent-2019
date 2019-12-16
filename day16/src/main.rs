use std::error::Error;
use std::io::{self, Read};
use std::num::ParseIntError;

fn parse_signal(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.chars().map(|c| c.to_string().parse()).collect()
}

fn multiply_signal(signal: &[i32], times: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(signal.len() * times);
    for _ in 0..times {
        result.extend(signal);
    }
    result
}

fn compute_from_offset(signal: &mut [i32], offset: usize) {
    let suffix = &mut signal[offset..];
    let mut sum: i32 = suffix.iter().sum();
    for i in 0..suffix.len() {
        let curr = suffix[i];
        suffix[i] = (sum % 10).abs();
        sum -= curr;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let signal = parse_signal(&input.trim_end())?;
    let offset = input[0..7].parse()?;
    println!("Offset: {}", offset);
    let mut curr = multiply_signal(&signal, 10000);
    for phase in 0..100 {
        println!("Phase {}...", phase);
        compute_from_offset(&mut curr, offset);
    }
    println!("Result: {:?}", &curr[offset..offset+8]);
    Ok(())
}
