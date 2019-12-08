use std::io::{self, Read};

use permutohedron::LexicalPermutation;

fn extract_digit(num: i32, div: i32) -> i32 {
    (num / div) % 10
}

fn get_parameter(v: &[i32], position: i32, digit: i32, instruction: i32) -> i32 {
    match extract_digit(instruction, digit) {
        1 => position,
        0 => v[position as usize],
        _ => 0,
    }
}

struct Amplifier {
    v: Vec<i32>,
    phase: i32,
    input: i32,
    i: usize,
}

impl Amplifier {
    fn run(&mut self) -> i32 {
        let mut times_input_requested = 0;
        while self.i < self.v.len() {
            let instruction = self.v[self.i];
            let op_code = self.v[self.i] % 100;
            if op_code == 99 {
                break;
            }
            let v1 = self.v[self.i + 1] as usize;
            let mut jumped = false;
            match op_code {
                1 => {
                    let v3 = self.v[self.i + 3] as usize;
                    self.v[v3] = get_parameter(&self.v, self.v[self.i + 1], 100, instruction)
                        + get_parameter(&self.v, self.v[self.i + 2], 1000, instruction)
                }
                2 => {
                    let v3 = self.v[self.i + 3] as usize;
                    self.v[v3] = get_parameter(&self.v, self.v[self.i + 1], 100, instruction)
                        * get_parameter(&self.v, self.v[self.i + 2], 1000, instruction)
                }
                3 => {
                    self.v[v1] = match times_input_requested {
                        0 => {
                            if (self.phase >= 0) {
                                self.phase
                            } else {
                                self.input
                            }
                        }
                        1 => self.input,
                        _ => -1,
                    };
                    times_input_requested = 1;
                    self.phase = -1;
                }
                4 => {
                    self.i += 2;
                    return get_parameter(&self.v, self.v[self.i - 1], 100, instruction);
                }
                5 => {
                    if get_parameter(&self.v, self.v[self.i + 1], 100, instruction) != 0 {
                        self.i =
                            get_parameter(&self.v, self.v[self.i + 2], 1000, instruction) as usize;
                        jumped = true;
                    }
                }
                6 => {
                    if get_parameter(&self.v, self.v[self.i + 1], 100, instruction) == 0 {
                        self.i =
                            get_parameter(&self.v, self.v[self.i + 2], 1000, instruction) as usize;
                        jumped = true;
                    }
                }
                7 => {
                    let v3 = self.v[self.i + 3] as usize;
                    self.v[v3] = (get_parameter(&self.v, self.v[self.i + 1], 100, instruction)
                        < get_parameter(&self.v, self.v[self.i + 2], 1000, instruction))
                        as i32;
                }
                8 => {
                    let v3 = self.v[self.i + 3] as usize;
                    self.v[v3] = (get_parameter(&self.v, self.v[self.i + 1], 100, instruction)
                        == get_parameter(&self.v, self.v[self.i + 2], 1000, instruction))
                        as i32;
                }
                99 => break,
                _ => (),
            }
            if !jumped {
                self.i += match op_code {
                    1 | 2 | 7 | 8 => 4,
                    3 | 4 => 2,
                    5 | 6 => 3,
                    _ => 0,
                };
            }
        }
        -1
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n') {
        input.truncate(input.len() - 1);
    }

    let v: Vec<i32> = input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut data = [0, 1, 2, 3, 4];
    let mut permutations = Vec::new();

    loop {
        permutations.push(data.to_vec());
        if !data.next_permutation() {
            break;
        }
    }

    let mut max_power: i32 = -1;
    let mut amps: Vec<Amplifier>;
    for values in permutations {
        amps = Vec::new();
        for i in 0..5 {
            amps.push(Amplifier {
                v: v.clone(),
                phase: values[i],
                input: 0,
                i: 0,
            });
        }

        let mut vv = amps[0].run();
        for i in 1..=4 {
            amps[i].input = vv;
            vv = amps[i].run();
        }
        if max_power < vv {
            max_power = vv;
        }
    }
    println!("Part 1: {}", max_power);

    let mut data = [5, 6, 7, 8, 9];
    let mut permutations = Vec::new();
    let mut max_signal = -1;

    loop {
        permutations.push(data.to_vec());
        if !data.next_permutation() {
            break;
        }
    }

    for values in permutations {
        println!("Values: {:?}", values);
        amps = Vec::new();
        for i in 0..5 {
            amps.push(Amplifier {
                v: v.clone(),
                phase: values[i],
                input: 0,
                i: 0,
            });
        }

        let mut input = 0;
        loop {
            amps[0].input = input;
            let mut vv = amps[0].run();
            if (vv == -1) {
                if input  > max_signal
                {
                    max_signal = input;
                }
                break;
            }
            for i in 1..=4 {
                amps[i].input = vv;
                vv = amps[i].run();
            }
            input = vv;
        }
    }

    println!("Part 2: {}", max_signal);
}
