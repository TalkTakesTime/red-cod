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
    pub fn split_stack(&mut self) -> Result<(), StackError> {
        let new_stack = self.curr().split()?;
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

    pub fn split(&mut self) -> Result<Self, StackError> {
        let n = self.pop()? as usize;
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
        self.push(if (y - x).abs() < std::f64::EPSILON {
            1f64
        } else {
            0f64
        });
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

    // &
    pub fn swap_register(&mut self) -> Result<(), StackError> {
        if let Some(val) = self.register {
            self.push(val);
        } else {
            self.register = Some(self.pop()?);
        }
        Ok(())
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
                    #[allow(unused_mut)]
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

        macro_rules! verify_result {
            ($stack:ident, $actual:ident, {
                result: $expected:expr,
                stack: [$($res_vals:expr),*]
            }) => {
                assert_eq!($actual, $expected);
                assert_stack_eq!($stack, vec![$($res_vals),*]);
            };
            ($stack:ident, $actual:ident, $expected:expr) => {
                verify_result!($stack, $actual, { result: $expected, stack: [] });
            };
        }

        macro_rules! test_stack_method {
            (method: $method:ident, cases: {
                $(
                    $name:ident: [$($init_vals:expr),*] => $result:tt,
                )*
            }) => {
                mod $method {
                    use super::*;
                    $(
                        #[test]
                        fn $name() {
                            let mut test_stack = stack![$($init_vals),*];
                            let op_result = test_stack.$method();
                            verify_result!(test_stack, op_result, $result);
                        }
                    )*
                }
            };
        }

        #[test]
        fn test_into_iterator() {
            let s = stack![1f64, 2f64, 3f64];
            let stack_vec: Vec<_> = s.into_iter().collect();
            assert_eq!(stack_vec, vec![1f64, 2f64, 3f64]);
        }

        test_stack_method! {
            method: pop,
            cases: {
                empty_stack: [] => (Err(StackError::Underflow)), // why does this need parentheses?
                single_value: [1f64] => (Ok(1f64)),
                multiple_values: [3f64, 2f64] => {
                    result: Ok(2f64),
                    stack: [3f64]
                },
            }
        }

        test_stack_method! {
            method: add,
            cases: {
                empty_stack: [] => (Err(StackError::Underflow)), // why does this need parentheses?
                single_value: [1f64] => (Err(StackError::Underflow)),
                multiple_values: [1f64, 2f64] => {
                    result: Ok(()),
                    stack: [3f64]
                },
            }
        }

        test_stack_method! {
            method: subtract,
            cases: {
                empty_stack: [] => (Err(StackError::Underflow)), // why does this need parentheses?
                single_value: [1f64] => (Err(StackError::Underflow)),
                multiple_values: [3f64, 1f64] => {
                    result: Ok(()),
                    stack: [2f64]
                },
            }
        }

        test_stack_method! {
            method: multiply,
            cases: {
                empty_stack: [] => (Err(StackError::Underflow)), // why does this need parentheses?
                single_value: [1f64] => (Err(StackError::Underflow)),
                multiple_values: [3f64, 2f64] => {
                    result: Ok(()),
                    stack: [6f64]
                },
            }
        }

        test_stack_method! {
            method: divide,
            cases: {
                empty_stack: [] => (Err(StackError::Underflow)), // why does this need parentheses?
                single_value: [1f64] => (Err(StackError::Underflow)),
                multiple_values: [10f64, 5f64] => {
                    result: Ok(()),
                    stack: [2f64]
                },
                fractional_result: [5f64, 10f64] => {
                    result: Ok(()),
                    stack: [0.5f64]
                },
            }
        }

        test_stack_method! {
            method: modulo,
            cases: {
                empty_stack: [] => (Err(StackError::Underflow)), // why does this need parentheses?
                single_value: [1f64] => (Err(StackError::Underflow)),
                multiple_values: [10f64, 3f64] => {
                    result: Ok(()),
                    stack: [1f64]
                },
            }
        }

        test_stack_method! {
            method: equals,
            cases: {
                empty_stack: [] => (Err(StackError::Underflow)), // why does this need parentheses?
                single_value: [1f64] => (Err(StackError::Underflow)),
                inequal_values: [10f64, 3f64] => {
                    result: Ok(()),
                    stack: [0f64]
                },
                equal_values: [10f64, 10f64] => {
                    result: Ok(()),
                    stack: [1f64]
                },
            }
        }
    }
}
