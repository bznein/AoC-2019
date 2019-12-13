use intcode::IntcodeMachine;
use std::io::{self, Read};
use std::collections::HashMap;

fn main() {   
	let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n') {
        input.truncate(input.len() - 1);
    }

    let v: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
		
	let mut executor = IntcodeMachine::new(v.clone());
	
	let mut m = HashMap::new();
	
	let mut blocks = 0;
	let mut x =0;
	let mut y =0;
	let mut t_id =0;
	loop
	{
		executor.run();
		let s = executor.get_output();
		match s
		{
			None => break,
			Some(v) => x = v,
		}
		
		executor.run();
		y = executor.get_output().unwrap();
		
		executor.run();
		t_id = executor.get_output().unwrap();
		
		m.insert((x,y),t_id);
		if t_id == 2
		{
			blocks += 1;
		}
	}
	
	println!("Part 1: {}", blocks);
	
	let mut v = v;
	let mut ball_x = -1;
	let mut ball_y = -1;
	let mut pad_x = -1;
	let mut input = 0;
	v[0] = 2;
	let mut executor = IntcodeMachine::new(v.clone());
	loop
	{
		pad_x += input;
		loop
		{
		executor.set_input(input);
			executor.run();
			let s = executor.get_output();
			match s
			{
				None => break,
				Some(v) => x = v,
			}
			
			executor.set_input(input);
			executor.run();
			y = executor.get_output().unwrap();
			
			executor.set_input(input);
			executor.run();
			t_id = executor.get_output().unwrap();
			
			match t_id
			{
				3 => {println!("Updating pad"); pad_x = x}
				4 => {ball_x = x; ball_y = y; println!("updating");}
				_ => (),
			}
			//println!("Ball x: {}", ball_x);
			//println!("pad x: {}", pad_x);
			input = if ball_x < pad_x {-1} else if ball_x > pad_x {1} else {0};
			if x==-1
			{
				println!("Score: {}", t_id);
			}
		}
	}
	
}
