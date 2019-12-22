use std::io::{self, Read};
use std::collections::HashMap;
use modinverse::modinverse;
use mod_exp::mod_exp;

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n') {
        input.truncate(input.len() - 1);
    }

    let params : Vec<_>= input.split('\n').map(|x| x.split_whitespace().collect::<Vec<_>>()).collect();

    let cmd : Vec<(usize,i128)>=
        params
        .iter()
        .map(|x| if x[0]=="deal"
             {
                 if x[1] == "into"
                 {
                     (0,-1)
                 }
                 else
                 {
                     (1,x[3].parse::<i128>().unwrap())
                 }
             }
             else
             {
                 (2,x[1].parse::<i128>().unwrap())
             }
        )
        .collect();


    let num_cards : i128 = 10007;
    let mut index : i128 = 2019;

    for param in &cmd
    {
        index = match param.0
        {
            0 => num_cards-1-index,
            1 => (index*param.1)%num_cards,
            2 => (index-param.1+num_cards)%num_cards,
            _ => panic!("Unknown command!"),
        }
    }

    println!("Part 1: {}", index);



    let num_cards : i128 = 119315717514047;
    let index : i128 = 2020;
    let num_shuffles : i128 = 101741582076661;

    let mut a = 1;
    let mut b = 0;
    for param in cmd.iter().rev()
    {
        match param.0
        {
            0 => {
                a*=-1;
                b+=1;
                b*=-1;
            }
            1 =>
            {
                let inv = modinverse(param.1,num_cards).unwrap();
                a = a * inv % num_cards;
                b = b * inv % num_cards;
            }
            2 =>
            {
                b += if param.1 < 0 {param.1 + num_cards} else {param.1};
            }
            _ => panic!("Unknown command!"),
        }
        a %= num_cards;
        b %= num_cards;

        if a < 0 {
            a += num_cards;
        }

        if b < 0 {
            b += num_cards;
        }
    }

    let a_n = mod_exp(a,num_shuffles,num_cards);
    if a==1
    {
        println!("Part 2: {}", a_n*index+b*num_shuffles);
        return;
    }
    let a_n1 = (a_n-1)%num_cards;
    let inv = modinverse(a-1,num_cards).unwrap();
    println!("Part 2: {}", (index*a_n + ((b * a_n1)%num_cards) * inv)%num_cards);


}
