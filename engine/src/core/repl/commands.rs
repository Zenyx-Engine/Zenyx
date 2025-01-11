use std::{fs, path::PathBuf, str::FromStr};

use anyhow::anyhow;
use colored::Colorize;
use mlua::prelude::*;
use mlua::{Lua, MultiValue};
use parking_lot::RwLock;
use regex::Regex;
use rustyline::{DefaultEditor, error::ReadlineError};

use super::{handler::Command, input::tokenize};
use crate::core::repl::handler::COMMAND_MANAGER;

#[derive(Default)]
pub struct HelpCommand;

impl Command for HelpCommand {
    fn execute(&self, _args: Option<Vec<String>>) -> Result<(), anyhow::Error> {
        let manager = COMMAND_MANAGER.read();
        println!("Available commands:\n");

        for (_, command) in manager.get_commands() {
            println!(
                "Command: {}\n\tDescription: {}\n\tParameters: {}\n\tHelp: {}\n",
                command.get_name().to_lowercase(),
                command.get_description(),
                command.get_params(),
                command.get_help()
            );
        }

        if !manager.aliases.is_empty() {
            println!("Aliases:");
            for (alias, command) in &manager.aliases {
                println!("\t{} -> {}", alias, command);
            }
        }
        Ok(())
    }

    fn undo(&self) {}

    fn redo(&self) {}

    fn get_description(&self) -> String {
        String::from("help")
    }

    fn get_help(&self) -> String {
        String::from("Displays a list of available commands and their descriptions.")
    }

    fn get_params(&self) -> String {
        String::from("No parameters required.")
    }

    fn get_name(&self) -> String {
        String::from("Help")
    }
}
#[derive(Default)]
pub struct ClearCommand;

impl Command for ClearCommand {
    fn execute(&self, _args: Option<Vec<String>>) -> Result<(), anyhow::Error> {
        println!("Clearing screen..., running command");
        let _result = if cfg!(target_os = "windows") {
            std::process::Command::new("cmd")
                .args(["/c", "cls"])
                .spawn()
        } else {
            std::process::Command::new("clear").spawn()
        };
        Ok(())
    }

    fn undo(&self) {}

    fn redo(&self) {}

    fn get_description(&self) -> String {
        String::from("A simple command that clears the terminal")
    }

    fn get_name(&self) -> String {
        String::from("clear")
    }

    fn get_help(&self) -> String {
        String::from("Clears the terminal")
    }

    fn get_params(&self) -> String {
        String::from("None")
    }
}

#[derive(Default)]
pub struct ExitCommand;

impl Command for ExitCommand {
    fn execute(&self, args: Option<Vec<String>>) -> Result<(), anyhow::Error> {
        match args {
            Some(args) => {
                let exit_code = args[0].parse()?;
                std::process::exit(exit_code);
                // Ok(())
            }
            None => {
                std::process::exit(0);
            }
        }
    }

    fn undo(&self) {
        todo!()
    }

    fn redo(&self) {
        todo!()
    }

    fn get_description(&self) -> String {
        String::from("Gracefully exists the program")
    }

    fn get_name(&self) -> String {
        String::from("exit")
    }

    fn get_help(&self) -> String {
        String::from("Exits, probably")
    }

    fn get_params(&self) -> String {
        String::from("None")
    }
}
#[derive(Default)]
pub struct ExecFile;

impl Command for ExecFile {
    fn execute(&self, args: Option<Vec<String>>) -> Result<(), anyhow::Error> {
        match args {
            Some(args) => {
                let file_path = PathBuf::from_str(&args[0])?;
                if file_path.extension().is_some() && file_path.extension().unwrap() != "zensh" {
                    return Err(anyhow!("Selected file was not a zensh file"));
                } else {
                    let zscript = fs::read_to_string(file_path)?;
                    if let Ok(command) = eval(zscript) {
                        println!("{:#?}", command);
                        for (cmd_name, cmd_args) in command {
                            match COMMAND_MANAGER.read().execute(&cmd_name, cmd_args) {
                                Ok(_) => (),
                                Err(e) => {
                                    println!(
                                        "Error executing command returned an error: {}. Aborting script",
                                        e
                                    );
                                    break;
                                }
                            }
                        }
                    }
                }
                Ok(())
            }
            None => Err(anyhow!("Not enough argumentss")),
        }
    }

    fn undo(&self) {}

    fn redo(&self) {}

    fn get_description(&self) -> String {
        String::from("Executes a file path")
    }

    fn get_name(&self) -> String {
        String::from("exec")
    }

    fn get_help(&self) -> String {
        String::from("this will read the contents of a .zensh file, evaluate it, and run its input")
    }

    fn get_params(&self) -> String {
        String::from("1: File path")
    }
}

#[derive(Default)]
pub struct CounterCommand {
    counter: RwLock<u32>,
}

impl Command for CounterCommand {
    fn execute(&self, _args: Option<Vec<String>>) -> Result<(), anyhow::Error> {
        // Increment the counter
        let mut count = self.counter.write();
        *count += 1;
        println!("CounterCommand executed. Current count: {}", *count);
        Ok(())
    }

    fn undo(&self) {
        println!("Undo CounterCommand.");
    }

    fn redo(&self) {
        println!("Redo CounterCommand.");
    }

    fn get_description(&self) -> String {
        String::from("counter")
    }

    fn get_help(&self) -> String {
        String::from("Increments a counter every time it's executed.")
    }

    fn get_params(&self) -> String {
        String::from("No parameters for CounterCommand.")
    }

    fn get_name(&self) -> String {
        String::from("count")
    }
}
#[derive(Default)]
pub struct PanicCommmand;
impl Command for PanicCommmand {
    fn execute(&self, args: Option<Vec<String>>) -> Result<(), anyhow::Error> {
        if args.is_some() {
            let panic_msg = &args.unwrap()[0];
            panic!("{}", panic_msg)
        }
        let option: Option<i32> = None;
        println!("Unwrapping None: {}", option.unwrap());
        panic!("Panic command was called")
    }

    fn undo(&self) {}

    fn redo(&self) {}

    fn get_description(&self) -> String {
        String::from("causes a panic with your provided message")
    }

    fn get_name(&self) -> String {
        String::from("panic")
    }

    fn get_help(&self) -> String {
        String::from("")
    }

    fn get_params(&self) -> String {
        String::from("optional: panic msg")
    }
}

fn eval(input: String) -> Result<Vec<(String, Option<Vec<String>>)>, anyhow::Error> {
    if input.trim().is_empty() {
        return Err(anyhow!("Input was empty"));
    }

    let pattern = Regex::new(r"[;|\n]").unwrap();
    let commands: Vec<&str> = pattern.split(&input).collect();
    let mut evaluted = vec![];

    for command in commands {
        let command = command.trim();
        if command.is_empty() {
            println!("Empty command, skipping.");
            continue;
        }

        let tokens = tokenize(command);
        if tokens.is_empty() {
            println!("Empty command, skipping.");
            continue;
        }
        let cmd_name = &tokens[0];
        let args: Option<Vec<String>> = if tokens.len() > 1 {
            Some(tokens[1..].iter().map(|s| s.to_string()).collect())
        } else {
            None
        };
        evaluted.push((cmd_name.to_owned(), args));
    }
    Ok(evaluted)
}
