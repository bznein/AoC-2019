use core::usize::MAX;
use crossterm::{cursor::*, execute, style::*, terminal::*, ExecutableCommand};
use factorial::Factorial;
use pathfinding::prelude::bfs;
use permutohedron::LexicalPermutation;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};
use std::io::{stdout, Write};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn successors(&self, grid: &[Vec<char>]) -> Vec<Pos> {
        let &Pos(x, y) = self;
        let mut v = Vec::new();
        if x > 0 && grid[x - 1][y] != '#' {
            v.push(Pos(x - 1, y));
        }
        if y > 0 && grid[x][y - 1] != '#' {
            v.push(Pos(x, y - 1));
        }
        if x < grid.len() - 1 && grid[x + 1][y] != '#' {
            v.push(Pos(x + 1, y));
        }
        if y < grid[x].len() - 1 && grid[x][y + 1] != '#' {
            v.push(Pos(x, y + 1));
        }
        v
    }
}

fn print_grid(grid: &[Vec<char>]) {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0), Hide, Clear(ClearType::All));
    println!("");
    println!("");
    for row in grid {
        for column in row {
            if column.is_ascii_lowercase() {
                execute!(stdout, SetForegroundColor(Color::Green));
            } else if column.is_ascii_uppercase() {
                execute!(stdout, SetForegroundColor(Color::Red));
            } else if *column == '@' {
                execute!(stdout, SetForegroundColor(Color::Yellow));
            } else {
                execute!(stdout, SetForegroundColor(Color::Reset));
            }
            print!("{}", column);
        }
        println!("");
    }
}

fn generate_move(
    moves: &mut Vec<(usize, usize, Vec<char>)>,
    grid: &[Vec<char>],
    x: usize,
    y: usize,
    keys: &Vec<char>,
) {
	/* Trying to move to (x,y) */
    let goal = grid[x][y];

	/* Do not move into a wall*/
    if  goal == '#' {
        return;
    }
	
	/* If it is a door, do not pass 
	 * if we don't have the key */
    if goal.is_ascii_uppercase()
        && !keys.contains(&goal.to_ascii_lowercase())
    {
        return;
    }

    let mut keys = keys.clone();

	/* If it is a key, add it to our keys */
    if goal.is_ascii_lowercase() {
        keys.push(goal);
		/* Avoid having multiple copies of keys */
		keys.sort();
        keys.dedup();
    }
	/* If we arrive so far, this is a valid move */
    moves.push((x, y, keys))
}

fn path(grid: &[Vec<char>], doors: &HashSet<char>) -> i32 {

	let mut x=0;
	let mut	y=0;
	/* Find the starting position */
	for (i,ival) in grid.iter().enumerate()
	{
		for (j, ijval) in grid[i].iter().enumerate()
		{
			if *ijval == '@'
			{
				x=i;
				y=j;
			}
		}
	}

	/* BFS search usind pathfinding crate.
	 * Stop criteria is having all the keys */
    let start: (usize, usize, Vec<char>) = (x, y, Vec::new());
    let result = pathfinding::directed::bfs::bfs(
        &start,
        |(x, y, keys)| {
            let mut moves = Vec::new();
            generate_move(&mut moves, grid, *x, *y-1, &keys);
            generate_move(&mut moves, grid, *x, *y+1, &keys);
            generate_move(&mut moves, grid, *x-1, *y,  &keys);
            generate_move(&mut moves, grid, *x+1, *y,  &keys);

            moves
        },
        |(x, y, keys)| keys.len() == doors.len(),
    );

    let result = result.unwrap();

    result.len() as i32 - 1
}

#[derive(Debug,Eq,PartialEq,Hash, Clone)]
struct Position
{
	x: [usize;4],
	y: [usize;4],
	keys: Vec<char>,
	bot: Option<usize>
}

impl Position
{
	fn generate_move(&self,  moves: &mut Vec<Position>,
    grid: &Vec<Vec<char>>,
	bot: usize,
    dx: i32,
    dy: i32) 
	{
	
		/* Trying to move to (x,y) */
		let x = self.x[bot] + dx as usize;
		let y = self.y[bot] + dy as usize;
		let goal = grid[x][y];

		/* If we have a current bot and is not bot,
		return */
		if self.bot.is_some()
		{
			if bot != self.bot.unwrap()
			{
				return;
			}
		}
		
		let mut cur_bot = Some(bot);

		/* Do not move into a wall*/
		if  goal == '#' {
			return;
		}
		
		/* If it is a door, do not pass 
		 * if we don't have the key */
		if goal.is_ascii_uppercase()
			&& !self.keys.contains(&goal.to_ascii_lowercase())
		{
			return;
		}

		let mut keys = self.keys.clone();

		/* If it is a key, add it to our keys */
		if goal.is_ascii_lowercase() {
			keys.push(goal);
			/* Avoid having multiple copies of keys */
			keys.sort();
			keys.dedup();
			/* Deactivate the bot */
			cur_bot = None;
		}
		/* If we arrive so far, this is a valid move */
		
		/* Update position of the current bot */   
		let mut new_x = self.x.clone();
        let mut new_y = self.y.clone();
        new_x[bot] = x;
        new_y[bot] = y;
        moves.push(
            Position {
                x:new_x,y:new_y,keys,
                bot: cur_bot
            }
        )
	}
}


fn path2(grid: & mut Vec<Vec<char>>, doors: &HashSet<char>) -> i32 {

	let mut x=0;
	let mut	y=0;
	/* Find the starting position */
	for (i,ival) in grid.iter().enumerate()
	{
		for (j, ijval) in grid[i].iter().enumerate()
		{
			if *ijval == '@'
			{
				x=i;
				y=j;
			}
		}
	}
	
	let mut grid = grid.clone();
	grid[x][y] = '#';
    grid[x-1][y] = '#';
    grid[x+1][y] = '#';
    grid[x][y-1] = '#';
    grid[x][y+1] = '#';

	/* BFS search usind pathfinding crate.
	 * Stop criteria is having all the keys */
    let start = Position {x :[x-1,x-1,x+1,x+1], y: [y-1,y+1,y-1,y+1], keys : Vec::new(), bot : None};
    let result = pathfinding::directed::bfs::bfs(
        &start,
        |pos| {
            let mut moves = Vec::new();
			for bot in 0..4
			{
            pos.generate_move(&mut moves, &grid, bot, 0, -1);
            pos.generate_move(&mut moves, &grid, bot, 0, 1 );
            pos.generate_move(&mut moves, &grid, bot, -1, 0 );
            pos.generate_move(&mut moves, &grid, bot,  1, 0);
			}

            moves
        },
        |pos| pos.keys.len() == doors.len(),
    );

    let result = result.unwrap();

    result.len() as i32 - 1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n') {
        input.truncate(input.len() - 1);
    }

    let mut grid: Vec<Vec<char>> = input
        .split_whitespace()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let doors: HashSet<char> = input
        .chars()
        .filter(|x| x.is_alphabetic())
        .map(|x| x.to_ascii_uppercase())
        .collect();

    println!("Path: {}", path(&grid, &doors));
    println!("Path: {}", path2(& mut grid, &doors));
	
	
}
