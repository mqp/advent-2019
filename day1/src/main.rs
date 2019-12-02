use std::cmp;
use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};

fn fuel_per_mass(mass: u32) -> u32 {
    cmp::max(2, mass / 3) - 2
}

fn fuel_per_module(module_mass: u32) -> u32 {
    let mut remaining_mass = module_mass;
    let mut module_fuel: u32 = 0;
    while remaining_mass > 0 {
        let fuel = fuel_per_mass(remaining_mass);
        module_fuel += fuel;
        remaining_mass = fuel;
    }
    module_fuel
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = File::open("input")?;
    let reader = BufReader::new(input);
    let mut total_fuel: u32 = 0;
    for line in reader.lines() {
        let module_mass: u32 = line?.parse()?;
        total_fuel += fuel_per_module(module_mass)
    }
    println!("Result is: {}", total_fuel);
    Ok(())
}
