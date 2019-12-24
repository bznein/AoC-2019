use core::usize::MAX;
use crossterm::{cursor::*, execute, style::*, terminal::*, ExecutableCommand};
use pathfinding::prelude::bfs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

#[macro_use]
extern crate maplit;

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

fn print_grid(
    grid: &[Vec<char>],
    pos: (usize, usize),
    colormap: &HashMap<char, Color>,
    keys: &Vec<char>,
    steps: usize,
) {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0), Hide, Clear(ClearType::All));
    let modifier: i32 = 15;
    let mut min_i = pos.0 as i32 - modifier;
    let mut min_j = pos.1 as i32 - modifier;
    let mut max_i = pos.0 as i32 + modifier;
    let mut max_j = pos.1 as i32 + modifier;
    if min_i < 0 {
        max_i -= min_i;
        min_i = 0;
    }
    if min_j < 0 {
        max_j -= min_j;
        min_j = 0;
    }
    if max_i >= grid.len() as i32 {
        max_i = grid.len() as i32 - 1;
    }
    if max_j >= grid[0].len() as i32 {
        max_j = grid[0].len() as i32 - 1;
    }
    for i in min_i..=max_i {
        for j in min_j..=max_j {
            let i = i as usize;
            let j = j as usize;
            let column = grid[i][j];
            if column.is_ascii_lowercase() {
                execute!(
                    stdout,
                    SetForegroundColor(*colormap.get(&column.to_ascii_uppercase()).unwrap())
                );
                print!("🔑");
            } else if column.is_ascii_uppercase() {
                execute!(stdout, SetForegroundColor(*colormap.get(&column).unwrap()));
                print!("█");
            } else if column == '@' {
                execute!(stdout, SetForegroundColor(Color::Red));
                print!("@");
            } else if column == '#' {
                execute!(stdout, SetForegroundColor(Color::Reset));
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    execute!(stdout, MoveTo(35, 12));
    execute!(stdout, SetForegroundColor(Color::Reset));
    print!("STEPS: {}", steps);
    execute!(stdout, MoveTo(35, 10));
    execute!(stdout, SetForegroundColor(Color::Reset));
    print!("KEYS: ");
    for k in keys.iter().filter(|x| **x != '.') {
        execute!(stdout, SetForegroundColor(*colormap.get(k).unwrap()));
        print!("⚩ ");
    }
    stdout.flush();
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
    if goal == '#' {
        return;
    }

    /* If it is a door, do not pass
     * if we don't have the key */
    if goal.is_ascii_uppercase() && !keys.contains(&goal.to_ascii_lowercase()) {
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

fn path(grid: &mut Vec<Vec<char>>, doors: &HashSet<char>, colormap: &HashMap<char, Color>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    /* Find the starting position */
    for (i, ival) in grid.iter().enumerate() {
        for (j, ijval) in grid[i].iter().enumerate() {
            if *ijval == '@' {
                x = i;
                y = j;
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
            generate_move(&mut moves, grid, *x, *y - 1, &keys);
            generate_move(&mut moves, grid, *x, *y + 1, &keys);
            generate_move(&mut moves, grid, *x - 1, *y, &keys);
            generate_move(&mut moves, grid, *x + 1, *y, &keys);

            moves
        },
        |(x, y, keys)| keys.len() == doors.len(),
    );

    let result = result.unwrap();
/*
    let mut old_p = &result[0];
    let mut stdout = stdout();
    let mut keys = vec!['.'; 26];
    let mut steps = 0;
    for p in result.iter().skip(1) {
        thread::sleep(Duration::from_millis(100));
        print_grid(grid, (old_p.0, old_p.1), colormap, &keys, steps);
        grid[old_p.0][old_p.1] = '.';

        old_p = p;
        let c = grid[old_p.0][old_p.1];
        if c.is_ascii_lowercase() {
            let c = c.to_ascii_uppercase();
            keys[c as usize - 'A' as usize] = c;
        } else if c.is_ascii_uppercase() {
            keys[c as usize - 'A' as usize] = '.';
        }
        grid[old_p.0][old_p.1] = '@';
        steps += 1;
    }*/

    result.len() as i32 - 1
}

fn print_grid2(
    grid: &[Vec<char>],
    colormap: &HashMap<char, Color>,
    robot: usize,
    keys: &Vec<char>,
    steps: usize,
) {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0), Hide, Clear(ClearType::All));
    let mut min_i = grid.len() / 2 * (robot % 2);
    let mut min_j = if robot < 2 { 0 } else { grid[0].len() / 2 };
    let len_i = grid.len() / 2;
    let len_j = grid[0].len() / 2;
    execute!(stdout, SetForegroundColor(Color::Red));
    println!("Robot #{} camera feed", robot);
    for i in 0..=len_i {
        for j in 0..=len_j {
            let i = i + min_i as usize;
            let j = j + min_j as usize;
            let column = grid[i][j];
            if column.is_ascii_lowercase() {
                execute!(
                    stdout,
                    SetForegroundColor(*colormap.get(&column.to_ascii_uppercase()).unwrap())
                );
                print!("⚩");
            } else if column.is_ascii_uppercase() {
                execute!(stdout, SetForegroundColor(*colormap.get(&column).unwrap()));
                print!("█");
            } else if column == '@' {
                execute!(stdout, SetForegroundColor(Color::Red));
                print!("@");
            } else if column == '#' {
                execute!(stdout, SetForegroundColor(Color::Reset));
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    execute!(stdout, MoveTo(43, 12));
    execute!(stdout, SetForegroundColor(Color::Reset));
    print!("STEPS: {}", steps);
    execute!(stdout, MoveTo(43, 10));
    execute!(stdout, SetForegroundColor(Color::Reset));
    print!("KEYS: ");
    for k in keys.iter() {
        execute!(
            stdout,
            SetForegroundColor(*colormap.get(&k.to_ascii_uppercase()).unwrap())
        );
        print!("⚩ ");
    }
    stdout.flush();
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Position {
    x: [usize; 4],
    y: [usize; 4],
    keys: Vec<char>,
    bot: Option<usize>,
}

impl Position {
    fn generate_move(
        &self,
        moves: &mut Vec<Position>,
        grid: &Vec<Vec<char>>,
        bot: usize,
        dx: i32,
        dy: i32,
    ) {
        /* Trying to move to (x,y) */
        let x = self.x[bot] + dx as usize;
        let y = self.y[bot] + dy as usize;
        let goal = grid[x][y];

        /* If we have a current bot and is not bot,
        return */
        if self.bot.is_some() {
            if bot != self.bot.unwrap() {
                return;
            }
        }

        let mut cur_bot = Some(bot);

        /* Do not move into a wall*/
        if goal == '#' {
            return;
        }

        /* If it is a door, do not pass
         * if we don't have the key */
        if goal.is_ascii_uppercase() && !self.keys.contains(&goal.to_ascii_lowercase()) {
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
        moves.push(Position {
            x: new_x,
            y: new_y,
            keys,
            bot: cur_bot,
        })
    }
}

fn path2(grid: &mut Vec<Vec<char>>, doors: &HashSet<char>, colormap: &HashMap<char, Color>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    /* Find the starting position */
    for (i, ival) in grid.iter().enumerate() {
        for (j, ijval) in grid[i].iter().enumerate() {
            if *ijval == '@' {
                x = i;
                y = j;
            }
        }
    }

    let mut grid = grid.clone();
    grid[x][y] = '#';
    grid[x - 1][y] = '#';
    grid[x + 1][y] = '#';
    grid[x][y - 1] = '#';
    grid[x][y + 1] = '#';

    /* BFS search usind pathfinding crate.
     * Stop criteria is having all the keys */
    let start = Position {
        x: [x - 1, x + 1, x - 1, x + 1],
        y: [y - 1, y - 1, y + 1, y + 1],
        keys: Vec::new(),
        bot: None,
    };
    let result = pathfinding::directed::bfs::bfs(
        &start,
        |pos| {
            let mut moves = Vec::new();
            for bot in 0..4 {
                pos.generate_move(&mut moves, &grid, bot, 0, -1);
                pos.generate_move(&mut moves, &grid, bot, 0, 1);
                pos.generate_move(&mut moves, &grid, bot, -1, 0);
                pos.generate_move(&mut moves, &grid, bot, 1, 0);
            }

            moves
        },
        |pos| pos.keys.len() == doors.len(),
    );

    let result = result.unwrap();
   /* let mut old_p = &result[0];
    let mut stdout = stdout();
    let mut steps = 0;
    for p in result.iter().skip(1) {
        let mut bot_p = 0;
        thread::sleep(Duration::from_millis(75));
        if let Some(bot) = p.bot {
            grid[old_p.x[bot]][old_p.y[bot]] = '.';
            grid[p.x[bot]][p.y[bot]] = '@';
            bot_p = bot;
        } else if let Some(bot) = old_p.bot {
            grid[old_p.x[bot]][old_p.y[bot]] = '.';
            grid[p.x[bot]][p.y[bot]] = '@';
            bot_p = bot;
        }
        steps += 1;
        print_grid2(&grid, colormap, bot_p, &p.keys, steps);
        old_p = p;
    }*/

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

   let colormap = hashmap! {
        'A' => Color::Rgb{r: 255, g: 255, b: 128},
        'B' => Color::Rgb{r: 240, g: 163, b: 255},
        'C' => Color::Rgb{r: 0, g: 117, b: 220},
        'D' => Color::Rgb{r: 153, g: 63, b: 0},
        'E' => Color::Rgb{r: 76, g: 0, b: 92},
        'F' => Color::Rgb{r: 255, g: 25, b: 25},
        'G' => Color::Rgb{r: 0, g: 92, b: 49},
        'H' => Color::Rgb{r: 43, g: 206, b: 72},
        'I' => Color::Rgb{r: 255, g: 204, b: 153},
        'J' => Color::Rgb{r: 128, g: 128, b: 128},
        'K' => Color::Rgb{r: 148, g: 255, b: 181},
        'L' => Color::Rgb{r: 143, g: 124, b: 0},
        'M' => Color::Rgb{r: 157, g: 204, b: 0},
        'N' => Color::Rgb{r: 194, g: 0, b: 136},
        'O' => Color::Rgb{r: 0, g: 51, b: 128},
        'P' => Color::Rgb{r: 255, g: 164, b: 5},
        'Q' => Color::Rgb{r: 255, g: 168, b: 187},
        'R' => Color::Rgb{r: 66, g: 102, b: 0},
        'S' => Color::Rgb{r: 255, g: 0, b: 16},
        'T' => Color::Rgb{r: 94, g: 241, b: 242},
        'U' => Color::Rgb{r: 0, g: 153, b: 143},
        'V' => Color::Rgb{r: 224, g: 255, b: 102},
        'W' => Color::Rgb{r: 116, g: 10, b: 255},
        'X' => Color::Rgb{r: 153, g: 0, b: 0},
        'Y' => Color::Rgb{r: 255, g: 255, b: 0},
        'Z' => Color::Rgb{r: 255, g: 80, b: 5},
    };

    println!("Path: {}", path(& mut grid.clone(), &doors, &colormap));
    println!("Path: {}", path2(&mut grid, &doors, &colormap));
}
