use std::cmp::Ordering;
use std::error::Error;
use std::io::{self, Read};
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Moon {
    position: [i64; 3],
    velocity: [i64; 3]
}

fn parse_moons(input: &str) -> Result<Vec<Moon>, Box<dyn Error>> {
    let mut moons = Vec::new();
    let re = Regex::new(r"(\-?\d+)").unwrap();
    for line in input.trim_end().split('\n') {
        let mut matches = re.find_iter(line);
        moons.push(Moon {
            position: [
                matches.next().unwrap().as_str().parse()?,
                matches.next().unwrap().as_str().parse()?,
                matches.next().unwrap().as_str().parse()?
            ],
            velocity: [0, 0, 0]
        });
    }
    Ok(moons)
}

fn apply_gravity(moons: &mut [Moon], dim: usize) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if i != j {
                moons[i].velocity[dim] += match moons[i].position[dim].cmp(&moons[j].position[dim]) {
                    Ordering::Less => 1,
                    Ordering::Greater => -1,
                    Ordering::Equal => 0
                }
            }
        }
    }
}

fn apply_velocity(moons: &mut [Moon], dim: usize) {
    for m in moons.iter_mut() {
        m.position[dim] += m.velocity[dim];
    }
}

fn step(moons: &mut [Moon], dim: usize) {
    apply_gravity(moons, dim);
    apply_velocity(moons, dim);
}

fn gcd(mut m: usize, mut n: usize) -> usize {
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }
    n
}

fn lcm(m: usize, n: usize) -> usize {
    m * (n / gcd(m, n))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut moons = parse_moons(&input)?;
    let cycle_lengths: Vec<_> = (0..3).map(|d| {
        let original = (
            (moons[0].position[d],
             moons[1].position[d],
             moons[2].position[d],
             moons[3].position[d]),
            (moons[0].velocity[d],
             moons[1].velocity[d],
             moons[2].velocity[d],
             moons[3].velocity[d]),
        );
        for i in 1.. {
            step(&mut moons, d);
            if original == (
                (moons[0].position[d],
                 moons[1].position[d],
                 moons[2].position[d],
                 moons[3].position[d]),
                (moons[0].velocity[d],
                 moons[1].velocity[d],
                 moons[2].velocity[d],
                 moons[3].velocity[d]),
            ) {
                println!("Dimension {}: repeated after {} steps", d, i);
                return i;
            }
        }
        unreachable!();
    }).collect();

    let result = lcm(lcm(cycle_lengths[0], cycle_lengths[1]), cycle_lengths[2]);
    println!("Total period: {}", result);
    Ok(())
}
