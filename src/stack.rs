use std::collections::VecDeque;
use std::iter::FromIterator;

pub struct ProgramStack {
    base: Stack,
    substacks: Vec<Stack>,
}

#[derive(Debug, PartialEq)]
pub enum StackError {
    Underflow,
}

impl ProgramStack {
    fn curr(&mut self) -> &mut Stack {
        self.substacks.last_mut().unwrap_or(&mut self.base)
    }

    // [
    pub fn new_stack(&mut self, n: usize) -> Result<(), StackError> {
        let new_stack = self.curr().split(n)?;
        self.substacks.push(new_stack);
        Ok(())
    }

    // ]
    pub fn drop_stack(&mut self) {
        if let Some(top) = self.substacks.pop() {
            self.curr().extend(top);
        } else {
            self.curr().clear();
        }
    }
}

pub struct Stack {
    entries: VecDeque<f64>,
    register: Option<f64>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::new(),
            register: None,
        }
    }

    pub fn pop(&mut self) -> Result<f64, StackError> {
        self.entries.pop_back().ok_or(StackError::Underflow)
    }

    pub fn push(&mut self, val: f64) {
        self.entries.push_back(val);
    }

    pub fn clear(&mut self) {
        self.register = None;
        self.entries.clear();
    }

    pub fn split(&mut self, n: usize) -> Result<Self, StackError> {
        let self_len = self.entries.len();
        if self_len < n {
            Err(StackError::Underflow)
        } else {
            let s = self.entries.split_off(self_len - n);
            Ok(s.into_iter().collect())
        }
    }

    // +
    pub fn add(&mut self) -> Result<(), StackError> {
        let x = self.pop()?;
        let y = self.pop()?;
        self.push(y + x);
        Ok(())
    }

    // -
    pub fn subtract(&mut self) -> Result<(), StackError> {
        let x = self.pop()?;
        let y = self.pop()?;
        self.push(y - x);
        Ok(())
    }

    // *
    pub fn multiply(&mut self) -> Result<(), StackError> {
        let x = self.pop()?;
        let y = self.pop()?;
        self.push(y * x);
        Ok(())
    }

    // ,
    pub fn divide(&mut self) -> Result<(), StackError> {
        let x = self.pop()?;
        let y = self.pop()?;
        self.push(y / x);
        Ok(())
    }

    // %
    pub fn modulo(&mut self) -> Result<(), StackError> {
        let x = self.pop()?;
        let y = self.pop()?;
        self.push(y % x);
        Ok(())
    }

    // =
    pub fn equals(&mut self) -> Result<(), StackError> {
        let x = self.pop()?;
        let y = self.pop()?;
        self.push(if y == x { 1f64 } else { 0f64 });
        Ok(())
    }

    // )
    pub fn greater_than(&mut self) -> Result<(), StackError> {
        let x = self.pop()?;
        let y = self.pop()?;
        self.push(if y > x { 1f64 } else { 0f64 });
        Ok(())
    }

    // (
    pub fn less_than(&mut self) -> Result<(), StackError> {
        let x = self.pop()?;
        let y = self.pop()?;
        self.push(if y < x { 1f64 } else { 0f64 });
        Ok(())
    }

    // :
    pub fn dup(&mut self) -> Result<(), StackError> {
        let val = self.entries.back().ok_or(StackError::Underflow)?;
        self.push(*val);
        Ok(())
    }

    // $ and @
    pub fn swap(&mut self, n: usize) -> Result<(), StackError> {
        let len = self.entries.len();
        if n > len {
            return Err(StackError::Underflow);
        }

        for i in (len - 1)..(len - n) {
            self.entries.swap(i, i - 1);
        }
        Ok(())
    }

    // }
    pub fn shift_right(&mut self) {
        if let Some(val) = self.entries.pop_back() {
            self.entries.push_front(val);
        }
    }

    // {
    pub fn shift_left(&mut self) {
        if let Some(val) = self.entries.pop_front() {
            self.entries.push_back(val);
        }
    }

    // r
    pub fn reverse(&mut self) {
        let vals: Vec<_> = self.entries.drain(..).collect();
        for val in vals {
            self.entries.push_front(val);
        }
    }

    // l
    pub fn push_len(&mut self) {
        self.entries.push_back(self.entries.len() as f64);
    }
}

impl FromIterator<f64> for Stack {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        Self {
            entries: iter.into_iter().collect(),
            register: None,
        }
    }
}

impl IntoIterator for Stack {
    type Item = f64;
    type IntoIter = std::collections::vec_deque::IntoIter<f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl Extend<f64> for Stack {
    fn extend<I: IntoIterator<Item = f64>>(&mut self, iter: I) {
        self.entries.extend(iter);
    }
}

#[cfg(test)]
mod test {
    mod stack {
        use super::super::*;

        macro_rules! stack {
            ( $( $x:expr ),* ) => {
                {
                    let mut temp_stack = Stack::new();
                    $(
                        temp_stack.push($x);
                    )*
                    temp_stack
                }
            };
        }

        macro_rules! assert_stack_eq {
            ($s:expr, $v:expr) => {{
                let stack_vec: Vec<_> = $s.into_iter().collect();
                assert_eq!(stack_vec, $v);
            }};
        }

        #[test]
        fn test_into_iterator() {
            let s = stack![1f64, 2f64, 3f64];
            assert_stack_eq!(s, vec![1f64, 2f64, 3f64]);
        }

        #[test]
        fn test_pop_on_nonempty_stack() {
            let mut s = stack![1f64, 2f64, 3f64];
            assert_eq!(s.pop(), Ok(3f64));
        }

        #[test]
        fn test_pop_on_empty_stack() {
            let mut s = stack![];
            assert_eq!(s.pop(), Err(StackError::Underflow));
        }

        #[test]
        fn test_add() {
            let mut s = stack![1f64, 2f64];
            s.add().unwrap();
            assert_stack_eq!(s, vec![3f64]);
        }

        #[test]
        fn test_subtract() {
            let mut s = stack![2f64, 1f64];
            s.subtract().unwrap();
            assert_stack_eq!(s, vec![1f64]);
        }

        #[test]
        fn test_multiply() {
            let mut s = stack![2f64, 3f64];
            s.multiply().unwrap();
            assert_stack_eq!(s, vec![6f64]);
        }

        #[test]
        fn test_divide() {
            let mut s = stack![10f64, 5f64];
            s.divide().unwrap();
            assert_stack_eq!(s, vec![2f64]);
        }

        #[test]
        fn test_modulo() {
            let mut s = stack![10f64, 3f64];
            s.modulo().unwrap();
            assert_stack_eq!(s, vec![1f64]);
        }

        #[test]
        fn test_equal_with_equal_vals() {
            let mut s = stack![2f64, 2f64];
            s.equals().unwrap();
            assert_stack_eq!(s, vec![1f64]);
        }

        #[test]
        fn test_equal_with_inequal_vals() {
            let mut s = stack![2f64, 3f64];
            s.equals().unwrap();
            assert_stack_eq!(s, vec![0f64]);
        }
    }
}
