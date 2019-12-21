use intcode::IntcodeMachine;
use intcode::State;
use std::io::{self, Read};

fn string_to_command(s: String) -> Vec<i64> {
    s.chars().map(|x| x as u8 as i64).collect()
}

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

    let mut input_idx = 0;
    let commands = string_to_command(String::from(
        "OR A J
         AND C J
         NOT J J
         AND D J
         WALK
         "));
    loop {
        executor.run();
        match executor.state() {
            State::Halted => break,
            State::Stopped => {
                let val = executor.get_output().unwrap();
                if val < 255 {
                    print!("{}", val as u8 as char)
                } else {
                    println!("Part 1: {}\n\n", val);
                }
            }
            State::WaitingForInput => {
                executor.set_input(commands[input_idx]);
                input_idx += 1;
            }
            State::Running => {
                panic!("Still running!!");
            }
        }
    }

    let mut executor = IntcodeMachine::new(v);

    let mut input_idx = 0;
    let commands = string_to_command(String::from(
        "OR A J
         AND B J
         AND C J
         NOT J J
         AND D J
         OR E T
         OR H T
         AND T J
         RUN
         "));
    loop {
        executor.run();
        match executor.state() {
            State::Halted => break,
            State::Stopped => {
                let val = executor.get_output().unwrap();
                if val < 255 {
                    print!("{}", val as u8 as char)
                } else {
                    println!("Part 2: {}", val);
                }
            }
            State::WaitingForInput => {
                executor.set_input(commands[input_idx]);
                input_idx += 1;
            }
            State::Running => {
                panic!("Still running!!");
            }
        }
    }
}
