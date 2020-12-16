use std::fs;

#[derive(Copy, Clone, Debug)]
enum Operations {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

struct Console {
    program_counter: isize,
    accumulator: isize,
    instructions_count: isize,
    instructions: Vec<Operations>,
    executed_instructions: Vec<bool>,
    halts: bool,
}

impl Console {
    fn new(instructions: &Vec<Operations>) -> Self {
        Console {
            program_counter: 0,
            accumulator: 0,
            instructions: instructions.clone(),
            instructions_count: instructions.len() as isize,
            executed_instructions: vec![false; instructions.len()],
            halts: false,
        }
    }
    fn run(&mut self) {
        while self.program_counter < self.instructions_count
            && !self.executed_instructions[self.program_counter as usize]
        {
            let ins = self.instructions[self.program_counter as usize];
            match ins {
                Operations::Acc(x) => {
                    self.accumulator += x;
                    self.executed_instructions[self.program_counter as usize] = true;
                    self.program_counter += 1
                }
                Operations::Jmp(x) => {
                    self.executed_instructions[self.program_counter as usize] = true;
                    self.program_counter += x;
                }
                Operations::Nop(_) => {
                    self.executed_instructions[self.program_counter as usize] = true;
                    self.program_counter += 1;
                }
            }
        }
        if self.program_counter >= self.instructions_count {
            self.halts = true;
        }
    }
}

fn read_input(path: &str) -> Vec<Operations> {
    let mut instructions = Vec::<Operations>::new();
    let input = fs::read_to_string(path)
        .expect("Read Error")
        .lines()
        .map(|l| l.split(" ").map(|s| s.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for ins in input {
        match ins[0].as_str() {
            "nop" => instructions.push(Operations::Nop(ins[1].parse().expect("Parse Error"))),
            "acc" => instructions.push(Operations::Acc(ins[1].parse().expect("Parse Error"))),
            "jmp" => instructions.push(Operations::Jmp(ins[1].parse().expect("Parse Error"))),
            _ => (),
        }
    }
    return instructions;
}

pub fn solve(path: &str) {
    let instructions = read_input(path);
    let mut console = Console::new(&instructions);
    console.run();
    let part1 = console.accumulator;

    let mut part2 = 0;
    for (i, ins) in instructions.iter().enumerate() {
        if let Operations::Nop(x) = ins {
            let mut new_instructions = instructions.clone();
            new_instructions[i] = Operations::Jmp(*x);
            let mut console = Console::new(&new_instructions);
            console.run();
            if console.halts {
                part2 = console.accumulator;
                break;
            }
        }

        if let Operations::Jmp(x) = ins {
            let mut new_instructions = instructions.clone();
            new_instructions[i] = Operations::Nop(*x);
            let mut console = Console::new(&new_instructions);
            console.run();
            if console.halts {
                part2 = console.accumulator;
                break;
            }
        }
    }

    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
