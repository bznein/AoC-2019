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
    loop {
        executor.set_input(1);
        executor.run();
        let val = executor.get_output().unwrap();
        if val != 0 {
            println!("Part 1: {}", val);
            break;
        }
    }

    let mut executor = IntcodeMachine::new(v.clone());
    executor.set_input(5);
    executor.run();
    println!("Part 2: {}", executor.get_output().unwrap());
}
