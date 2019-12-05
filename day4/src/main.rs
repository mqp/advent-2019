use std::collections::HashMap;

fn is_valid_adjacency(pw: &str) -> bool {
    // this is correct given that all valid passwords are non-decreasing
    let mut counts = HashMap::new();
    for c in pw.chars() {
        *counts.entry(c).or_insert(0) += 1
    }
    for c in counts.values() {
        if *c == 2 {
            return true;
        }
    }
    return false;
}

fn is_non_decreasing(pw: &str) -> bool {
    let mut prev = '0';
    for ch in pw.chars() {
        if ch < prev {
            return false;
        }
        prev = ch;
    }
    return true;
}

fn main() {
    let mut count = 0;
    for candidate in 271973..785961 {
        let pw = candidate.to_string();
        if is_non_decreasing(&pw) && is_valid_adjacency(&pw) {
            count += 1;
        }
    }
    println!("Result is: {} ", count);
}
