use std::collections::VecDeque;

pub struct Stack {
    entries: VecDeque<char>,
    register: Option<char>,
}

pub struct ProgramStack {
    base: Stack,
    children: Vec<Stack>,
}

pub enum StackError {
    Underflow,
}

impl ProgramStack {
    fn curr(&mut self) -> &mut Stack {
        self.children.last_mut().unwrap_or(&mut self.base)
    }

    pub fn new_stack(&mut self, n: usize) -> Result<(), StackError> {
        let s_len = self.curr().len();
        if n > s_len {
            Err(StackError::Underflow)
        } else {
            let new_stack = self.curr().split_off(s_len - n);
            self.children.push(new_stack);
            Ok(())
        }
    }

    pub fn drop_stack(&mut self) {
        if self.children.len() > 0 {

        }
    }
}
