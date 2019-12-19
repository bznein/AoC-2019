use std::io::{self, Read};
use intcode::State;
use intcode::IntcodeMachine;

fn main() {

let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n') {
        input.truncate(input.len() - 1);
    }

    let mut v: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
	
	let mut tot = 0;
	
	let size = 50;
	let mut grid = vec![vec![0;size];size];
	for i in 0..size
	{
		for j in 0..size
		{
			let mut executor = IntcodeMachine::new(v.clone());
			executor.set_input(i as i64);
			executor.run();
			executor.set_input(j as i64);
			executor.run();
			let val = executor.get_output().unwrap();
			grid[j as usize][i as usize]=(val);
			tot += val;
		}
	}
	println!("Part 1: {}", tot);
	
	
	let size = 2000;
	let mut start_j=0;
	'outer: for i in 0..size
	{
		let mut one_found = false;
		for j in start_j..size
		{
			let mut executor = IntcodeMachine::new(v.clone());
			executor.set_input(i as i64);
			executor.run();
			executor.set_input(j as i64);
			executor.run();
			let val = executor.get_output().unwrap();
			if val == 1
			{
				one_found = true;
				start_j = j;
				let mut executor = IntcodeMachine::new(v.clone());
				executor.set_input(i as i64 +99);
				executor.run();
				executor.set_input(j as i64);
				executor.run();
				let val = executor.get_output().unwrap();
				if val ==1
				{
					let mut executor = IntcodeMachine::new(v.clone());
					executor.set_input(i as i64);
					executor.run();
					executor.set_input(j as i64 + 99);
					executor.run();
					let val = executor.get_output().unwrap();
					if val ==1
					{
						println!("Part 2: {}", (i)*10000+(j));
						break 'outer;
					}
					else
					{
						break;
					}
				}
			}
			else
			{
				if one_found
				{
					break;
				}
			}
		
		}
	}
}
