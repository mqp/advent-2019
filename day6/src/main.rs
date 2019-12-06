use std::error::Error;
use std::io::{self, Read};
use std::collections::HashMap;

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

fn get_ancestors<'a>(graph: &'a HashMap<&'a str, &'a str>, mut node: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    while let Some(&parent) = graph.get(node) {
        result.push(parent);
        node = parent;
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let graph = parse_graph(&input)?;
    let mut you_ancestors = get_ancestors(&graph, "YOU");
    let mut san_ancestors = get_ancestors(&graph, "SAN");
    while you_ancestors.pop() == san_ancestors.pop() {
        // continue going down the common ancestors
    }
    println!("Result: {}", you_ancestors.len() + san_ancestors.len() + 2); // +2 because we popped one extra level
    Ok(())
}
