pub struct IntcodeMachine {
    program: Vec<i64>,
    ip: usize,
    input: Option<i64>,
    phase: Option<i64>,
    relative_base: i64,
	output : Option<i64>,
}

impl IntcodeMachine {
    pub fn new(program: Vec<i64>) -> IntcodeMachine {
        IntcodeMachine {
            program,
            ip: 0,
            relative_base: 0,
            input: None,
            phase: None,
			output: None,
        }
    }

    pub fn set_input(&mut self, input: i64)
    {
        self.input = Some(input);
    }

    pub fn set_phase(&mut self, phase: i64)
    {
        self.phase = Some(phase);
    }

    fn get_mode(&self, instruction: i64, place: i64) -> i64 {
        (instruction % place) / (place / 10)
    }

	pub fn get_output(&self) -> Option<i64>
	{
		self.output
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
		self.output = None;
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
                    let input_val =
                    {
                        match self.phase
                        {
                            Some(x) =>
                            {
                                self.phase = None;
                                x
                            }
                            None => match self.input
                            {
                                Some(y) => y,
                                None => panic!("No input provided"),
                            }
                        }
                    };
                    self.write(self.program[ip + 1], mode_1, input_val);
                    ip += 2;
                }
                4 => {
					self.output = Some(self.read(self.program[ip + 1], mode_1));
                    ip += 2;
					self.ip = ip;
					break;
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
