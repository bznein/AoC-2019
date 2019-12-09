use std::io::{self, Read};

use permutohedron::LexicalPermutation;


struct IntcodeMachine {
    program: Vec<i64>,
    ip: usize,
    input: i64,
    relative_base: i64,
}

impl IntcodeMachine {
    pub fn new(program: Vec<i64>) -> IntcodeMachine {
        IntcodeMachine {
            program,
            ip: 0,
            relative_base: 0,
            input: 0,
        }
    }

    fn get_mode(&self, instruction: i64, place: i64) -> i64 {
        (instruction % place) / (place / 10)
    }

    fn read(&self, param: i64, mode: i64) -> i64 {
        match mode {
            0 => {
                if (param as usize) < self.program.len() {
                    self.program[param as usize]
                } else {
                    0
                }
            }
            1 => param,
            2 => {
                let new_pos = (param + self.relative_base) as usize;
                if new_pos < self.program.len() {
                    self.program[new_pos]
                } else {
                    0
                }
            }
            _ => panic!("Unsupported parameter mode: {}", mode),
        }
    }

    fn write(&mut self, param: i64, mode: i64, val: i64) {
        let address = match mode {
            0 => param as usize,
            2 => (param + self.relative_base) as usize,
            _ => panic!("Unsupported parameter mode: {}", mode),
        };

        if address >= self.program.len() {
            self.program.resize(address + 1, 0)
        }
        self.program[address] = val;
    }

    pub fn run(&mut self) {
        let mut ip: usize = self.ip;
        loop {
            let instruction = self.program[ip];
            let op_code = instruction % 100;
            let mode_1 = self.get_mode(instruction, 1000);
            let mode_2 = self.get_mode(instruction, 10_000);
            let mode_3 = self.get_mode(instruction, 100_000);
            match op_code {
                1 => {
                    let v1 = self.read(self.program[ip + 1], mode_1);
                    let v2 = self.read(self.program[ip + 2], mode_2);
                    self.write(self.program[ip + 3], mode_3, v1 + v2);
                    ip += 4;
                }
                2 => {
                    let v1 = self.read(self.program[ip + 1], mode_1);
                    let v2 = self.read(self.program[ip + 2], mode_2);
                    self.write(self.program[ip + 3], mode_3, v1 * v2);
                    ip += 4;
                }
                3 => {
                    self.write(self.program[ip + 1], mode_1, self.input);
                    ip += 2;
                }
                4 => {
                    println!("OUT: {}", self.read(self.program[ip + 1], mode_1));
                    ip += 2;
                }
                5 => {
                    let check = self.read(self.program[ip + 1], mode_1);
                    let val = self.read(self.program[ip + 2], mode_2);
                    if check != 0 {
                        ip = val as usize;
                    } else {
                        ip += 3;
                    }
                }
                6 => {
                    let check = self.read(self.program[ip + 1], mode_1);
                    let val = self.read(self.program[ip + 2], mode_2);
                    if check == 0 {
                        ip = val as usize;
                    } else {
                        ip += 3;
                    }
                }
                7 => {
                    let v1 = self.read(self.program[ip + 1], mode_1);
                    let v2 = self.read(self.program[ip + 2], mode_2);
                    self.write(self.program[ip + 3], mode_3, if v1 < v2 { 1 } else { 0 });
                    ip += 4;
                }
                8 => {
                    let v1 = self.read(self.program[ip + 1], mode_1);
                    let v2 = self.read(self.program[ip + 2], mode_2);
                    self.write(self.program[ip + 3], mode_3, if v1 == v2 { 1 } else { 0 });
                    ip += 4;
                }
                9 => {
                    let offset = self.read(self.program[ip + 1], mode_1);
                    self.relative_base = self.relative_base as i64 + offset;
                    ip += 2;
                }
                99 => {
                    self.ip = ip;
                    break;
                }
                _ => panic!("Unknown opcode!"),
            }
        }
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
        for item in values.iter().take(5)
		{
            amps.push(Amplifier {
                v: v.clone(),
                phase: *item,
                input: 0,
                i: 0,
            });
        }

        let mut vv = amps[0].run();
        for item in amps.iter_mut().take(4+1).skip(1) {
            item.input = vv;
            vv = item.run();
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
        amps = Vec::new();
        for item in values.iter().take(5) {
            amps.push(Amplifier {
                v: v.clone(),
                phase: *item,
                input: 0,
                i: 0,
            });
        }

        let mut input = 0;
        loop {
            amps[0].input = input;
            let mut vv = amps[0].run();
            if vv == -1 {
                if input  > max_signal
                {
                    max_signal = input;
                }
                break;
            }
			for item in amps.iter_mut().take(4+1).skip(1){
                item.input = vv;
                vv = item.run();
            }
            input = vv;
        }
    }

    println!("Part 2: {}", max_signal);
}
