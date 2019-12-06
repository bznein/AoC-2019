use std::convert::TryInto;
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
    let input = String::from("1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,1,19,9,23,1,23,6,27,1,9,27,31,1,31,10,35,2,13,35,39,1,39,10,43,1,43,9,47,1,47,13,51,1,51,13,55,2,55,6,59,1,59,5,63,2,10,63,67,1,67,9,71,1,71,13,75,1,6,75,79,1,10,79,83,2,9,83,87,1,87,5,91,2,91,9,95,1,6,95,99,1,99,5,103,2,103,10,107,1,107,6,111,2,9,111,115,2,9,115,119,2,13,119,123,1,123,9,127,1,5,127,131,1,131,2,135,1,135,6,0,99,2,0,14,0");
    let v: Vec<i32> = input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    



    println!("Part 1: {:}", get_final_value(12,2,v.clone()));
	
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
