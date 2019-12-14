use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug)]
struct Reaction {
    input: Vec<(u64, String)>,
    output: u64,
}

fn requirements(
    s: String,
    amount: u64,
    m: &HashMap<String, Reaction>,
    remainders: &mut HashMap<String, u64>,
) -> u64 {
    let mut amount = amount;
    if s == "ORE" {
        return amount;
    }

    if remainders.contains_key(&s) {
        let v = remainders.get(&s).unwrap();
        let old_amount = amount;
        amount = if amount > *v { amount - *v } else { 0 };
        remainders.insert(s.clone(), if old_amount > *v { 0 } else { *v - old_amount });
    }
    if amount == 0 {
        return 0;
    }
    let r = m.get(&s).unwrap();

    let multiplier: u64 = (amount + (r.output - 1)) / r.output;
    if r.output * multiplier > amount {
        let mult_output = r.output * multiplier;
        remainders
            .entry(s)
            .and_modify(|e| *e += mult_output - amount)
            .or_insert(mult_output - amount);
    }
    let mut tot = 0;
    for i in r.input.clone() {
        tot += requirements(i.1, i.0 * multiplier, m, remainders);
    }
    tot
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n') {
        input.truncate(input.len() - 1);
    }

    let mut m = HashMap::new();
    let mut remainders = HashMap::new();

    let v: Vec<String> = input.split('\n').map(String::from).collect();

    for s in v {
        let strings: Vec<String> = s.split(" => ").map(String::from).collect();
        let before: Vec<String> = strings[0].split(", ").map(String::from).collect();
        let after: Vec<String> = strings[1].split(' ').map(String::from).collect();
        let mut pre = Vec::new();
        for reaction in before {
            let r: Vec<String> = reaction.split(' ').map(String::from).collect();
            let amount = r[0].parse::<u64>().unwrap();
            pre.push((amount, r[1].clone()));
        }

        let amount = after[0].parse::<u64>().unwrap();
        m.insert(
            after[1].clone(),
            Reaction {
                input: pre,
                output: amount,
            },
        );
    }

    println!(
        "Part 1: {}",
        requirements(String::from("FUEL"), 1, &m, &mut remainders)
    );

    let mut fuel = 1;
    let mut increment = 100_000_000_000;
    loop {
        let v = requirements(String::from("FUEL"), fuel, &m, &mut remainders);
        if v > 1_000_000_000_000 {
            if increment == 1 {
                println!("Part 2: {}", fuel - 1);
                break;
            } else {
                fuel -= increment;
                increment /= 10;
            }
        }
        fuel += increment;
    }
}
