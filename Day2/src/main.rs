use std::convert::TryInto;
use std::io::{self, Read};
use std::process;

fn get_final_value(noun: i32, verb: i32, mut v: Vec<i32>) -> i32 {
    v[1] = noun;
    v[2] = verb;
    for i in (0..v.len()).step_by(4) {
        let v1: usize = v[i + 1].try_into().unwrap();
        let v2: usize = v[i + 2].try_into().unwrap();
        let v3: usize = v[i + 3].try_into().unwrap();
        match v[i] {
            1 => v[v3] = v[v1] + v[v2],
            2 => v[v3] = v[v1] * v[v2],
            99 => break,
            _ => (),
        }
    }

    v[0]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n')
    {
        input.truncate(input.len()-1);
    }
    let v: Vec<i32> = input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("Part 1: {:}", get_final_value(12, 2, v.clone()));
    let output: i32 = 19_690_720;

    for i in 0..=99 {
        for j in 0..=99 {
            if get_final_value(i, j, v.clone()) == output {
                println!("Part 2: {}", 100 * i + j);
                process::exit(0);
            }
        }
    }
}
