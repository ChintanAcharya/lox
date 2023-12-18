#![allow(dead_code)]

mod scanner;

use std::io::{stdin, stdout, Write};
use std::process;
use std::{env, fs};

use crate::scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_] => run_prompt(),
        [_, file_arg] => run_file(file_arg),
        _ => {
            println!("Usage: lox [script]");
            process::exit(64)
        }
    };
}

fn run_prompt() {
    loop {
        let mut input = String::new();
        print!(">");
        stdout().flush().expect("Failed to flush stdout");
        if let Ok(n) = stdin().read_line(&mut input) {
            if n == 0 {
                end();
            }
            run(input.as_ref())
        } else {
            end();
        }
    }
}

fn run(source: &str) {
    let scanner = Scanner::new(source);
    dbg!(scanner.scan_tokens());
}

fn end() -> ! {
    println!("reached end of input");
    process::exit(0)
}

fn run_file(file_name: &str) {
    println!("Running file: {}", file_name);
    let file_contents =
        fs::read_to_string(&file_name).expect(&format!("Failed to open file {}", file_name));

    run(&file_contents);
}
