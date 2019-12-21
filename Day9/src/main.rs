use intcode::IntcodeMachine;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n') {
        input.truncate(input.len() - 1);
    }

    let v: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut executor = IntcodeMachine::new(v.clone());
    executor.set_input(1);
    executor.run();
    println!("Part 1: {}", executor.get_output().unwrap());

    let mut executor = IntcodeMachine::new(v);
    executor.set_input(2);
    executor.run();
    println!("Part 2: {}", executor.get_output().unwrap());
}
