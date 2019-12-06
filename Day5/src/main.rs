use std::io::{self, Read};

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

fn execute(mut v: Vec<i32>, input : i32)
{
    let mut i = 0;
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
            3 => v[v1] = input,
            4 => println!("Result: {}", get_parameter(&v, v[i + 1], 100, instruction)),
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
    
    println!("Part 1");
    execute(v.clone(),1);
    println!("\n\nPart2 ");
    
    execute(v,5);
}
