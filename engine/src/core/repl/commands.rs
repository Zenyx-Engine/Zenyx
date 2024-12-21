use std::{fs, path::PathBuf, str::FromStr};
use colored::Colorize;
use mlua::prelude::*;
use anyhow::anyhow;
use mlua::{Lua, MultiValue};
use parking_lot::RwLock;
use regex::Regex;
use rustyline::{error::ReadlineError, DefaultEditor};

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
#[derive(Default)]
pub struct ZLua;

impl Command for ZLua {
    fn execute(&self, args: Option<Vec<String>>) -> Result<(), anyhow::Error> {
        let time = chrono::Local::now().format("%H:%M:%S.%3f").to_string();
        let prompt = format!("[{}/{}] {}", time, "ZLUA", ">>\t");
        let lua = Lua::new();
        let globals = lua.globals();
        let sum = lua.create_function(|_, (list1, list2): (i32, i32)| {
            // This function just checks whether two string lists are equal, and in an inefficient way.
            // Lua callbacks return `mlua::Result`, an Ok value is a normal return, and an Err return
            // turns into a Lua 'error'. Again, any type that is convertible to Lua may be returned.
            Ok(list1 == list2)
        })?;
        globals.set("sum", sum)?;
        let log = lua.create_function(|_, (msg,): (String,)| {
            println!("{}", msg);
            Ok(())
        })?;
        globals.set("log", log)?;
        let mut editor = DefaultEditor::new().expect("Failed to create editor");

        loop {
            let mut prompt = &prompt;
            let mut line = String::new();

            loop {
                match editor.readline(prompt) {
                    Ok(input) => line.push_str(&input),
                    Err(ReadlineError::Interrupted) => {
                        println!("Exiting ZLUA shell...");
                        return Ok(());
                    }
                    Err(_) => {}
                }

                match lua.load(&line).eval::<MultiValue>() {
                    Ok(values) => {
                        editor.add_history_entry(line).unwrap();
                        println!(
                            "{}",
                            values
                                .iter()
                                .map(|value| format!("{:#?}", value))
                                .collect::<Vec<_>>()
                                .join("\t")
                        );
                        break;
                    }
                    Err(mlua::Error::SyntaxError {
                        incomplete_input: true,
                        ..
                    }) => {
                        // continue reading input and append it to `line`
                        line.push_str("\n"); // separate input lines
                        prompt = prompt;
                    }

                    Err(e) => {
                        eprintln!("error: {}", e);
                        break;
                    }
                }
            }
        }
    }

    fn undo(&self) {}

    fn redo(&self) {}

    fn get_description(&self) -> String {
        String::from("Runs the ZLua interpreter")
    }

    fn get_name(&self) -> String {
        String::from("zlua")
    }

    fn get_help(&self) -> String {
        String::from("zlua")
    }

    fn get_params(&self) -> String {
        String::from("No parameters required.")
    }
}



