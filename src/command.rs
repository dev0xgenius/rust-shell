use super::*;
use std::collections::HashMap;

pub enum CmdType {
    BuiltIn(fn(&Cmd, &[&str])),
}

pub struct Cmd<'a> {
    pub name: &'a str,
    pub cmd_type: &'a CmdType,
}

pub fn run(cmd: &str, args: &[&str], commands: &HashMap<&str, CmdType>) {
    let mut commands_iter = commands.iter();

    match commands_iter.find(|command| *command.0 == cmd) {
        Some(command) => {
            let cmd = &Cmd {
                name: command.0,
                cmd_type: command.1,
            };

            match cmd.cmd_type {
                CmdType::BuiltIn(operation) => operation(cmd, args),
            }
        }

        None => println!("{cmd}: command not found"),
    }
}

pub fn load_commands() -> HashMap<&'static str, CmdType> {
    let mut built_in = HashMap::new();

    built_in.insert("exit", CmdType::BuiltIn(bin::exit));
    built_in.insert("echo", CmdType::BuiltIn(bin::echo));
    built_in.insert("type", CmdType::BuiltIn(bin::type_));

    built_in
}
