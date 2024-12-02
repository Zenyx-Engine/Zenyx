use super::{commands, Callable, COMMAND_LIST};
use anyhow::Result;
use chrono::Local;
use colored::Colorize;
use log::debug;
use regex::Regex;
use rustyline::DefaultEditor;

fn register_commands() {
    COMMAND_LIST.add_command(
        "hello",
        Some("Displays \"Hello World\"!"),
        Callable::Simple(commands::say_hello),
        None,
    );

    COMMAND_LIST.add_command(
        "exit",
        Some("Exits the application gracefully."),
        Callable::Simple(commands::exit),
        None,
    );

    COMMAND_LIST.add_command(
        "clear",
        Some("Clears the terminal screen."),
        Callable::Simple(commands::clear),
        None,
    );

    COMMAND_LIST.add_command(
        "echo",
        Some("Prints the provided arguments back to the terminal."),
        Callable::WithArgs(commands::echo),
        Some(1), // Requires at least one argument
    );

    COMMAND_LIST.add_command(
        "help",
        Some("Displays a list of all available commands."),
        Callable::Simple(commands::help),
        None,
    );

    // EXAMPLE
    // Adding aliases for commands
    COMMAND_LIST.add_alias("clear".to_string(), "cls".to_string());
}



fn evaluate_command(input: &str) {
    if input.trim().is_empty() {
        return;
    }

    let pattern = Regex::new(r"[;|\n]").unwrap();
    let commands: Vec<&str> = pattern.split(input).collect();

    for command in commands {
        let command = command.trim();
        if command.is_empty() {
            println!("Empty command, skipping.");
            continue;
        }

        let tokens: Vec<&str> = command.split_whitespace().collect();
        if tokens.is_empty() {
            return;
        }

        let cmd_name = tokens[0];
        let args: Vec<String> = tokens[1..].iter().map(|&s| s.to_string()).collect();

        COMMAND_LIST.execute_command(
            cmd_name.to_string(),
            if args.is_empty() { None } else { Some(args) },
        );
    }
}

pub async fn handle_repl() -> rustyline::Result<()> {
    let mut line_editor = DefaultEditor::new()?;
    if line_editor.load_history("history.txt").is_err() {
        debug!("No previous history.");
    }
    let time = Local::now().format("%H:%M:%S.%3f").to_string();
    let prompt = format!("[{}/{}] {}", time,"SHELL", ">>\t");
    register_commands();

    loop {
        let sig = line_editor.readline(
            &prompt.bright_white()
        );
        match sig {
            Ok(line) => {
                line_editor.add_history_entry(line.as_str())?;
                evaluate_command(line.as_str());
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("CTRL+C received, exiting...");
                std::process::exit(0);
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("Error: CTRL+D pressed. Exiting...");
                std::process::exit(0);
            }
            Err(err) => {
                println!("Error: {}", err);
                
            }
        }
    }
}
