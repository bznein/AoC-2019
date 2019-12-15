use intcode::IntcodeMachine;
use itertools::Itertools;
use std::io::{self, Read};
use std::process;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n') {
        input.truncate(input.len() - 1);
    }
    let mut v: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    v[1] = 12;
    v[2] = 2;
    let mut executor = IntcodeMachine::new(v.clone());
    executor.run();
    println!("Part 1: {:}", executor.peek_memory(0));
    let output: i64 = 19_690_720;

    (0..99)
        .cartesian_product(0..99)
        .find(|(i, j)| {
            v[1] = *i; v[2]= *j;
            let mut exec = IntcodeMachine::new(v.clone());
            exec.run();
            exec.peek_memory(0)
        } == output)
        .iter()
        .for_each(|(i, j)| println!("Part 2: {}", 100 * i + j));
}
