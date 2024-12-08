use std::{
    borrow::Cow::{self, Borrowed, Owned},
    sync::Arc,
};


use chrono::Local;
use colored::Colorize;
use log::debug;
use parking_lot::Mutex;
use regex::Regex;
use rustyline::{
    error::ReadlineError, highlight::Highlighter, hint::HistoryHinter, history::DefaultHistory,
    Cmd, Completer, ConditionalEventHandler, Editor, Event, EventContext, EventHandler, Helper,
    Hinter, KeyEvent, RepeatCount, Validator,
};

use crate::{
    core::repl::{commands, Callable, COMMAND_LIST},
    utils::logger::LOGGER,
};

#[derive(Completer, Helper, Hinter, Validator)]
struct MyHelper(#[rustyline(Hinter)] HistoryHinter);

impl Highlighter for MyHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Owned(prompt.bright_black().bold().to_string())
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(hint.italic().bright_black().to_string())
    }
}

#[derive(Clone)]
struct BacktickEventHandler {
    toggle_state: Arc<Mutex<bool>>, // Tracks whether logging is enabled or disabled
}

impl ConditionalEventHandler for BacktickEventHandler {
    fn handle(&self, evt: &Event, _: RepeatCount, _: bool, _: &EventContext) -> Option<Cmd> {
        if let Some(k) = evt.get(0) {
            if *k == KeyEvent::from('`') {
                let mut state = self.toggle_state.lock();
                println!(
                    "Stdout Logging: {}",
                    if *state { "ON".green() } else { "OFF".red() }
                );
                if *state {
                    LOGGER.write_to_stdout();
                } else {
                    LOGGER.write_to_file("z.log");
                }
                *state = !*state;
                Some(Cmd::Noop)
            } else {
                None
            }
        } else {
            unreachable!()
        }
    }
}

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
    COMMAND_LIST.add_command(
        "exec",
        Some("Executes a .nyx file."),
        Callable::WithArgs(commands::exec),
        Some(1),
    );

    // Example of adding aliases for commands
    COMMAND_LIST.add_alias("clear".to_string(), "cls".to_string());
}

fn tokenize(command: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut inside_string = false;

    for char in command.chars() {
        if char == '"' || char == '\'' {
            inside_string = !inside_string;
        } else if char.is_whitespace() && !inside_string {
            if !current_token.is_empty() {
                tokens.push(current_token);
                current_token = String::new();
            }
        } else {
            current_token.push(char);
        }
    }

    // ignore the last token if it's empty. Who are we. Mojang? - Caz
    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}


pub fn parse_command(input: &str) -> anyhow::Result<Vec<String>> {
    let pattern = Regex::new(r"[;|\n]").unwrap();
    let commands: Vec<String> = pattern.split(input).map(|s| String::from(s)).collect();
    Ok(commands)
}
pub fn evaluate_command(input: &str) -> anyhow::Result<()> {
    if input.trim().is_empty() {
        println!("Empty command, skipping. type 'help' for a list of commands.");
        return Ok(());
    }

    let pattern = Regex::new(r"[;|\n]").unwrap();
    let commands: Vec<&str> = pattern.split(input).collect();

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
        let args: Vec<String> = tokens[1..].iter().map(|s| s.to_string()).collect();

        COMMAND_LIST.execute_command(
            cmd_name.to_string(),
            if args.is_empty() { None } else { Some(args) },
        )?;
    }
    Ok(())
}

pub async fn handle_repl() -> anyhow::Result<()> {
    let mut rl = Editor::<MyHelper, DefaultHistory>::new()?;
    rl.set_helper(Some(MyHelper(HistoryHinter::new())));

    rl.bind_sequence(
        KeyEvent::from('`'),
        EventHandler::Conditional(Box::new(BacktickEventHandler {
            toggle_state: Arc::new(Mutex::new(false)),
        })),
    );

    if rl.load_history("history.txt").is_err() {
        debug!("No previous history.");
    }

    register_commands();

    loop {
        let time = Local::now().format("%H:%M:%S.%3f").to_string();
        let prompt = format!("[{}/{}] {}", time, "SHELL", ">>\t");
        let sig = rl.readline(&prompt.bright_white());

        match sig {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                evaluate_command(line.as_str())?;
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL+C received, exiting...");
                std::process::exit(0);
            }
            Err(ReadlineError::Eof) => {
                println!("Error: CTRL+D pressed. Exiting...");
                std::process::exit(0);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
