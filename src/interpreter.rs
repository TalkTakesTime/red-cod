use crate::codebox::{Codebox, Instruction, Pos};
use crate::stack::ProgramStack;

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
enum State {
    Running,
    Done,
}

#[derive(Debug, PartialEq)]
enum ParseMode {
    Normal,
    Text(char),
}

#[derive(Debug)]
enum RuntimeError {
    InvalidInstruction(char),
    UnimplementedInstruction(char),
    CharConversionFailure,
}

#[derive(Debug)]
pub struct Interpreter {
    codebox: Codebox,
    stack: ProgramStack,
    ptr: Pos,
    dir: Direction,
    state: State,
    mode: ParseMode,
}

impl Interpreter {
    pub fn new(code: &str) -> Self {
        Self {
            codebox: Codebox::new(code),
            stack: ProgramStack::new(),
            ptr: Pos { x: 0, y: 0 },
            dir: Direction::East,
            state: State::Running,
            mode: ParseMode::Normal,
        }
    }

    pub fn run(&mut self) {
        if let Ok(_) = self.run_to_end() {
            println!();
        } else {
            println!("something smells fishy...");
        }
    }

    fn run_to_end(&mut self) -> Result<(), Box<Error>> {
        while self.state != State::Done {
            self.step()?;
        }
        Ok(())
    }

    fn step(&mut self) -> Result<(), Box<Error>> {
        let instr = self.codebox.instruction_at(&self.ptr);
        if let Instruction::Op(instr) = instr {
            self.execute_instruction(instr)?;
        } else if let ParseMode::Text(_) = self.mode {
            self.push_char(' ');
        }
        self.move_to_next();
        Ok(())
    }

    fn execute_instruction(&mut self, instr: char) -> Result<(), Box<Error>> {
        if let ParseMode::Text(quote_type) = self.mode {
            if instr != quote_type {
                self.push_char(instr);
                return Ok(());
            }
        }

        match instr {
            // literals
            '0'...'9' | 'a'...'f' => self.push_num(instr),

            // maths
            '+' => self.stack.top().add()?,
            '-' => self.stack.top().subtract()?,
            '*' => self.stack.top().multiply()?,
            ',' => self.stack.top().divide()?,
            '%' => self.stack.top().modulo()?,

            // comparisons
            '=' => self.stack.top().equals()?,
            ')' => self.stack.top().greater_than()?,
            '(' => self.stack.top().less_than()?,

            // stack manipulation
            ':' => self.stack.top().dup()?,
            '~' => {
                self.stack.top().pop()?;
            }
            '$' => self.stack.top().swap(2)?,
            '@' => self.stack.top().swap(3)?,
            '}' => self.stack.top().shift_right(),
            '{' => self.stack.top().shift_left(),
            '[' => self.stack.split_stack()?,
            ']' => self.stack.drop_stack(),
            'l' => self.stack.top().push_len(),
            'r' => self.stack.top().reverse(),

            // trampolines
            '!' => self.move_to_next(),
            '?' => {
                if self.stack.top().pop()? == 0f64 {
                    self.move_to_next();
                }
            }

            // directions
            '^' => self.dir = Direction::North,
            '>' => self.dir = Direction::East,
            'v' => self.dir = Direction::South,
            '<' => self.dir = Direction::West,

            // mirrors
            '/' => {
                self.dir = match self.dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                }
            }
            '\\' => {
                self.dir = match self.dir {
                    Direction::North => Direction::West,
                    Direction::East => Direction::South,
                    Direction::South => Direction::East,
                    Direction::West => Direction::North,
                }
            }
            '|' => {
                if self.dir == Direction::West || self.dir == Direction::East {
                    self.dir = self.dir.reverse();
                }
            }
            '_' => {
                if self.dir == Direction::North || self.dir == Direction::North {
                    self.dir = self.dir.reverse();
                }
            }
            '#' => self.dir = self.dir.reverse(),

            // input/output
            '"' | '\'' => self.switch_parse_mode(instr),
            'n' => print!("{}", self.stack.top().pop()?),
            'o' => print_char(self.stack.top().pop()?)?,

            // end
            ';' => self.state = State::Done,

            // explicitly unimplemented
            'p' | 'g' => Err(RuntimeError::UnimplementedInstruction(instr))?,

            // yet to be implemented
            'x' | '.' => Err(RuntimeError::UnimplementedInstruction(instr))?,

            // everything else
            _ => Err(RuntimeError::InvalidInstruction(instr))?,
        }
        Ok(())
    }

    fn move_to_next(&mut self) {
        self.ptr = self.get_next_pos();

        // in text mode, noops can't be skipped
        if self.mode == ParseMode::Normal {
            while self.codebox.instruction_at(&self.ptr) == Instruction::Noop {
                self.ptr = self.get_next_pos();
            }
        }
    }

    fn get_next_pos(&self) -> Pos {
        let Pos { x, y } = self.ptr;
        match self.dir {
            Direction::North => Pos {
                y: get_wrapped_coord(y, -1, self.codebox.height()),
                x,
            },
            Direction::East => Pos {
                y,
                x: get_wrapped_coord(x, 1, self.codebox.width()),
            },
            Direction::South => Pos {
                y: get_wrapped_coord(y, 1, self.codebox.height()),
                x,
            },
            Direction::West => Pos {
                y,
                x: get_wrapped_coord(x, -1, self.codebox.width()),
            },
        }
    }

    fn push_num(&mut self, chr: char) {
        self.stack.top().push(chr.to_digit(16).unwrap() as f64);
    }

    fn push_char(&mut self, chr: char) {
        self.stack.top().push((chr as u32) as f64);
    }

    fn switch_parse_mode(&mut self, quote_type: char) {
        self.mode = if self.mode == ParseMode::Normal {
            ParseMode::Text(quote_type)
        } else {
            ParseMode::Normal
        }
    }
}

fn get_wrapped_coord(coord: usize, incr: isize, max: usize) -> usize {
    let coord = coord as isize;
    if coord == 0 && incr < 0 {
        max - 1
    } else if coord + incr >= max as isize {
        0
    } else {
        (coord + incr) as usize
    }
}

fn print_char(chr: f64) -> Result<(), RuntimeError> {
    if chr < u32::min_value() as f64 || chr > u32::max_value() as f64 || chr != chr.trunc() {
        return Err(RuntimeError::CharConversionFailure);
    }
    let chr = std::char::from_u32(chr as u32).ok_or(RuntimeError::CharConversionFailure)?;
    print!("{}", chr as char);
    Ok(())
}

impl Direction {
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl Error for RuntimeError {
    fn description(&self) -> &str {
        "" // hm
    }
}

#[cfg(test)]
mod test {
    use super::Interpreter;

    #[test]
    fn test_helloworld() {
        let mut interpreter = Interpreter::new(
            "\"hello, world\"rv
          o;!?l<",
        );

        let res = interpreter.run_to_end();
        if res.is_err() {
            println!();
            println!("{:#?}", interpreter);
        }
        println!();
    }

    #[test]
    fn test_fizzbuzz() {
        let mut interpreter = Interpreter::new(
            "0voa                            ~/?=0:\\
 voa            oooo'Buzz'~<     /
 >1+:aa*1+=?;::5%:{3%:@*?\\?/'zziF'oooo/
 ^oa                 n:~~/",
        );

        let res = interpreter.run_to_end();
        if res.is_err() {
            println!();
            println!("{:#?}", interpreter);
        }
        println!();
    }
}
