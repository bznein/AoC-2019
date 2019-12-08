use std::io::{self, Read};

fn layer(v: &[i32], message : & mut Vec<i32>) -> (i32, i32, i32)
{
    let mut zeroes = 0;
    let mut ones = 0;
    let mut twos = 0;

    for (i,val) in v.iter().enumerate()
    {
        match val
        {
            0 => zeroes+=1,
            1 => ones+=1,
            2 => twos+=1,
            _ => (),
        }
        if message[i] == -1 && *val != 2
        {
            message[i] = *val;
        }
    }

    (zeroes, ones, twos)
}

fn main ()
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n')
    {
        input.truncate(input.len()-1);
    }

    let image : Vec<i32> = input.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
    let w : i32 = 25;
    let h : i32 = 6;
    let size : usize = w as usize * h as usize;
    let mut min_zeroes : i32 = w*h;
    let mut final_val = 0;
    let mut message : Vec<i32> = vec![-1; size];

    for i in 0..image.len()/(size)
    {
        let (z,o,t) = layer(&image[(i*size)..(i+1)*size], &mut message);
        if z < min_zeroes
        {
            min_zeroes = z;
            final_val = o * t;
        }
    }
    println!("Part 1: {}" , final_val);
    println!("Part 2: \n");

    let w = w as usize;
    let h = h as usize;
    for i in 0..h
    {
        println!("{:?}", &message[i*w..(i+1)*w]);
    }
}
