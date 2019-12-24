use crossterm::{cursor::*, execute, style::*, terminal::*, ExecutableCommand};
use intcode::IntcodeMachine;
use std::collections::{HashMap, HashSet};
use std::io::stdout;
use std::io::Write;
use std::io::{self, Read};
extern crate text_io;
use std::thread;
use std::time::Duration;

fn print_space(m: &HashMap<(i32, i32), char>, d_position: (i32, i32)) {
    let min_x = -50;
    let max_x = 50;
    let min_y = -30;
    let max_y = 30;
    thread::sleep(Duration::from_millis(100));
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 1), Hide, Clear(ClearType::All));
    for i in min_y..=max_y {
        for j in min_x..=max_x {
            if (i, j) == d_position {
                print!("D");
            } else if m.contains_key(&(i, j)) {
                print!("{}", m.get(&(i, j)).unwrap());
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn fill(m: &mut HashMap<(i32, i32), char>, position: (i32, i32), steps: i32, most_steps: &mut i32) {
    m.insert(position, 'O');
    let mut print_steps: bool = true;
    for command in 1..=4 {
        let position = move_position(position, command);
        if m.contains_key(&position) && *m.get(&position).unwrap() == ' ' {
            // print_space(m,(-100,-100));
            fill(m, position, steps + 1, most_steps);
            print_steps = false;
        }
    }
    if print_steps && steps>*most_steps{
        *most_steps = steps;
    }
}

fn explore(
    m: &mut HashMap<(i32, i32), char>,
    visited: &mut HashSet<(i32, i32)>,
    position: (i32, i32),
    executor: IntcodeMachine,
    steps: i32,
) {
    visited.insert(position);
    let mut position = position;
    for command in 1..=4 {
        let mut exec = executor.clone();
        exec.set_input(command.into());
        exec.run();
        match exec.get_output().unwrap() {
            0 => {
                m.insert(move_position(position, command), '#');
            }
            1 => {
                let position = move_position(position, command);
                m.insert(position, ' ');
                if !visited.contains(&position) {
                    explore(m, visited, position, exec.clone(), steps + 1);
                }
            }
            2 => {
                m.insert(move_position(position, command), 'X');
                println!("Part 1: {}", steps + 1);
                //   println!("Coordinates: {:?}",move_position(position,command) );
            }
            _ => (),
        }
        // print_space(m,position);
    }
}

fn move_position(p: (i32, i32), command: i32) -> (i32, i32) {
    match command {
        1 => (p.0 - 1, p.1),
        2 => (p.0 + 1, p.1),
        3 => (p.0, p.1 + 1),
        4 => (p.0, p.1 - 1),
        _ => panic!("Unkwnown command"),
    }
}

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

    let mut visited = HashSet::new();

    m.insert((0, 0), 'S');
    explore(&mut m, &mut visited, (0, 0), executor.clone(), 0);
    // print_space(&m, (-1000,-1000));
    let mut steps=0;
    fill(&mut m, (14, -14), 0, &mut steps);
    println!("Part 2: {}", steps);
}
