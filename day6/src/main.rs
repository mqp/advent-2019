use std::error::Error;
use std::io::{self, Read};
use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;

fn parse_orbit(s: &str) -> Result<(&str, &str), Box<dyn Error>> {
    let mut parts = s.split(")");
    let inner = parts.next().ok_or("Invalid orbit.")?;
    let outer = parts.next().ok_or("Invalid orbit.")?;
    Ok((inner, outer))
}

fn parse_graph(input: &str) -> Result<HashMap<&str, &str>, Box<dyn Error>> {
    let mut graph = HashMap::new();
    for orbit in input.split_whitespace().map(parse_orbit) {
        let (inner, outer) = orbit?;
        graph.insert(outer, inner);
    }
    Ok(graph)
}

fn get_path<'a>(graph: &'a HashMap<&'a str, &'a str>, outer: &'a str, target: &'a str) -> VecDeque<&'a str> {
    let mut result = VecDeque::new();
    let mut curr = outer;
    while let Some(&parent) = graph.get(curr) {
        result.push_front(parent);
        if parent == target {
            return result;
        }
        curr = parent;
    }
    panic!(format!("No path from {} to {}!", outer, target));
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let graph = parse_graph(&input)?;
    let mut com_to_you = get_path(&graph, "YOU", "COM");
    let mut com_to_san = get_path(&graph, "SAN", "COM");
    loop {
        if com_to_you.pop_front() != com_to_san.pop_front() {
            break;
        }
    }
    println!("Result: {}", com_to_you.len() + com_to_san.len() + 2); // +2 because we popped one extra level
    Ok(())
}
