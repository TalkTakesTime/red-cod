use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    Noop,
    Op(char),
}

#[derive(Debug)]
pub struct Codebox {
    code: HashMap<Pos, Instruction>,
    width: usize,
    height: usize,
}

impl Codebox {
    pub fn new(code: &str) -> Self {
        let lines: Vec<_> = code.lines().map(String::from).collect();
        let width = lines
            .iter()
            .max_by_key(|l| l.len())
            .unwrap_or(&String::new())
            .len();
        let height = lines.len();
        let mut code = HashMap::new();

        for (y, line) in lines.into_iter().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                code.insert(
                    Pos { x, y },
                    if chr == ' ' {
                        Instruction::Noop
                    } else {
                        // technically, some of these ops might be invalid
                        // we'll handle that during interpretation
                        Instruction::Op(chr)
                    },
                );
            }
        }

        Self {
            code,
            width,
            height,
        }
    }

    pub fn get_instruction(&self, pos: &Pos) -> Instruction {
        *self.code.get(pos).unwrap_or(&Instruction::Noop)
    }

    pub fn set_instruction(&mut self, pos: Pos, instr: char) {
        self.code.insert(pos, Instruction::Op(instr));
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
