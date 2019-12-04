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

fn is_valid_password(num: u32) -> bool {
    let vec = num_to_vec(num);
    let mut double_digit = false;
    for i in 1..vec.len() {
        match vec[i].cmp(&vec[i - 1]) {
            Ordering::Less => return false,
            Ordering::Equal => {
                if (i < 2 || vec[i - 2] != vec[i]) && (i + 1 >= vec.len() || vec[i + 1] != vec[i]) {
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
    let mut count = 0;
    for num in lower..=higher {
        if is_valid_password(num) {
            count += 1;
        }
    }

    println!("{}", count);
}
