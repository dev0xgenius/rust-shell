#[allow(unused_imports)]
use codecrafters_shell::Cmd;
// use std::convert::TryInto;
use std::io::{self, Write};

// const KEYWORDS: [&str; 1] = ["echo"];

fn main() {
    let mut valid_commands: Vec<Cmd> = Vec::new();

    valid_commands.push(Cmd {
        cmd: String::from("exit"),
        description: String::from("exit is a shell built-in"),
        operation: Box::new(|arg| std::process::exit(arg)),
    });

    loop {
        print!("$ ");
        let mut input = String::new();
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("an internal error occured");

        let input: Vec<&str> = input
            .split(' ')
            .map(|arg| arg.trim())
            .filter(|arg| !arg.is_empty())
            .collect();

        if !input.is_empty() {
            let cmd = &input[0];
            let args = &input[1..];

            // match valid_commands.iter().find(|&command| command.cmd == *cmd) {
            //     Some(cmd) => cmd.exec(args),
            //     None => println!("{}: not found", *cmd),
            // }

            if *cmd == "exit" {
                if args.len() > 1 {
                    println!("{cmd}: too many arguments");
                } else if args.is_empty() {
                    std::process::exit(0);
                } else {
                    let code = args[0].parse().unwrap_or(0);
                    std::process::exit(code);
                }
            } else if *cmd == "echo" && !args.is_empty() {
                let output = args.join(" ");
                println!("{output}");
            } else {
                println!("{cmd}: not found");
            }
        }
    }
}
