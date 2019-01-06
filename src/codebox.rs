#[derive(Debug, Hash, PartialEq)]
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
    code: Vec<Instruction>,
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
        let mut code = vec![Instruction::Noop; width * lines.len()];

        for (y, line) in lines.into_iter().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                code[y * width + x] = if chr == ' ' {
                    Instruction::Noop
                } else {
                    // technically, some of these ops might be invalid
                    // we'll handle that during interpretation
                    Instruction::Op(chr)
                };
            }
        }

        Self {
            code,
            width,
            height,
        }
    }

    pub fn instruction_at(&self, pos: &Pos) -> Instruction {
        let pos = pos.y * self.width + pos.x;
        if pos >= self.code.len() {
            Instruction::Noop
        } else {
            self.code[pos]
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
