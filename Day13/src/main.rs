use core::i64::MAX;
use core::i64::MIN;
use crossterm::{cursor::*, execute, style::*, terminal::*, ExecutableCommand};
use intcode::IntcodeMachine;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::io::stdout;
use std::io::Write;
use std::io::{self, Read};
use std::thread;
use std::time::Duration;

fn print_game(
    m: &HashMap<(i64, i64), i64>,
    (min_x, min_y, max_x, max_y): (i64, i64, i64, i64),
    score: Option<i64>,
) {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0), Hide);
    for i in min_y..=max_y {
        for j in min_x..=max_x {
            match m.get(&(j, i)).unwrap() {
                0 => print!(" "),
                1 => print!("{}", if i == min_y { "_" } else { "|" }),
                2 => {
                    execute!(stdout, SetForegroundColor(Color::Red));
                    print!("|");
                    execute!(stdout, SetForegroundColor(Color::Reset));
                }
                3 => print!("_"),
                4 => {
                    execute!(stdout, SetForegroundColor(Color::Green));
                    print!(".");
                    execute!(stdout, SetForegroundColor(Color::Reset));
                }
                _ => panic!("Error in print"),
            }
        }
        println!("");
    }
    match score {
        None => (),
        Some(x) => print!("Score: {}", x),
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

    let mut blocks = 0;
    let mut x;
    let mut y;
    let mut t_id = 0;
    let mut score = 0;
    let mut min_x: i64 = MAX;
    let mut min_y: i64 = MAX;
    let mut max_x: i64 = MIN;
    let mut max_y: i64 = MIN;
    loop {
        executor.run();
        let s = executor.get_output();
        match s {
            None => break,
            Some(v) => x = v,
        }

        executor.run();
        y = executor.get_output().unwrap();

        executor.run();
        t_id = executor.get_output().unwrap();

        m.insert((x, y), t_id);
        min_x = min(min_x, x);
        min_y = min(min_y, y);
        max_x = max(max_x, x);
        max_y = max(max_y, y);
        if t_id == 2 {
            blocks += 1;
        }
    }

    println!("Part 1: {}", blocks);
    let mut stdout = stdout();
//    execute!(stdout, Clear(ClearType::All));

    let mut v = v;
    let mut ball_x = 0;
    let mut pad_x = 0;
    let mut input = 0;
    v[0] = 2;
    let mut executor = IntcodeMachine::new(v.clone());
    loop {
        executor.set_input(input);
        executor.run();
        let s = executor.get_output();
        match s {
            None => break,
            Some(v) => x = v,
        }

        executor.set_input(input);
        executor.run();
        y = executor.get_output().unwrap();

        executor.set_input(input);
        executor.run();
        t_id = executor.get_output().unwrap();

        match t_id {
            3 => pad_x = x,
            4 => ball_x = x,
            _ => (),
        }
        input = if ball_x < pad_x {
            -1
        } else if ball_x > pad_x {
            1
        } else {
            0
        };

        if x == -1 {
            score = t_id;
        }

        m.insert((x, y), t_id);
        //print_game(&m, (min_x, min_y, max_x, max_y), Some(score));
        //thread::sleep(Duration::from_millis(1));
    }
    println!("Part 2: {}", score);
}
