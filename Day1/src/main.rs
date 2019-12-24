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
    let res = input
        .split_whitespace()
        .fold((0,0), |acc,x|
              {
                  let v = x.parse::<i32>().unwrap();
                  (acc.0+v,acc.1+requirements(v))
              });
    println!("Part 1 {}\nPart2: {}", res.0, res.1);
 
}
