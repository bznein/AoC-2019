use intcode::IntcodeMachine;
use intcode::State;
use std::collections::VecDeque;
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

    let num_machines = 50;
    let mut input_queues = vec![VecDeque::new(); num_machines];
    let mut executors = vec![IntcodeMachine::new(v.clone()); num_machines];

    executors
        .iter_mut()
        .enumerate()
        .for_each(|(i, x)| x.set_input(i as i64));

    let mut i = 0;

    let mut nat: (i64, i64) = (0, 0);
    let mut last_y_to_0 = -1;
    let idle_threshold = 2;
    let mut is_idle = vec![0; num_machines];
    let mut part_one_end = 0;
    loop {
        executors[i].run();
        match executors[i].state() {
            State::WaitingForInput => {
                executors[i].set_input(if let Some(x) = input_queues[i].pop_front() {
                    is_idle[i] = 0;
                    x
                } else {
                    is_idle[i] += 1;
                    if is_idle.iter().all(|x| *x >= idle_threshold) {
                        input_queues[0].push_back(nat.0);
                        input_queues[0].push_back(nat.1);
                        if nat.1 == last_y_to_0 {
                            println!("Part 2: {}", nat.1);
                            break;
                        }
                        last_y_to_0 = nat.1;
                        for i in &mut is_idle {
                            *i = 0;
                        }
                    }
                    -1
                });
            }
            State::Stopped => {
                is_idle[i] = 0;
                let val = executors[i].get_output().unwrap();
                executors[i].run();
                let x = executors[i].get_output().unwrap();
                if val != 255 {
                    input_queues[val as usize].push_back(x);
                } else {
                    nat.0 = x;
                    part_one_end += 1;
                }
                executors[i].run();
                let y = executors[i].get_output().unwrap();
                if val == 255 {
                    if part_one_end == 1 {
                        println!("Part 1: {}", y);
                    }
                    nat.1 = y;
                } else {
                    input_queues[val as usize].push_back(y);
                }
            }
            State::Halted => break,
            State::Running => panic!("Still running"),
        }
        i = (i + 1) % num_machines;
    }
}
