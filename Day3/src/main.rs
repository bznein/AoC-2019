use num::abs;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Corner
{
    wire_1 : i32,
    wire_2 : i32
}

fn cost(c: &Corner) -> i32 {
    c.wire_1+c.wire_2
}

fn manhattan(p1: &Point, p2: &Point) -> i32 {
    abs(p1.x - p2.x) + abs(p1.y - p2.y)
}


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n')
    {
        input.truncate(input.len()-1);
    }
  
    let v: Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
    let wire_1: Vec<String> = v[0].split(',').map(|x| x.to_string()).collect();
    let wire_2: Vec<String> = v[1].split(',').map(|x| x.to_string()).collect();

    let mut grid = HashMap::new();
    let origin = Point { x: 0, y: 0 };

    let mut p1 = Point { x: 0, y: 0 };
    let mut steps = 0;
    for i in 0..wire_1.len() {
        let temp : i32 = (&wire_1[i][1..]).parse::<i32>().unwrap();
        let start_x = p1.x;
        let start_y = p1.y;
        match &wire_1[i][0..1] {
            "R" => {
                for j in start_x+1..=start_x +temp {
                    p1.x = j;
                    steps+=1;
                    grid.insert(p1,Corner{wire_1 : steps, wire_2:0});
                }
            }
            "U" => {
                for j in start_y+1..=start_y + temp {
                    p1.y = j;
                    steps+=1;
                    grid.insert(p1,Corner{wire_1 : steps, wire_2:0});
                }
            }
            "L" => {
                for j in start_x-temp..start_x {
                    p1.x = j;
                    steps+=1;
                    grid.insert(p1,Corner{wire_1 : steps, wire_2:0});
                }
                p1.x = start_x-temp;
            }
            "D" => {
                for j in start_y-temp..start_y {
                    p1.y = j;
                    steps+=1;
                    grid.insert(p1,Corner{wire_1 : steps, wire_2:0});
                }
                p1.y = start_y-temp;
            }
            _ => ()
        }
    }
    
    p1 = Point { x: 0, y: 0 };
    steps=0;
    for i in 0..wire_2.len() {
        let temp : i32 = (&wire_2[i][1..]).parse::<i32>().unwrap();
        let start_x = p1.x;
        let start_y = p1.y;
        match &wire_2[i][0..1] {
            "R" => {
                for j in start_x+1..=start_x +temp {
                    p1.x = j;
                    steps+=1;
                    grid.entry(p1).and_modify(|c| c.wire_2 = steps).or_insert(Corner{wire_1 : 0, wire_2:steps});
                }
            }
            "U" => {
                for j in start_y+1..=start_y + temp {
                    p1.y = j;
                    steps+=1;
                    grid.entry(p1).and_modify(|c| c.wire_2 = steps).or_insert(Corner{wire_1 : 0, wire_2:steps});
                }
            }
            "L" => {
                for j in start_x-temp..start_x {
                    p1.x = j;
                    steps+=1;
                    grid.entry(p1).and_modify(|c| c.wire_2 = steps).or_insert(Corner{wire_1 : 0, wire_2:steps});
                }
                p1.x = start_x-temp;
            }
            "D" => {
                for j in start_y-temp..start_y {
                    p1.y = j;
                    steps+=1;
                    grid.entry(p1).and_modify(|c| c.wire_2 = steps).or_insert(Corner{wire_1 : 0, wire_2:steps});
                }
                p1.y = start_y-temp;
            }
            _ => ()
        }
    }
    
    let mut min_cost: i32 = -1;
    let mut min_dist: i32 = -1;

    for (key,val) in grid
    {
        if val.wire_1>0 && val.wire_2>0{ 
            if min_cost == -1 || cost(&val)<min_cost
            {
                min_cost = cost(&val);
            }
			let cur_dist = manhattan(&key,&origin);
            if min_dist==-1 || min_dist>cur_dist
                {
                    min_dist = cur_dist;
                }
        }
    }
    println!("Part 1: {}", min_dist);
    println!("Part 2: {}", min_cost);
}
