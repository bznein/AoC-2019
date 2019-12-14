use std::io::{self, Read};
use std::time::Duration;
use std::thread;
use core::f64::MAX;
use std::io::{Write, stdout};
use crossterm::{terminal::*,style::*,execute,ExecutableCommand, cursor::*};

fn print_grid(grid: &[Vec<char>], asteroid_to_destroy: Option<(usize,usize)>)
{
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0),Hide);
    for (idx_i,i) in grid.iter().enumerate()
    {
        for (idx_j,j) in i.iter().enumerate()
        {
            match j
            {
                'x' =>
                {
                    execute!(stdout,SetForegroundColor(Color::Green));
                    print!("X");
                }
                '.' =>
                {
                    execute!(stdout,SetForegroundColor(Color::Black));
                    print!(".");
                }
                '#' =>
                {
                    if (asteroid_to_destroy.is_some() && asteroid_to_destroy.unwrap()==(idx_i,idx_j))
                    {
                        execute!(stdout,SetForegroundColor(Color::Red));
                    }
                    else {
                        execute!(stdout,SetForegroundColor(Color::Blue));
                    }
                    print!("#");
                }
                _ => panic!("Unknown character")
            }
        }
        println!("");
    }
}

fn distance(from: (usize, usize), to: (usize, usize)) -> f64 {
    let from = (from.0 as isize, from.1 as isize);
    let to = (to.0 as isize, to.1 as isize);
    let d1: f64 = (from.0 - to.0) as f64;
    let d2: f64 = (from.1 - to.1) as f64;
    (d1.powi(2) + d2.powi(2)).sqrt()
}

fn is_visible(grid: &[Vec<char>], from: (usize, usize), to: (usize, usize)) -> bool {
    let distance_val = distance(from, to);
    for (i, item) in grid.iter().enumerate() {
        for (j, item_j) in item.iter().enumerate() {
            if (i, j) == from || (i, j) == to || *item_j == '.' {
                continue;
            }
            if (distance((i, j), from) + distance((i, j), to) - distance_val).abs() < 0.0001 {
                return false;
            }
        }
    }
    true
}

fn visible_asteroids(grid: &[Vec<char>], i: usize, j: usize) -> i32 {
    let mut tot = 0;
    for (ii, item) in grid.iter().enumerate() {
        for (jj, item_j) in item.iter().enumerate() {
            if (i == ii && j == jj) || *item_j == '.' {
                continue;
            }
            /* Got to an asteroid, check if visibile */
            if is_visible(&grid, (i, j), (ii, jj)) {
                tot += 1;
            }
        }
    }
    tot
}

fn clockwise_angle(c: (usize, usize), a: (usize, usize), b: (usize, usize)) -> f64 {
    let c = (c.0 as f64, c.1 as f64);
    let a = (a.0 as f64, a.1 as f64);
    let b = (b.0 as f64, b.1 as f64);

    let dir_c_to_a = (a.1 - c.1).atan2(a.0 - c.0);
    let dir_c_to_b = (b.1 - c.1).atan2(b.0 - c.0);
    let mut angle_acb = dir_c_to_a - dir_c_to_b;

    let pi = core::f64::consts::PI;

    if angle_acb < 0.0 {
        angle_acb += 2.0 * pi;
    }
    angle_acb
}

fn closest_right_side(
    grid: &[Vec<char>],
    line: ((usize, usize), (usize, usize)),
) -> (usize, usize) {
    let mut min_dist = MAX;
    let mut closest = (grid.len(), grid.len());
    let mut candidate_single = (grid.len(), grid.len());
    for (i, item) in grid.iter().enumerate() {
        for (j, item_j) in item.iter().enumerate() {
            if *item_j == '#' {
                let dist = clockwise_angle(line.0, line.1, (i, j));
                if (dist != 0.0 && dist < min_dist)
                    || (dist == min_dist && distance((i, j), line.0) < distance(closest, line.0))
                {
                    min_dist = dist;
                    closest = (i, j);
                }
                else if (dist == 0.0)
                {
                    candidate_single=(i,j)
                }
            }
        }
    }
    if closest != (grid.len(), grid.len())
    {
        closest
    }
    else if candidate_single != (grid.len(), grid.len())
    {
        candidate_single
    }
    else
    {
        panic!("No asteroid found!");
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n') {
        input.truncate(input.len() - 1);
    }

    let grid: Vec<Vec<char>> = input
        .split_whitespace()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let mut max_asteroids = 0;
    let mut tot_asteroids = 0;
    let mut best_pos = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '#' {
                tot_asteroids += 1;
                let asteroids = visible_asteroids(&grid, i, j);
                if asteroids > max_asteroids {
                    max_asteroids = asteroids;
                    best_pos = (i, j);
                }
            }
        }
    }
    println!("Part 1: {}", max_asteroids);
    let mut stdout = stdout();
    execute!(stdout,Clear(ClearType::All));

    let mut grid = grid;
    let best_pos = best_pos;
    grid[best_pos.0][best_pos.1] = 'x';
    print_grid(&grid,None);

    let mut asteroids_pulverized = 0;

    let mut current_asteroid = (0, 0);
    let mut pulverizing_line: ((usize, usize), (usize, usize)) = ((0, 0), (0, 0));
    for i in (0..best_pos.0).rev() {
        if grid[i][best_pos.1] == '#' {
            current_asteroid = (i, best_pos.1);
            pulverizing_line = (best_pos, current_asteroid);
            asteroids_pulverized += 1;
            grid[i][best_pos.1] = '.';
            break;
        }
        print_grid(&grid,None);
    }

    let mut part_two_result = 0;
    for _i in 1..tot_asteroids-1
    {
        current_asteroid = closest_right_side(&grid, pulverizing_line);
        pulverizing_line = (best_pos, current_asteroid);
        asteroids_pulverized += 1;
        print_grid(&grid,Some(current_asteroid));
        thread::sleep(Duration::from_millis(50));
        grid[current_asteroid.0][current_asteroid.1] = '.';
        print_grid(&grid,None);
        thread::sleep(Duration::from_millis(50));
        if asteroids_pulverized == 200
        {
            part_two_result=current_asteroid.1 * 100 + current_asteroid.0;
        }
    }
    println!("Part 2: {} ", part_two_result);
}
