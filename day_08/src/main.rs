extern crate im_rc;
extern crate itertools;

use im_rc::Vector;
use itertools::Itertools;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

fn main() {
    println!("--- [AoC 2020] Day 8: Handheld Halting ---");
    let input: Vector<String> = utils::read_strings_from_param();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vector<String>) {
    let instructions: Vector<Instruction> = input
        .iter()
        .map(|line| Instruction::try_from(line).unwrap())
        .collect();

    let mut interpreter = Interpreter::new(instructions);

    let mut last_value = interpreter.accumulator;
    while !interpreter.in_loop() {
        last_value = interpreter.accumulator;
        interpreter.execute();
    }

    println!("Solution to part one: {}", last_value);
}

fn part_two(input: &Vector<String>) {
    let instructions: Vector<Instruction> = input
        .iter()
        .map(|line| Instruction::try_from(line).unwrap())
        .collect();

    let mut interpreter = Interpreter::new(instructions);

    let mut change_offset: Option<usize> = None;

    while !interpreter.is_done() {
        interpreter.execute();
        if interpreter.in_loop() {
            if change_offset.is_some() {
                interpreter.flip(change_offset.unwrap());
            }
            change_offset = Some(interpreter.flip_next(change_offset.map(|i| i + 1).unwrap_or(0)));
            interpreter.reset();
        }
    }

    println!("Solution to part two: {}", interpreter.accumulator);
}

struct Interpreter {
    instructions: Vector<Instruction>,
    accumulator: i64,
    offset: usize,
    history: Vector<usize>,
}

impl Interpreter {
    fn new(instructions: Vector<Instruction>) -> Interpreter {
        Interpreter {
            instructions,
            accumulator: 0,
            offset: 0,
            history: Vector::new(),
        }
    }

    fn flip(&mut self, position: usize) {
        let replacement = match &self.instructions[position] {
            Instruction::NoOp(value) => Instruction::Jump(*value),
            Instruction::Jump(value) => Instruction::NoOp(*value),
            Instruction::Accumulator(value) => Instruction::Accumulator(*value),
        };

        self.instructions[position] = replacement;
    }

    fn flip_next(&mut self, offset: usize) -> usize {
        let possible_flip_offset = offset
            + self
                .instructions
                .iter()
                .skip(offset)
                .position(|ins| match ins {
                    Instruction::NoOp(_) | Instruction::Jump(_) => true,
                    _ => false,
                })
                .unwrap();

        self.flip(possible_flip_offset);
        possible_flip_offset
    }

    fn reset(&mut self) {
        self.offset = 0;
        self.accumulator = 0;
        self.history = Vector::new();
    }

    fn state(&self) -> (usize, i64) {
        (self.offset, self.accumulator)
    }

    fn current_instruction(&self) -> &Instruction {
        &self.instructions[self.offset]
    }

    fn update_history(&mut self) {
        self.history.insert_ord(self.offset);
    }

    fn execute(&mut self) {
        let instruction = self.current_instruction();
        let (next_offset, next_accumulator) = instruction.execute(self);
        self.update_history();
        self.offset = next_offset;
        self.accumulator = next_accumulator;
    }

    fn in_loop(&self) -> bool {
        self.history.binary_search(&self.offset).is_ok()
    }

    fn is_done(&self) -> bool {
        self.offset >= self.instructions.len()
    }
}

impl fmt::Display for Interpreter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:03}: {} | {:+}",
            self.offset,
            self.current_instruction(),
            self.accumulator
        )
    }
}

impl fmt::Debug for Interpreter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = format!("------------\n acc: {}\n------------", self.accumulator);
        for pos in 0..self.instructions.len() {
            let in_history = self.history.binary_search(&pos).is_ok();
            let current_off = pos == self.offset;
            let pos_marker = if current_off { ">" } else { " " };
            let his_marker = if in_history { "X" } else { "" };
            buffer = format!(
                "{}\n{} {} {}",
                buffer, pos_marker, self.instructions[pos], his_marker
            )
        }

        write!(f, "{}\n------------", buffer)
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Accumulator(i64),
    Jump(i64),
    NoOp(i64),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Accumulator(value) => write!(f, "acc {:>+3}", value),
            Self::Jump(value) => write!(f, "jmp {:>+3}", value),
            Self::NoOp(value) => write!(f, "nop {:>+3}", value),
        }
    }
}

impl Instruction {
    fn execute(&self, interpreter: &Interpreter) -> (usize, i64) {
        let (current_offset, current_accumulator) = interpreter.state();
        match self {
            Self::Accumulator(delta) => (current_offset + 1, current_accumulator + delta),
            Self::Jump(offset) => {
                let next_offset: usize = (current_offset as i64 + offset).try_into().unwrap();
                (next_offset, current_accumulator)
            }
            _ => (current_offset + 1, current_accumulator),
        }
    }
}

impl TryFrom<&String> for Instruction {
    type Error = ();

    fn try_from(input: &String) -> Result<Instruction, Self::Error> {
        let (command, argument): (&str, &str) =
            input.split_ascii_whitespace().collect_tuple().unwrap();
        let numeric_argument = argument.parse::<i64>().unwrap();
        match command {
            "nop" => Ok(Self::NoOp(numeric_argument)),
            "acc" => Ok(Self::Accumulator(numeric_argument)),
            "jmp" => Ok(Self::Jump(numeric_argument)),
            _ => Err(()),
        }
    }
}
