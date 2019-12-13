use intcode::IntcodeMachine;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::io::{self, Read};
use intcode::State::*;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum MoveDirection {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Point {
    i: i32,
    j: i32,
    move_dir: MoveDirection,
}

use crate::MoveDirection::*;

fn move_direction(m: MoveDirection, dir: i32) -> MoveDirection {
    match m {
        UP => {
            if dir == 0 {
                LEFT
            } else {
                RIGHT
            }
        }
        RIGHT => {
            if dir == 0 {
                UP
            } else {
                DOWN
            }
        }
        DOWN => {
            if dir == 0 {
                RIGHT
            } else {
                LEFT
            }
        }
        LEFT => {
            if dir == 0 {
                DOWN
            } else {
                UP
            }
        }
    }
}

impl Point {
    pub fn new() -> Point {
        Point {
            i: 0,
            j: 0,
            move_dir: UP,
        }
    }
    fn move_point(&mut self, dir: i32) {
        self.move_dir = move_direction(self.move_dir, dir);
        match self.move_dir {
            UP => self.i -= 1,
            DOWN => self.i += 1,
            RIGHT => self.j += 1,
            LEFT => self.j -= 1,
        }
    }
}

fn print_map(min_i: i32, min_j: i32, max_i: i32, max_j: i32, m: HashMap<(i32, i32), i64>) {
    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if m.contains_key(&(i, j)) {
                print!(
                    "{}",
                    if *m.get(&(i, j)).unwrap() == 1 {
                        "█"
                    } else {
                        " "
                    }
                );
            } else {
                print!(" ");
            }
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

    let v: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut executor = IntcodeMachine::new(v.clone());

    let mut m = HashMap::new();
    let mut p = Point::new();

    loop {
        if !m.contains_key(&(p.i, p.j)) {
            m.insert((p.i, p.j), 0);
        }
        executor.set_input(*m.get(&(p.i, p.j)).unwrap());
        executor.run();
        if executor.state()==Halted {
            println!("Part 1: {}", m.len());
            break;
        }
        let new_val = executor.get_output().unwrap();
        m.insert((p.i, p.j), new_val);
        executor.run();
        let move_val = executor.get_output().unwrap();
        p.move_point(move_val as i32);
    }

    let mut executor = IntcodeMachine::new(v.clone());

    let mut m = HashMap::new();
    let mut p = Point::new();

    let mut min_i = 0;
    let mut min_j = 0;
    let mut max_i = 0;
    let mut max_j = 0;
    executor.set_input(1);
    executor.run();


    let new_val = executor.get_output().unwrap();
    m.insert((p.i, p.j), new_val);
    executor.run();
    let move_val = executor.get_output().unwrap();
    p.move_point(move_val as i32);
    loop {
        min_i = min(min_i, p.i);
        min_j = min(min_j, p.j);
        max_i = max(max_i, p.i);
        max_j = max(max_j, p.j);
        if !m.contains_key(&(p.i, p.j)) {
            m.insert((p.i, p.j), 0);
        }
        executor.set_input(*m.get(&(p.i, p.j)).unwrap());
        executor.run();
        if executor.state()==Halted {
            println!("Part 2");
            print_map(min_i, min_j, max_i, max_j, m);
            break;
        }
        let new_val = executor.get_output().unwrap();
        m.insert((p.i, p.j), new_val);
        executor.run();
        let move_val = executor.get_output().unwrap();
        p.move_point(move_val as i32);
    }
}
