#![feature(backtrace)]

use red_cod::Interpreter;

use std::error::Error;
use std::fs::read_to_string;
use std::io::{self, Read, Stdin};
use std::os::unix::io::AsRawFd;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    let file = args.get(1).unwrap();
    let data = read_to_string(file)?;

    // termios code based on https://stackoverflow.com/a/37416107
    let stdin_fd = io::stdin().as_raw_fd();
    let termios = Termios::from_fd(stdin_fd).expect("failed to open stdin from fd");
    let mut new_termios = termios.clone(); // make a mutable copy of termios
                                           // that we will modify
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin_fd, TCSANOW, &mut new_termios).expect("failed to enter raw mode");

    let stdin_iter = StdinIter(io::stdin());
    let mut interpreter = Interpreter::new(&data, stdin_iter);
    let res = interpreter.run_to_end();

    tcsetattr(stdin_fd, TCSANOW, &termios).expect("failed to restore tty state");

    println!();
    Ok(res?)
}

struct StdinIter(Stdin);

impl Iterator for StdinIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 1];
        self.0.read_exact(&mut buf).ok()?;
        Some(buf[0] as char)
    }
}
