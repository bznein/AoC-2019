use std::io::{self, Read};
use intcode::IntcodeMachine;
use intcode::State;

fn print_grid(grid: &[Vec<char>])
{
	for row in grid
	{
		for column in row
		{
			print!("{}", column);
		}
		println!("");
	}
}

fn generate_moves(grid: &[Vec<char>], position: (usize,usize), orientation: char)
{
	let mut position = (position.0 as i32, position.1 as i32);
	let mut orientation = orientation;
	loop
	{
		let mut finished = true;
		let i = position.0 as usize;
		let j = position.1 as usize;
		/* Find position of near # */
		let mut moves = -1;
		if orientation !='v' && i>0 && grid[i-1][j]=='#'
		{
			match orientation
			{
				'^' => panic!("Keep going up!"),
				'v' => panic!("Can't go back!"),
				'>' => 
				{
					print!("L");
					orientation = '^';
				}
				'<' => 
				{
					print!("R");
					orientation = '^';
				}
				_ => panic!("Unknown orientation")
			}
			while position.0>=0 && grid[position.0 as usize ][position.1 as usize ]=='#'
			{
				moves+=1;
				position.0-=1;
			}
			position.0+=1;
			print!("{}, ", moves);
			finished = false;
		}
		else if orientation !='^' && i<grid.len()-1 && grid[i+1][j]=='#'
		{
			match orientation
			{
				'v' => panic!("Keep going down!"),
				'^' => panic!("Can't go back!"),
				'<' => 
				{
					print!("L");
					orientation = 'v';
				}
				'>' => 
				{
					print!("R");
					orientation = 'v';
				}
				_ => panic!("Unknown orientation")
			}
			while position.0<grid.len() as i32 && grid[position.0 as usize ][position.1 as usize ]=='#'
			{
				moves+=1;
				position.0+=1;
			}
			position.0-=1;
			print!("{}, ", moves);
			finished = false;
		}
		else if orientation !='>' && j>0 && grid[i][j-1]=='#'
		{
			match orientation
			{
				'<' => panic!("Keep going left!"),
				'>' => panic!("Can't go back!"),
				'^' => 
				{
					print!("L");
					orientation = '<';
				}
				'v' => 
				{
					print!("R");
					orientation = '<';
				}
				_ => panic!("Unknown orientation")
			}
			while position.1>=0 && grid[position.0 as usize ][position.1 as usize ]=='#'
			{
				moves+=1;
				position.1-=1;
			}
			position.1+=1;
			print!("{}, ", moves);
			finished = false;
		}
		else if orientation !='<' && j<grid[i].len() && grid[i][j+1]=='#'
		{
			match orientation
			{
				'>' => panic!("Keep going Right!"),
				'<' => panic!("Can't go back!"),
				'v' => 
				{
					print!("L");
					orientation = '>';
				}
				'^' => 
				{
					print!("R");
					orientation = '>';
				}
				_ => panic!("Unknown orientation")
			}
			while position.1<grid[i].len() as i32 && grid[position.0 as usize ][position.1 as usize ]=='#'
			{
				moves+=1;
				position.1+=1;
			}
			position.1-=1;
			print!("{}, ", moves);
			finished = false;
		}
		if finished
		{
			return;
		}
	}
}

fn part_1(grid: &[Vec<char>])
{
	let mut tot = 0;
	for (i,row) in grid.iter().enumerate().skip(1).take(grid.len()-4)
	{
		for (j, column) in row.iter().enumerate().skip(1).take(row.len()-2)
		{
			if *column == '#' && grid[i-1][j] == '#'
			&& grid[i+1][j] == '#'
			&& grid[i][j-1] == '#'
			&& grid[i][j+1] == '#'
			{
				tot+=i*j;
			}
		}
	}
	println!("Part 1: {}", tot);
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
	let mut executor = IntcodeMachine::new(v.clone());
	let mut grid : Vec<Vec<char>> = Vec::new();
	grid.push(Vec::new());
	let mut line = 0;
	let mut start_position : (usize,usize) = (0,0);
	let mut start_orientation: char = 'x';
	loop
	{
		executor.run();
		if executor.state()==State::Stopped
		{
			match executor.get_output()
			{
				Some(x) => print!("{}", x as u8 as char),
				None => break
			}
			let x = executor.get_output().unwrap() as u8 as char;
			match x 
			{
				'^'|'>'|'<'|'v' => 
				{
					if grid.len()<line+1
					{
						grid.push(Vec::new());
					}
					start_position = (line,grid[line].len());
					grid[line].push('#');
					start_orientation = x;
				}
				
				'\n' => {line+=1;}
				_ => {
					if grid.len()<line+1
						{
							grid.push(Vec::new());
						}
						grid[line].push(x);
					}
			}
		}
		else if  executor.state()==State::Halted
		{
			break;
		}
	}
	
	part_1(&grid);
	generate_moves(&grid, start_position, start_orientation);
	v[0] = 2;
    let mut executor = IntcodeMachine::new(v.clone());

	
	
	/*let mut inputs = Vec::new();
	
	inputs.push('A' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('B' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('B' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('C' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('C' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('A' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('B' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('B' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('C' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('A' as u8 as i64);
	inputs.push('\n' as u8 as i64);*/
	/* Function A */
	/*inputs.push('R' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('4' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('R' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('1' as u8 as i64);
	inputs.push('2' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('R' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('1' as u8 as i64);
	inputs.push('0' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('L' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('1' as u8 as i64);
	inputs.push('2' as u8 as i64);
	inputs.push('\n' as u8 as i64);*/
	/*----------------------------*/
	/* Function B */
	/*inputs.push('L' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('1' as u8 as i64);
	inputs.push('2' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('R' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('4' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('R' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('1' as u8 as i64);
	inputs.push('2' as u8 as i64);
	inputs.push('\n' as u8 as i64);*/
	/*----------------------------*/
	/* Function C */
/*	inputs.push('L' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('1' as u8 as i64);
	inputs.push('2' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('L' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('8' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('R' as u8 as i64);
	inputs.push(',' as u8 as i64);
	inputs.push('1' as u8 as i64);
	inputs.push('0' as u8 as i64);
	inputs.push('\n' as u8 as i64);*/
	/*----------------------------*/
	/*inputs.push('n' as u8 as i64);
	inputs.push('\n' as u8 as i64);
	let mut i =0;
	loop
	{
		executor.run();
		if executor.state()==State::Stopped
		{
			match executor.get_output()
			{
				Some(x) => 
				{
					match x
					{
						35|10|46|118|94|62|60 => print!("{}", x as u8 as char),
						_ => {
							println!("Dust 2: {}", x);
							println!("{:?}", executor.state());
						}
					}
				}
				None => break
			}
		}
		else if executor.state()==State::WaitingForInput
		{
			executor.set_input(inputs[i]);
			i+=1;
		}
	}*/
}
