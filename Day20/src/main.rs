use std::io::{self, Read};
use std::io::{stdout, Write};
use std::collections::HashMap;
use crossterm::{cursor::*, execute, style::*, terminal::*, ExecutableCommand};
use std::time::Duration;
use std::thread;

fn print_grid(grid: &[Vec<char>], position: (usize,usize), depth: usize)
{
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0), Hide, Clear(ClearType::All));
	for (i,r) in grid.iter().enumerate()
	{
		for (j,val) in r.iter().enumerate()
		{
			if (i,j)==position 
			{
				execute!(stdout, SetForegroundColor(Color::Red));
				print!("@");
			}
			else
			{
				execute!(stdout, SetForegroundColor(Color::Reset));
				print!("{}", val);
			}
		}
		println!("");
	}
    execute!(stdout, MoveTo(55, 15));
	print!("Depth: {}", depth);
	stdout.flush().unwrap();
}
fn generate_move(
    moves: &mut Vec<(usize,usize)>,
    grid: &[Vec<char>],
    x: usize,
    y: usize,
	passages: &HashMap<(usize,usize),(usize,usize)>,
) {

	/* Trying to move to (x,y) */
    let goal = grid[x][y];

	/* Do not move into a wall*/
    if  goal == '#' {
        return;
    }
	
	/* Special case for start */
    if  goal.is_ascii_uppercase() {
        return;
    }

	/* If it is in the location of a portal, set x,y to the new position */
	let mut x = x;
	let mut y = y;
	
	if let Some((new_x,new_y)) = passages.get(&(x,y))
	{
		x = *new_x;
		y = *new_y;
	}
  
	/* If we arrive so far, this is a valid move */
    moves.push((x, y))
}

fn path(grid:&Vec<Vec<char>>, passages: HashMap<(usize,usize),(usize,usize)>, start_position: (usize,usize), goal_position: (usize,usize)) -> i32 {


	/* BFS search usind pathfinding crate.*/
    let start = start_position;
    let result = pathfinding::directed::bfs::bfs(
        &start,
        |(x, y)| {
            let mut moves = Vec::new();
            generate_move(&mut moves, grid, *x, *y-1, &passages);
            generate_move(&mut moves, grid, *x, *y+1, &passages);
            generate_move(&mut moves, grid, *x-1, *y,  &passages);
            generate_move(&mut moves, grid, *x+1, *y,  &passages);

            moves
        },
        |(x, y)| (*x,*y) == goal_position,
    );

    let result = result.unwrap();

	let mut jumps = 0;
	for p in &result
	{
		if passages.contains_key(&p)
		{
			jumps += 1;
		}
	}
    result.len() as i32 - 1 +jumps
}

fn generate_move2(
    moves: &mut Vec<((usize,usize), usize)>,
    grid: &[Vec<char>],
    x: usize,
    y: usize,
	depth: usize,
	passages: &HashMap<(usize,usize),(usize,usize)>,
) {

	/* Trying to move to (x,y) */
    let goal = grid[x][y];

	/* Do not move into a wall*/
    if  goal == '#' {
        return;
    }
	
	/* Special case for start */
    if  goal.is_ascii_uppercase() {
        return;
    }

	/* If it is in the location of a portal, set x,y to the new position */
	let mut x = x;
	let mut y = y;
	let mut depth = depth;
	
	if let Some((new_x,new_y)) = passages.get(&(x,y))
	{
		/* Check if it is an outer or inner portal */
		let is_outer = x==2 || y == 2 || x==grid.len()-3 || y==grid[x].len()-3;
		
		
		/* If it is outer, check that depth !=0 */
		if is_outer && depth !=0
		{
			x = *new_x;
			y = *new_y;
			depth -= 1;
		}
		else if !is_outer
		{
			x = *new_x;
			y = *new_y;
			depth+=1;
		}
		
	}
  
	/* If we arrive so far, this is a valid move */
    moves.push(((x, y), depth))
}


fn path2(grid:&Vec<Vec<char>>, passages: HashMap<(usize,usize),(usize,usize)>, start_position: (usize,usize), goal_position: (usize,usize)) -> i32 {


	/* BFS search usind pathfinding crate.*/
    let start = start_position;
    let result = pathfinding::directed::bfs::bfs(
        &(start, 0),
        |((x, y),d)| {
            let mut moves = Vec::new();
            generate_move2(&mut moves, grid, *x, *y-1, *d, &passages);
            generate_move2(&mut moves, grid, *x, *y+1, *d, &passages);
            generate_move2(&mut moves, grid, *x-1, *y, *d, &passages);
            generate_move2(&mut moves, grid, *x+1, *y, *d, &passages);

            moves
        },
        |((x, y),d)| (*x,*y) == goal_position && *d==0,
    );

    let result = result.unwrap();

	let mut jumps = 0;
	print_grid(grid,start_position,0);
	let mut old_p = start_position;
	let mut stdout = stdout();
	for p in result.iter().skip(1)
	{	
		
		print_grid(grid,p.0, p.1);
        thread::sleep(Duration::from_millis(100));
		if passages.contains_key(&(p.0))
		{
			jumps += 1;
		}
	}
    result.len() as i32 - 1 +jumps
}

fn main() {
	let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n') {
        input.truncate(input.len() - 1);
    }

    let mut grid: Vec<Vec<char>> = input
        .split('\n')
        .map(|x| x.trim_end().chars().collect::<Vec<char>>())
        .collect();
	
	
	let mut portals : HashMap<String,Vec<(usize,usize)>>= HashMap::new();
	
	for (i,r) in grid.iter().enumerate()
	{
		for (j, val) in r.iter().enumerate()
		{
			if *val =='.'
			{
				if grid[i-1][j].is_ascii_uppercase()
				{
					let mut name = String::new();
					name.push(grid[i-2][j]);
					name.push(grid[i-1][j]);
					portals.entry(name).and_modify(|x| x.push((i,j))).or_insert(vec![(i,j)]);
				}
				if grid[i+1][j].is_ascii_uppercase()
				{
					let mut name = String::new();
					name.push(grid[i+1][j]);
					name.push(grid[i+2][j]);
					portals.entry(name).and_modify(|x| x.push((i,j))).or_insert(vec![(i,j)]);
				}
				if grid[i][j-1].is_ascii_uppercase()
				{
					let mut name = String::new();
					name.push(grid[i][j-2]);
					name.push(grid[i][j-1]);
					portals.entry(name).and_modify(|x| x.push((i,j))).or_insert(vec![(i,j)]);
				}
				if grid[i][j+1].is_ascii_uppercase()
				{
					let mut name = String::new();
					name.push(grid[i][j+1]);
					name.push(grid[i][j+2]);
					portals.entry(name).and_modify(|x| x.push((i,j))).or_insert(vec![(i,j)]);
				}
			}
		}
	}
	
	let mut goal_position = (0,0);
	let mut start_position = (0,0);
	let mut passages = HashMap::new();
	for (k,v) in portals
	{
		match &*k
		{
			"AA" => start_position = v[0],
			"ZZ" => goal_position = v[0],
			_ => 
			{
				passages.insert(v[0],v[1]);
				passages.insert(v[1],v[0]);
			}
		}
	}
	
    println!("Path: {}", path(&grid.clone(), passages.clone(), start_position, goal_position));
    println!("Path: {}", path2(&grid.clone(), passages.clone(), start_position, goal_position));
}
