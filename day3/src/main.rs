use std::error::Error;
use std::io::{self, Read};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Span<'a> {
    direction: &'a str,
    distance: u32
}

impl<'a> Span<'a> {
    fn from_str(s: &'a str) -> Result<Self, Box<dyn Error>> {
        let (direction, rest) = s.split_at(1);
        Ok(Self { direction, distance: rest.parse()? })
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point2D {
    x: i32,
    y: i32
}

fn parse_input(input: &str) -> Result<Vec<Vec<Span>>, Box<dyn Error>> {
    input.split_whitespace().map(parse_wire).collect()
}

fn parse_wire(wire: &str) -> Result<Vec<Span>, Box<dyn Error>> {
    wire.split(',').map(Span::from_str).collect()
}

fn get_locations(wire: &[Span]) -> Result<HashMap<Point2D, u64>, Box<dyn Error>> {
    let mut locations = HashMap::new();
    let mut steps = 0;
    let (mut x, mut y) = (0, 0);
    for instruction in wire {
        for _ in 0..instruction.distance {
            match instruction.direction {
                "U" => { y += 1; }
                "D" => { y -= 1; }
                "R" => { x += 1; }
                "L" => { x -= 1; }
                _ => return Err(From::from(format!("Fishy direction: {}", instruction.direction)))

            }
            steps += 1;
            locations.insert(Point2D { x, y }, steps);
        }
    }
    Ok(locations)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let wires = parse_input(input.trim_end())?;
    let left_wire = wires.get(0).ok_or("No left wire in input!")?;
    let right_wire = wires.get(1).ok_or("No right wire in input!")?;
    let left_locations = get_locations(left_wire)?;
    let right_locations = get_locations(right_wire)?;

    let mut min_point: Option<Point2D> = None;
    let mut min_steps = u64::max_value();
    for (left_point, left_steps) in left_locations.iter() {
        if let Some(right_steps) = right_locations.get(left_point) {
            let total_steps = left_steps + right_steps;
            if total_steps < min_steps {
                min_steps = total_steps;
                min_point = Some(left_point.clone());
            }
        }
    }
    println!("Result is: {:?} with {} steps", min_point, min_steps);
    Ok(())
}
