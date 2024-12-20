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
    Cmd, Completer, ConditionalEventHandler, Editor, Event, EventContext, EventHandler, Helper,
    Hinter, KeyEvent, RepeatCount, Validator, completion::Completer, error::ReadlineError,
    highlight::Highlighter, hint::HistoryHinter, history::DefaultHistory,
};

use super::handler::COMMAND_MANAGER;
use crate::core::logger::LOGGER;

struct CommandCompleter;
impl CommandCompleter {
    fn new() -> Self {
        CommandCompleter {}
    }
}

impl Completer for CommandCompleter {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let binding = COMMAND_MANAGER.read();
        let binding = binding.get_commands();
        let filtered_commands: Vec<_> = binding
            .filter(|(command, _)| command.starts_with(line))
            .collect();

        let completions: Vec<String> = filtered_commands
            .iter()
            .filter(|(command, _)| command.starts_with(&line[..pos]))
            .map(|(command, _)| command[pos..].to_string())
            .collect();
        println!("{:#?}", completions);
        Ok((pos, completions))
    }
}
#[derive(Completer, Helper, Hinter, Validator)]
struct MyHelper {
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    #[rustyline(Completer)]
    completer: CommandCompleter,
}

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

pub fn tokenize(command: &str) -> Vec<String> {
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
    let commands: Vec<String> = pattern.split(input).map(String::from).collect();
    Ok(commands)
}
pub fn evaluate_command(input: &str) -> anyhow::Result<()> {
    if input.trim().is_empty() {
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
        let args: Option<Vec<String>> = if tokens.len() > 1 {
            Some(tokens[1..].iter().map(|s| s.to_string()).collect())
        } else {
            None
        };
            match COMMAND_MANAGER.read().execute(cmd_name, args) {
                Ok(_) => continue,
                Err(e) => return Err(e)
            }
    }
    Ok(())
}

pub async fn handle_repl() -> anyhow::Result<()> {
    let mut rl = Editor::<MyHelper, DefaultHistory>::new()?;
    rl.set_helper(Some(MyHelper {
        hinter: HistoryHinter::new(),
        completer: CommandCompleter::new(),
    }));

    rl.bind_sequence(
        KeyEvent::from('`'),
        EventHandler::Conditional(Box::new(BacktickEventHandler {
            toggle_state: Arc::new(Mutex::new(false)),
        })),
    );

    if rl.load_history("history.txt").is_err() {
        debug!("No previous history.");
    }

    loop {
        let time = Local::now().format("%H:%M:%S.%3f").to_string();
        let prompt = format!("[{}/{}] {}", time, "SHELL", ">>\t");
        let sig = rl.readline(&prompt.bright_white());

        match sig {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                match evaluate_command(line.as_str()) {
                    Ok(_) => continue,
                    Err(e) => println!("{e}"),
                }
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
