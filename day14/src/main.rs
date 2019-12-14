use std::cmp::Ordering;
use std::error::Error;
use std::io::{self, Read};
use std::collections::HashMap;

type Reagent<'a> = (usize, &'a str);

type Cookbook<'a> = HashMap<&'a str, (usize, Vec<Reagent<'a>>)>;

fn parse_pair(input: &str) -> Result<Reagent, Box<dyn Error>> {
    let mut parts = input.split_whitespace();
    let qty = parts.next().ok_or("No quantity specified!")?.parse()?;
    let chemical = parts.next().ok_or("No chemical specified!")?;
    Ok((qty, chemical))
}

fn parse_reactions(input: &str) -> Result<Cookbook, Box<dyn Error>> {
    let mut reactions = HashMap::new();
    for line in input.trim_end().split('\n') {
        let mut sides = line.split(" => ");
        let left = sides.next().ok_or("Equation must have both sides!")?;
        let (output_qty, output_chemical) = parse_pair(sides.next().ok_or("Equation must have both sides!")?)?;
        let mut reagents = Vec::new();
        for reagent in left.split(", ") {
            reagents.push(parse_pair(reagent)?);
        }
        reactions.insert(output_chemical, (output_qty, reagents));
    }
    Ok(reactions)
}

fn required_for<'a>(book: &'a Cookbook, output: &'a str, output_qty: usize) -> HashMap<&'a str, usize> {
    let mut desired = HashMap::new();
    let mut present = HashMap::new();
    desired.insert(output, output_qty);
    while let Some((ch, (qty, inputs))) = book.iter().filter(|(&ch, _)| desired.contains_key(ch)).next() {
        let desired_qty = desired.remove(ch).unwrap();
        let batches = (desired_qty - 1) / qty + 1;
        for (input_qty, input_ch) in inputs.iter() {
            let already_have = present.entry(input_ch).or_insert(0);
            let required_qty = input_qty * batches;
            match (*already_have).cmp(&required_qty) {
                Ordering::Less => { *desired.entry(input_ch).or_insert(0) += required_qty - *already_have; }
                Ordering::Greater => { *already_have = *already_have - required_qty; }
                _ => {}
            }
        }
    }
    desired
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let reactions = parse_reactions(&input)?;
    let mut lo = 1;
    let mut hi = 2;
    let target = 1_000_000_000_000;
    loop {
        let req = *required_for(&reactions, "FUEL", hi).get("ORE").ok_or("No ore in the output!")?;
        println!("Required for {} fuel: {} ore", req, hi);
        if req < target {
            lo = hi;
            hi = hi * 2;
        } else {
            break;
        }
    }
    while hi - lo >= 2 {
        let mid = lo + (hi - lo) / 2;
        let req = *required_for(&reactions, "FUEL", mid).get("ORE").ok_or("No ore in the output!")?;
        println!("Required for {} fuel: {} ore", req, mid);
        if req > target {
            hi = mid
        } else {
            lo = mid
        }
    }
    Ok(())
}
