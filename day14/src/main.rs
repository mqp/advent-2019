use std::error::Error;
use std::io::{self, Read};
use std::collections::HashMap;

type Reagent<'a> = (i64, String);

type Cookbook<'a> = HashMap<String, (i64, Vec<Reagent<'a>>)>;

fn parse_pair(input: &str) -> Result<Reagent, Box<dyn Error>> {
    let mut parts = input.split_whitespace();
    let qty = parts.next().ok_or("No quantity specified!")?.parse()?;
    let chemical = parts.next().ok_or("No chemical specified!")?;
    Ok((qty, chemical.to_owned()))
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

fn required_for<'a>(book: &'a Cookbook, output_qty: i64, output: &'a str) -> HashMap<String, i64> {
    let mut desired = HashMap::new(); // chemical: remaining desired qty
    let mut present = HashMap::new();
    desired.insert(output.to_owned(), output_qty);
    loop {
        let mut desired_ch = String::new();
        let mut desired_qty = 0;
        for (ch, qty) in &desired {
            if ch != "ORE" {
                desired_ch = ch.clone();
                desired_qty = *qty;
                break;
            }
        }
        if desired_ch.is_empty() {
            return desired;
        }
        desired.remove(&desired_ch);
        let (per_batch, inputs) = book.get(&desired_ch).unwrap();
        let batches = (desired_qty - 1) / per_batch + 1;
        for (c_qty, c_chem) in inputs.iter() {
            let already_have = match present.remove(c_chem) {
                None => 0,
                Some(n) => n
            };
            let required_qty = c_qty * batches;
            if already_have > required_qty {
                present.insert(c_chem.to_owned(), already_have - required_qty);
            } else if already_have < required_qty {
                *desired.entry(c_chem.to_owned()).or_insert(0) += required_qty - already_have;
            }
        }
        if batches > 0 {
            let leftover = per_batch * batches - desired_qty;
            *present.entry(desired_ch.to_owned()).or_insert(0) += leftover;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let reactions = parse_reactions(&input)?;
    let mut lo = 1;
    let mut hi = 2;
    let target = 1_000_000_000_000;
    loop {
        let req = *required_for(&reactions, hi, "FUEL").get("ORE").unwrap();
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
        let req = *required_for(&reactions, mid, "FUEL").get("ORE").unwrap();
        println!("Required for {} fuel: {} ore", req, mid);
        if req > target {
            hi = mid
        } else {
            lo = mid
        }
    }
    Ok(())
}
