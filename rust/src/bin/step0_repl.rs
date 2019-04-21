use std::io;
use std::io::{Write, BufRead};

fn read(input: &str) -> &str {
    return input;
}

fn eval(input: &str) -> &str {
    return input;
}

fn print(input: &str) -> &str {
    return input;
}

fn rep(input: &str) -> &str {
    print(eval(read(input)))
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut hstdin = stdin.lock();

    print!("user> ");
    io::stdout().flush().unwrap();
    let mut bytes_read = hstdin.read_line(&mut buffer).unwrap();

    while bytes_read > 0 {
        print!("{}", rep(&buffer));
        print!("user> ");
        io::stdout().flush().unwrap();

        buffer.clear();
        bytes_read = hstdin.read_line(&mut buffer).unwrap();
    }
}
