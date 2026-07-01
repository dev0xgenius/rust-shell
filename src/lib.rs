use std::collections::HashMap;

use crate::command::CmdType;

pub mod bin;

pub fn load_commands() -> HashMap<&'static str, command::CmdType> {
    let mut built_in = HashMap::new();

    built_in.insert("exit", CmdType::BuiltIn(bin::exit));
    built_in.insert("echo", CmdType::BuiltIn(bin::echo));
    built_in.insert("type", CmdType::BuiltIn(bin::type_));

    built_in
}

pub mod command {
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

            None => println!("invalid command: command not found"),
        }
    }
}
