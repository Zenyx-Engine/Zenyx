pub mod commands;
pub mod repl;

use std::{borrow::Borrow, collections::HashMap, sync::Arc};

use colored::Colorize;
use lazy_static::lazy_static;
use log::{debug, info};
use parking_lot::RwLock;

lazy_static! {
    pub static ref COMMAND_LIST: Arc<CommandList> = Arc::new(CommandList::new());
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
        match &self.function {
            Callable::Simple(f) => {
                if let Some(args) = args {
                    eprintln!(
                        "Command expected 0 arguments but {} args were given. Ignoring..",
                        args.len()
                    );
                }
                f()
            }
            Callable::WithArgs(f) => match args {
                Some(args) => f(args),
                None => eprintln!("Command expected arguments but received 0"),
            },
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  {:<10} {}",
            self.name,
            self.description.unwrap_or("No description available")
        )
    }
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
        info!("Adding command: {}", name);
        let mut commands = self.commands.write();

        commands.push(Command {
            name,
            description,
            function: func,
            arg_count: arg_count.unwrap_or(0),
        });
    }

    fn add_alias(&self, name: String, alias: String) {
        if self.aliases.read().contains_key(&alias) {
            eprintln!("Alias: '{}' already exists", alias);
            return;
        }
        let mut commands = self.commands.write();
        if let Some(command) = commands.iter_mut().find(|cmd| cmd.name == name) {
            debug!("Adding alias: {} for cmd: {}", alias, command.name);
            self.aliases
                .write()
                .insert(alias.to_string(), name.to_string());
        } else {
            eprintln!("Command: '{}' was not found", name);
        }
    }

    fn execute_command(&self, mut name: String, args: Option<Vec<String>>) {
        let commands = self.commands.borrow();
        if self.aliases.read().contains_key(&name) {
            name = self
                .aliases
                .read()
                .get_key_value(&name)
                .unwrap()
                .1
                .to_string();

            debug!("changed to {}", &name);
        }
        if let Some(command) = commands.read().iter().find(|cmd| cmd.name == name) {
            match (command.arg_count, args.as_ref()) {
                (expected, Some(args_vec)) if args_vec.len() != expected as usize => {
                    eprintln!(
                        "Command: '{}' expected {} arguments but received {}",
                        name,
                        expected,
                        args_vec.len()
                    );
                }
                (_, _) => command.execute(args),
            }
        } else {
            eprintln!("Command: '{}' was not found", name.red().italic());
        }
    }
}
