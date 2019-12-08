use std::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let width = 25;
    let height = 6;
    let chars: Vec<_> = input.trim_end().chars().collect();
    let layers: Vec<_> = chars.chunks(width*height).collect();
    for i in 0..height {
        for j in 0..width {
            let mut color = '2';
            for layer in &layers {
                if color == '2' {
                    color = layer[width*i + j];
                }
            }
            print!("{}", color);
        }
        print!("\n");
    }
    Ok(())
}
