#![allow(unused_imports)]
use codecrafters_shell::bin::handle_type_cmd;
use codecrafters_shell::command::{Cmd, CmdType};
use codecrafters_shell::{bin, command, load_commands};
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};

fn main() {
    let built_in = &mut load_commands();

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

            let mut commands_iter = built_in.iter();

            if *cmd == "type" {
                handle_type_cmd(args, &mut commands_iter);
                continue;
            }

            command::run(cmd, args, built_in);
        }
    }
}
