use std::error::Error;
use std::io::{self, Read};
use std::collections::{VecDeque, HashSet, HashMap};

#[derive(Debug)]
struct Region {
    width: i64,
    height: i64,
    asteroids: HashSet<(i64, i64)>
}

fn gcd(mut m: i64, mut n: i64) -> i64 {
    m = m.abs();
    n = n.abs();
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }
    n.abs()
}

fn parse_region(input: &str) -> Region {
    let mut asteroids = HashSet::new();
    let rows: Vec<_> = input.split_whitespace().collect();
    let height = rows.len();
    let width = rows[0].chars().count();
    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == '#' {
                asteroids.insert((x as i64, y as i64));
            }
        }
    }
    Region { width: width as i64, height: height as i64, asteroids }
}

fn angle_from_vertical(dx: i64, dy: i64) -> f64 {
    let mut rad = (dy as f64).atan2(dx as f64);
    rad += std::f64::consts::FRAC_PI_2;
    if rad < 0.0 {
        rad += std::f64::consts::PI * 2_f64;
    }
    rad
}

fn get_asteroids_dir(region: &Region, from_x: i64, from_y: i64, dx: i64, dy: i64) -> VecDeque<(i64, i64)> {
    let mut result = VecDeque::new();
    let mut candidate_x = from_x;
    let mut candidate_y = from_y;
    while 0 <= candidate_y
        && candidate_y < region.height
        && 0 <= candidate_x
        && candidate_x < region.width
    {
        candidate_x += dx;
        candidate_y += dy;
        if region.asteroids.contains(&(candidate_x, candidate_y)) {
            result.push_back((candidate_x, candidate_y));
        }
    }
    result
}

fn get_asteroids(region: &Region, from_x: i64, from_y: i64) -> HashMap<(i64, i64), VecDeque<(i64, i64)>> {
    let mut result = HashMap::new();
    for other_x in 0..region.width {
        for other_y in 0..region.height {
            let dx = (other_x as i64) - from_x;
            let dy = (other_y as i64) - from_y;
            if gcd(dx, dy) != 1 {
                continue;
            }
            let asteroids = get_asteroids_dir(region, from_x, from_y, dx, dy);
            if asteroids.len() > 0 {
                result.insert((dx, dy), asteroids);
            }
        }
    }
    result
}

fn get_station_coords(region: &Region) -> (i64, i64) {
    let mut best_coord = (0, 0);
    let mut best_count = 0;
    for &(curr_x, curr_y) in &region.asteroids {
        let others = get_asteroids(region, curr_x, curr_y);
        let visible_asteroids = others.keys().count();
        if visible_asteroids > best_count {
            best_count = visible_asteroids;
            best_coord = (curr_x, curr_y);
        }
    }
    best_coord
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let region = parse_region(&input);
    let (station_x, station_y) = get_station_coords(&region);
    let mut others_by_dir = get_asteroids(&region, station_x, station_y);
    let mut others_sorted: Vec<_> = others_by_dir.iter_mut().collect();
    others_sorted.sort_by(|(&(adx, ady), _), (&(bdx, bdy), _)| {
        angle_from_vertical(adx, ady).partial_cmp(&angle_from_vertical(bdx, bdy)).unwrap()
    });
    println!("Station: {}, {}", station_x, station_y);
    let mut vaporized = 0;
    while vaporized < 200 {
        for (&(dx, dy), asteroids) in others_sorted.iter_mut() {
            if let Some((x, y)) = asteroids.pop_front() {
                println!("Blew up: {}, {} in direction {}, {} ({} rad)",
                         x, y, dx, dy, angle_from_vertical(dx, dy));
                vaporized += 1;
                if vaporized == 200 {
                    break;
                }
            }
        }
    }
    Ok(())
}
