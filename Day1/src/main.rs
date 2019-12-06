use std::io::{self, Read};


fn requirements(x: i32) -> i32 {
    let mut ret_val = 0;
    let mut val: i32 = x / 3 - 2;
    while val > 0 {
        ret_val += val;
        val = val / 3 - 2;
    }
    ret_val
}


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!(
        "Part 1 {:?}",
        input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap() / 3 - 2)
            .sum::<i32>()
    );	
 	println!(
        "Part 2 {:?}",
        input
            .split_whitespace()
            .map(|x| requirements(x.parse::<i32>().unwrap()))
            .sum::<i32>()
    );
}
