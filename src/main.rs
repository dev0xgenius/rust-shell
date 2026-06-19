#[allow(unused_imports)]
use std::convert::TryInto;
use std::io::{self, Write};

const KEYWORDS: [&str; 1] = ["echo"];

fn main() {
    loop {
        print!("$ ");
        let mut input = String::new();
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("An internal error occured");

        let input: Vec<&str> = input
            .split(' ')
            .map(|arg| arg.trim())
            .filter(|arg| !arg.is_empty())
            .collect();

        if !input.is_empty() {
            let cmd = &input[0];
            let args = &input[1..];

            if *cmd == "exit" {
                if args.len() > 1 {
                    println!("{cmd}: too many arguments");
                } else if args.is_empty() {
                    std::process::exit(0);
                } else {
                    let code = args[0].parse().unwrap_or(0);
                    std::process::exit(code);
                }
            } else if *cmd == "echo" {
            }
        }
    }
}
