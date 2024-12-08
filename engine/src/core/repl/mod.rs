pub mod commands;
pub mod exec;

use std::{borrow::Borrow, collections::HashMap, sync::Arc};

use anyhow::Context;
use colored::Colorize;
use lazy_static::lazy_static;
use log::{debug, info};
use parking_lot::RwLock;



lazy_static! {
    pub static ref COMMAND_LIST: Arc<CommandList> = Arc::new(CommandList::new());
}

#[derive(Clone, Debug)]
enum Callable {
    Simple(fn() -> anyhow::Result<()>),
    WithArgs(fn(Vec<String>) -> anyhow::Result<()>),
}

#[derive(Debug)]
pub struct Command {
    pub name: &'static str,
    pub description: Option<&'static str>,
    function: Callable,
    pub arg_count: u8,
}
#[allow(private_interfaces)]
impl Command {
    pub fn new(
        name: &'static str,
        description: Option<&'static str>,
        function: Callable,
        arg_count: Option<u8>,
    ) -> Self {
        Command {
            name,
            description,
            function,
            arg_count: arg_count.unwrap_or(0),
        }
    }

    pub fn execute(&self, args: Option<Vec<String>>) -> anyhow::Result<()> {
        match &self.function {
            Callable::Simple(f) => {
                if let Some(args) = args {
                    eprintln!(
                        "Command expected 0 arguments but {} args were given. Ignoring..",
                        args.len()
                    );
                }
                f()?;
                Ok(())
            }
            Callable::WithArgs(f) => match args {
                Some(args) => f(args),
                None => Ok(()),
            },
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  {:<10} {}, {}",
            self.name,
            self.description.unwrap_or("No description available"),
            if self.arg_count > 0 {
                format!("{} args", self.arg_count)
            } else {
                "No args".to_string()
            }
        )
    }
}

pub struct CommandList {
    pub commands: RwLock<Vec<Command>>,
    pub aliases: RwLock<HashMap<String, String>>,
}

fn hamming_distance(a: &str, b: &str) -> Option<usize> {
    if a.len() != b.len() {
        return None;
    }
    Some(
        a.chars()
            .zip(b.chars())
            .filter(|(char_a, char_b)| char_a != char_b)
            .count(),
    )
}

fn edit_distance(a: &str, b: &str) -> usize {
    let m = a.len();
    let n = b.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        for j in 0..=n {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else if a.chars().nth(i - 1) == b.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]);
            }
        }
    }

    dp[m][n]
}

fn check_similarity(target: &str, strings: &[String]) -> Option<String> {
    let max_hamming_distance: usize = 2;
    let max_edit_distance: usize = 2;
    let mut best_match: Option<String> = None;
    let mut best_distance = usize::MAX;

    for s in strings {
        if let Some(hamming_dist) = hamming_distance(target, s) {
            if hamming_dist <= max_hamming_distance && hamming_dist < best_distance {
                best_distance = hamming_dist;
                best_match = Some(s.clone());
            }
        } else {
            let edit_dist = edit_distance(target, s);
            if edit_dist <= max_edit_distance && edit_dist < best_distance {
                best_distance = edit_dist;
                best_match = Some(s.clone());
            }
        }
    }

    best_match
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

    fn execute_command(&self, mut name: String, args: Option<Vec<String>>) -> anyhow::Result<()> {
        let commands = self.commands.borrow();
        if self.aliases.read().contains_key(&name) {
            name = self
                .aliases
                .read()
                .get_key_value(&name)
                .context("Failed to get alias")?
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
                    Ok(())
                }
                (expected, None) => {
                    eprintln!(
                        "Command: '{}' expected {} arguments but received none",
                        name,
                        expected
                    );
                    Ok(())
                }
                (_, _) => command.execute(args),
            }
        } else {
            eprintln!("Command: '{}' was not found", name.red().italic());

            let most_similar = check_similarity(
                &name,
                &self
                    .commands
                    .read()
                    .iter()
                    .map(|cmd| cmd.name.to_string())
                    .collect::<Vec<String>>(),
            );
            match most_similar {
                Some(similar) => {
                    eprintln!("Did you mean: '{}'?", similar.green().italic().bold());
                    Ok(())
                }
                None => {
                    println!("Type 'help' for a list of commands");
                    Ok(())
                },
            }
        }
    }
}
