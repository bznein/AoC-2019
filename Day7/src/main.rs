use std::io::{self, Read};

use permutohedron::LexicalPermutation;


fn extract_digit(num: i32, div: i32) -> i32 {
    (num / div) % 10
}

fn get_parameter(v: &[i32], position: i32, digit: i32, instruction: i32) -> i32 {
    match extract_digit(instruction, digit) {
        1 => position,
        0 => v[position as usize],
        _ => 0,
    }
}

fn run_amplifier(mut v: Vec<i32>, phase: i32, input : i32) -> i32
{
    let mut i = 0;
    let mut times_input_requested = 0;
    while i < v.len() {
        let instruction = v[i];
        let op_code = v[i] % 100;
        if op_code == 99 {
            break;
        }
        let v1 = v[i + 1] as usize;
        let mut jumped = false;
        match op_code {
            1 => {
                let v3 = v[i + 3] as usize;
                v[v3] = get_parameter(&v, v[i + 1], 100, instruction)
                    + get_parameter(&v, v[i + 2], 1000, instruction)
            }
            2 => {
                let v3 = v[i + 3] as usize;
                v[v3] = get_parameter(&v, v[i + 1], 100, instruction)
                    * get_parameter(&v, v[i + 2], 1000, instruction)
            }
            3 => {v[v1] = match times_input_requested
            {
                0 => phase,
                1 => input,
                _ => -1
            };
                  times_input_requested=1;
            },
            4 => return get_parameter(&v, v[i + 1], 100, instruction),
            5 => {
                if get_parameter(&v, v[i + 1], 100, instruction) != 0 {
                    i = get_parameter(&v, v[i + 2], 1000, instruction) as usize;
                    jumped = true;
                }
            }
            6 => {
                if get_parameter(&v, v[i + 1], 100, instruction) == 0 {
                    i = get_parameter(&v, v[i + 2], 1000, instruction) as usize;
                    jumped = true;
                }
            }
            7 => {
                let v3 = v[i + 3] as usize;
                v[v3] = (get_parameter(&v, v[i + 1], 100, instruction)
                    < get_parameter(&v, v[i + 2], 1000, instruction)) as i32;
            }
            8 => {
                let v3 = v[i + 3] as usize;
                v[v3] = (get_parameter(&v, v[i + 1], 100, instruction)
                    == get_parameter(&v, v[i + 2], 1000, instruction)) as i32;
            }
            99 => break,
            _ => (),
        }
        if !jumped {
            i += match op_code {
                1 | 2 | 7 | 8 => 4,
                3 | 4 => 2,
                5 | 6 => 3,
                _ => 0,
            };
        }
    }
    -1
}


fn main ()
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n')
    {
        input.truncate(input.len()-1);
    }

    let v: Vec<i32> = input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap()).collect();

    let mut data = [0, 1, 2, 3, 4];
    let mut permutations = Vec::new();

    loop {
        permutations.push(data.to_vec());
        if !data.next_permutation() {
            break;
        }
    }

    let mut  max_power : i32 = -1;
    for values in permutations
    {
        let mut vv = run_amplifier(v.clone(), values[0], 0);
        for i in 1..=4
        {
            let p = vv;
            vv = run_amplifier(v.clone(), values[i], p);
        }
        if max_power < vv
        {
            max_power = vv;
        }
    }
    println!("Part 1: {}" , max_power);


    let values = [9,7,8,5,6];
    let mut first = 0;
    loop
    {
        let mut vv : i32 =  run_amplifier(v.clone(), values[0], first);
        for i in 1..=4
        {
            vv = run_amplifier(v.clone(), values[i], vv);
        }
        first = vv;
        println!("vv: {}" ,vv);
    }

}
