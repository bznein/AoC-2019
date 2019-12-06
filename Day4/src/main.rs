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

fn is_valid_password(num: u32, triple_digit_check : bool) -> bool {
    let vec = num_to_vec(num);
    let mut double_digit = false;
    for i in 1..vec.len() {
        match vec[i].cmp(&vec[i - 1]) {
            Ordering::Less => return false,
            Ordering::Equal => {
                if (!triple_digit_check || i < 2 || vec[i - 2] != vec[i]) && (i + 1 >= vec.len() || vec[i + 1] != vec[i]) {
                    double_digit = true;
                }
            }
            _ => (),
        }
    }
    double_digit
}

fn main() {
    let lower: u32 = 367_479;
    let higher: u32 = 893_698;
    let mut count_p1 = 0;
    let mut count_p2 = 0;
    for num in lower..=higher {
        if is_valid_password(num, false) {
            count_p1 += 1;
        }
        if is_valid_password(num, true) {
            count_p2 += 1;
        }
    }

    println!("Part 1: {}", count_p1);
    println!("Part 2: {}", count_p2);
}
