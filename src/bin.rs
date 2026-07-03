use std::{
    collections::hash_map::Iter,
    env::{self, VarError},
    path::{self, Path},
};

use crate::{
    command::{Cmd, CmdType},
    util::is_executable,
};

pub fn exit(cmd: &Cmd, args: &[&str]) {
    if args.len() > 1 {
        println!("'{}': too many arguments", cmd.name);
        return;
    }

    let code = match args.first() {
        Some(arg) => arg.parse().expect("Invalid Argument"),
        None => 0,
    };

    std::process::exit(code);
}

pub fn echo(_: &Cmd, args: &[&str]) {
    if !args.is_empty() {
        let output = args.join(" ");
        println!("{output}");
    }
}

pub fn type_(_: &Cmd, _: &[&str]) {
    unimplemented!()
}

// special handlers

pub fn handle_ext(cmd: &str) -> Result<(), VarError> {
    let path = env::var("PATH")?;

    if let Some(exec_path) = path
        .split(":")
        .map(Path::new)
        .find(|directory| is_executable(Path::join(directory, cmd)))
    {
        let path_str = format!(
            "{}{}{cmd}",
            exec_path.to_string_lossy(),
            path::MAIN_SEPARATOR_STR
        );

        println!("{cmd} is {path_str}");
        Ok(())
    } else {
        Err(VarError::NotPresent)
    }
}

pub fn handle_type_cmd(args: &[&str], commands_iter: &mut Iter<'_, &str, CmdType>) {
    if args.len() > 1 {
        println!("'type': too many arguments");
        return;
    }

    let cmd_arg = args[0];
    let matched_command = commands_iter.find(|command| *command.0 == cmd_arg);

    match matched_command {
        Some((cmd, cmd_type)) => match cmd_type {
            CmdType::BuiltIn(..) => println!("{cmd} is a shell builtin "),
        },
        None => {
            if handle_ext(cmd_arg).is_err() {
                println!("{cmd_arg}: not found");
            }
        }
    }
}
