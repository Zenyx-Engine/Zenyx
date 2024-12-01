use super::commands;
use chrono::Local;
use lazy_static::lazy_static;
use log2::{debug, error, info};
use parking_lot::RwLock;
use reedline::{Prompt, Reedline, Signal};
use regex::Regex;
use std::{borrow::Borrow, collections::HashMap, sync::Arc};

struct ZPrompt {
    left_text: String,
    right_text: String,
}

#[derive(Clone, Debug)]
enum Callable {
    Simple(fn()),
    WithArgs(fn(Vec<String>)),
}
#[derive(Debug)]
pub struct Command {
    pub name: &'static str,
    pub description: Option<&'static str>,
    function: Callable,
    pub arg_count: u8,
}

impl Command {
    pub fn execute(&self, args: Option<Vec<String>>) {
        //debug!("Executing command: {}", self.name);
        match &self.function {
            Callable::Simple(f) => {
                if let Some(args) = args {
                    error!(
                        "Command expected 0 arguments but {} args were given. Ignoring..",
                        args.len()
                    );
                }
                f()
            }
            Callable::WithArgs(f) => match args {
                Some(args) => f(args),
                None => error!("Command expected arguments but received 0"),
            },
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\n\t{}",
            self.name,
            self.description.unwrap_or("No description")
        )
    }
}

lazy_static! {
    pub static ref COMMAND_LIST: Arc<CommandList> = Arc::new(CommandList::new());
}

pub struct CommandList {
    pub commands: RwLock<Vec<Command>>,
    pub aliases: RwLock<HashMap<String, String>>,
}

impl CommandList {
    fn new() -> Self {
        CommandList {
            commands: RwLock::new(Vec::new()),
            aliases: RwLock::new(HashMap::new()),
        }
    }

    fn add_command(
        &self,
        name: &'static str,
        description: Option<&'static str>,
        func: Callable,
        arg_count: Option<u8>,
    ) {
        debug!("Adding command: {}", name);
        let mut commands = self.commands.write();

        commands.push(Command {
            name,
            description,
            function: func,
            arg_count: arg_count.unwrap_or(0),
        });
    }
    fn add_alias(&self, name: String, alias: String) {
        //println!("Input alias: {}", alias);
        if self.aliases.read().contains_key(&alias) {
            error!("Alias: '{}' already exists", alias);
            return;
        }
        let mut commands = self.commands.write();
        if let Some(command) = commands.iter_mut().find(|cmd| cmd.name == name) {
            info!("Adding alias: {} for cmd: {}", alias, command.name);
            self.aliases
                .write()
                .insert(alias.to_string(), name.to_string());
        } else {
            error!("Command: '{}' was not found", name);
        }
    }

    fn execute_command(&self, mut name: String, args: Option<Vec<String>>) {
        //info!("received input command: {}", name);
        let commands = self.commands.borrow();
        if self.aliases.read().contains_key(&name) {
            name = self
                .aliases
                .read()
                .get_key_value(&name)
                .unwrap()
                .1
                .to_string();
            debug!("changed to {}", name);
        }
        if let Some(command) = commands.read().iter().find(|cmd| cmd.name == name) { match (command.arg_count, args.as_ref()) {
            (expected, Some(args_vec)) if args_vec.len() != expected as usize => {
                eprintln!(
                    "Command: '{}' expected {} arguments but received {}",
                    name,
                    expected,
                    args_vec.len()
                );
            }
            (_, _) => command.execute(args),
        } }
    }
}

impl Prompt for ZPrompt {
    fn render_prompt_left(&self) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed(&self.left_text)
    }

    fn render_prompt_right(&self) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed(&self.right_text)
    }

    fn render_prompt_history_search_indicator(
        &self,
        _history_search: reedline::PromptHistorySearch,
    ) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed("")
    }

    fn render_prompt_indicator(
        &self,
        prompt_mode: reedline::PromptEditMode,
    ) -> std::borrow::Cow<str> {
        match prompt_mode {
            reedline::PromptEditMode::Default => std::borrow::Cow::Borrowed(">>"),
            reedline::PromptEditMode::Emacs => {
                let timestamp = Local::now().format("[%H:%M:%S.%3f/SHELL] >>\t").to_string();
                std::borrow::Cow::Owned(timestamp)
            }
            reedline::PromptEditMode::Vi(_) => std::borrow::Cow::Borrowed("vi>>"),
            reedline::PromptEditMode::Custom(_) => std::borrow::Cow::Borrowed("custom>>"),
        }
    }

    fn render_prompt_multiline_indicator(&self) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed("><")
    }
}

fn setup() {
    COMMAND_LIST.add_command(
        "hello",
        Some("test"),
        Callable::Simple(commands::say_hello),
        None,
    );
    COMMAND_LIST.add_command("exit", None, Callable::Simple(commands::exit), None);
    COMMAND_LIST.add_command("clear", None, Callable::Simple(commands::clear), None);
    COMMAND_LIST.add_command("echo", None, Callable::WithArgs(commands::echo), Some(1));
    COMMAND_LIST.add_command("cmds", None, Callable::Simple(commands::cmds), None);
    COMMAND_LIST.add_alias("cmds".to_string(), "help".to_string());
    COMMAND_LIST.add_alias("cmds".to_string(), "cmd_list".to_string());
    COMMAND_LIST.add_alias("hello".to_string(), "exit".to_string());
    COMMAND_LIST.add_alias("clear".to_string(), "exit".to_string());
}

pub async fn handle_repl() {
    let mut line_editor = Reedline::create();
    setup();

    loop {
        let sig = line_editor.read_line(&ZPrompt {
            left_text: String::new(),
            right_text: "<<".to_string(),
        });

        match sig {
            Ok(Signal::Success(buffer)) => {
                if buffer == "exit" {
                    std::process::exit(0);
                } else {
                    evaluate_command(&buffer);
                }
            }
            Ok(Signal::CtrlC) => {
                println!("\nCONTROL+C RECEIVED, TERMINATING");
                std::process::exit(0);
            }
            err => {
                eprintln!("Error: {:?}", err);
            }
        }
    }
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
