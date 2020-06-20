#![feature(backtrace)]

mod codebox;
mod interpreter;
mod stack;

pub use interpreter::Interpreter;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
