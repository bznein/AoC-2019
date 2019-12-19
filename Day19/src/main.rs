
use std::io::{self, Read};
use intcode::State;
use intcode::IntcodeMachine;

fn fits_square(grid: &Vec<Vec<i64>>, i: usize, j: usize, size: usize) -> bool
{
	if i <size  || j<size
	{
		return false;
	}
	for v in 0..size
	{
		if grid[i-v][j] != 1 || grid[i][j-v] !=1
		{
			return false;
		}
	}
	return true;
}

fn print_grid(grid: &Vec<Vec<i64>>)
{
	for i in grid
	{
		for val in i 
		{
			print!("{}", val);
		}
		println!("");
	}
}


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
	let mut grid = vec![vec![0;size];size];
	'outer: for i in 0..size
	{
		let mut consecutive_ones = 0;
		for j in 0..size
		{
			let mut executor = IntcodeMachine::new(v.clone());
			executor.set_input(i as i64);
			executor.run();
			executor.set_input(j as i64);
			executor.run();
			let val = executor.get_output().unwrap();
			grid[i as usize][j as usize]=(val);
			if val ==1
			{
				consecutive_ones += 1;
				if consecutive_ones == 100  && fits_square(&grid,  i, j,100)
				{
					println!("Part 2: {}", (i-99)*10000+(j-99));
					break 'outer;
				}
			}
			else
			{
				consecutive_ones = 0;
			}
		}
	}
}
