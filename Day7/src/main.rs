use intcode::IntcodeMachine;
use permutohedron::LexicalPermutation;
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

    let mut data = [0, 1, 2, 3, 4];
    let mut permutations = Vec::new();

    loop {
        permutations.push(data.to_vec());
        if !data.next_permutation() {
            break;
        }
    }

    let mut max_power: i64 = -1;
    let mut amps: Vec<IntcodeMachine>;
    for values in permutations {
        amps = Vec::new();
        for item in values.iter().take(5) {
            let mut executor = IntcodeMachine::new(v.clone());
            executor.set_phase(*item);
            amps.push(executor);
        }

        amps[0].set_input(0);
        amps[0].run();
        let mut vv = amps[0].get_output().unwrap();
        for item in amps.iter_mut().take(4 + 1).skip(1) {
            item.set_input(vv);
            item.run();
            vv = item.get_output().unwrap();
        }
        if max_power < vv {
            max_power = vv;
        }
    }
    println!("Part 1: {}", max_power);

    let mut data = [5, 6, 7, 8, 9];
    let mut permutations = Vec::new();
    let mut max_signal = -1;

    loop {
        permutations.push(data.to_vec());
        if !data.next_permutation() {
            break;
        }
    }

    for values in permutations {
        amps = Vec::new();
        for item in values.iter().take(5) {
            let mut executor = IntcodeMachine::new(v.clone());
            executor.set_phase(*item);
            amps.push(executor);
        }

        let mut input = 0;
        loop {
            amps[0].set_input(input);
            amps[0].run();
            let mut vv = amps[0].get_output();
            if !vv.is_some() {
                if input > max_signal {
                    max_signal = input;
                }
                break;
            }
            let mut vv = vv.unwrap();
            for item in amps.iter_mut().take(4 + 1).skip(1) {
                item.set_input(vv);
                item.run();
                vv = item.get_output().unwrap();
            }
            input = vv;
        }
    }

    println!("Part 2: {}", max_signal);
}
