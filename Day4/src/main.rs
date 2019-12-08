use std::cmp::Ordering;

fn num_to_vec(mut input: u32) -> Vec<u32> {
    let n = (input as f32).log10() as usize + 1;
    let mut buf = Vec::with_capacity(n);
    while input != 0 {
        buf.push(input % 10);
        input /= 10;
    }
    buf.reverse();
    buf
}

fn is_valid_password(vec: &Vec<u32>) -> i32 {
    let mut double_digit = false;
    let mut triple_digit_check = false;
    for i in 1..vec.len() {
        match vec[i].cmp(&vec[i - 1]) {
            Ordering::Less => return 0,
            Ordering::Equal => {
                double_digit = true;
                if (i < 2 || vec[i - 2] != vec[i]) && (i + 1 >= vec.len() || vec[i + 1] != vec[i]) {
                    triple_digit_check = true;
                }
            }
            _ => (),
        }
    }
    if !(double_digit || triple_digit_check) {
        return 0;
    } else if double_digit && !triple_digit_check {
        return 1;
    }
    2
}

fn increment_vec(v: &mut Vec<u32>) {
    let mut pos = v.len();
    while {
        pos -= 1;
        v[pos] = (v[pos] + 1) % 10;
        v[pos] == 0
    } {}
}

fn main() {
    let lower: u32 = 367_479;
    let higher: u32 = 893_698;
    let mut count_p1 = 0;
    let mut count_p2 = 0;
    let mut vec = num_to_vec(lower);
    let high_vec = num_to_vec(higher);
    while vec != high_vec {
        let check = is_valid_password(&vec);
        match check {
            1 => count_p1 += 1,
            2 => {
                count_p1 += 1;
                count_p2 += 1
            }
            0 | _ => (),
        }
        increment_vec(&mut vec);
    }

    println!("Part 1: {}", count_p1);
    println!("Part 2: {}", count_p2);
}
