use std::collections::HashMap;
use std::io::{self, Read};

fn part1(orbit: &HashMap<String, String>) -> i32 {
    let mut total = 0;
    for mut val in orbit.values() {
        total += 1; // Direct orbit
        while orbit.contains_key(val) {
            total += 1;
            val = &orbit[val];
        }
    }
    total
}

fn part2(orbit: &HashMap<String, String>) -> i32 {
    let mut orbit_you: Vec<String> = Vec::new();
    let mut key = "YOU";
    while key != "COM" {
        key = &orbit[key];
        orbit_you.push(key.to_string());
    }

    let mut orbit_san: Vec<String> = Vec::new();
    let mut key = "SAN";
    while key != "COM" {
        key = &orbit[key];
        orbit_san.push(key.to_string());
    }

    for (i, p) in orbit_you.iter().enumerate() {
        if orbit_san.contains(&p) {
            return (i + orbit_san
                .iter()
                .position(|r| r.cmp(&p) == std::cmp::Ordering::Equal)
                .unwrap()) as i32;
        }
    }
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n') {
        input.truncate(input.len() - 1);
    }

    let v: Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
    let mut orbit = HashMap::new();

    v.iter().for_each(|e| {
        let t: Vec<String> = e.split(')').map(|x| x.to_string()).collect();
        orbit.insert(t[1].clone(), t[0].clone());
    });

    println!("Part 1: {}", part1(&orbit));
    println!("Part 2: {}", part2(&orbit));
}
