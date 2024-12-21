use std::collections::HashMap;
use colored::Colorize;
use lazy_static::lazy_static;
use parking_lot::RwLock;
lazy_static! {
    pub static ref COMMAND_MANAGER: RwLock<CommandManager> = RwLock::new(CommandManager::init());
}
#[macro_export]
macro_rules! commands {
    [$($command:ty),*] => [
        $(
            {
                let mut manager = $crate::core::repl::handler::COMMAND_MANAGER.write();
                manager.add_command(Box::new(<$command>::default()));
            }
        )*
    ];
}

#[macro_export]
macro_rules! alias {
    ($($alias:expr => $command:expr),*) => {
        $(
            {
                let mut manager = $crate::COMMAND_MANAGER.write();
                manager.add_alias($alias, $command);
            }
        )*
    };
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

fn check_similarity(target: &str) -> Option<String> {
    let max_hamming_distance: usize = 2;
    let max_edit_distance: usize = 2;
    let mut best_match: Option<String> = None;
    let mut best_distance = usize::MAX;

    for (cmd_name, _) in COMMAND_MANAGER.read().get_commands() {
        if let Some(hamming_dist) = hamming_distance(target, cmd_name) {
            if hamming_dist <= max_hamming_distance && hamming_dist < best_distance {
                best_distance = hamming_dist;
                best_match = Some(String::from(cmd_name));
            }
        } else {
            let edit_dist = edit_distance(target, cmd_name);
            if edit_dist <= max_edit_distance && edit_dist < best_distance {
                best_distance = edit_dist;
                best_match = Some(String::from(cmd_name));
            }
        }
    }

    best_match
}

pub struct CommandManager {
    pub commands: HashMap<String, Box<dyn Command>>,
    pub aliases: HashMap<String, String>,
    pub categories: HashMap<String, Category>,
}

impl CommandManager {
    pub fn init() -> CommandManager {
        CommandManager {
            commands: HashMap::new(),
            aliases: HashMap::new(),
            categories: HashMap::new(),
        }
    }

    pub fn add_category(&mut self, category: Category) {
        self.categories.insert(category.name.clone(), category);
    }

    pub fn get_commands(&self) -> std::collections::hash_map::Iter<'_, String, Box<dyn Command>> {
        self.commands.iter()
    }

    pub fn execute_command(
        &self,
        command: &str,
        args: Option<Vec<String>>,
    ) -> Result<(), anyhow::Error> {
        if let Some(command) = self.commands.get(command) {
            command.execute(args)?;
            Ok(())
        } else {
            let corrected_cmd = check_similarity(command);
            if corrected_cmd.is_some() {
                println!("Command: {} was not found. Did you mean {}?",command.red().bold(),corrected_cmd
                    .expect("A command was editied during execution, something has gone seriously wrong").green().bold().italic());
            }
            Err(anyhow::anyhow!("Command '{}' not found.", command))
        }
    }

    pub fn execute(&self, command: &str, args: Option<Vec<String>>) -> Result<(), anyhow::Error> {
        match self.aliases.get(command) {
            Some(command) => self.execute(command, args),
            // check to see if we are using an alias or the command just doesnt exist
            None => {
                self.execute_command(command, args)?;
                Ok(())
            }
        }
    }

    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands
            .insert(command.get_name().to_lowercase(), command);
    }

    pub fn add_command_with_category(&mut self, command: Box<dyn Command>, category: Category) {
        if self.categories.contains_key(&category.name) {
            let mut cmd_name = command.get_name().to_lowercase();
            cmd_name.insert_str(0, &format!("{}_", &&category.uid.to_lowercase()));
            println!("{}", cmd_name);
            self.commands.insert(cmd_name, command);
        } else {
            panic!("Category {} does not exist", category.name);
        }
    }

    pub fn add_alias(&mut self, alias: &str, command: &str) {
        self.aliases.insert(
            alias.to_string().to_lowercase(),
            command.to_string().to_lowercase(),
        );
    }
}
#[derive(Debug, Clone)]
pub struct Category {
    // eg:  Zenyx -> Z
    // eg: core -> cr
    // eg: exitcmd -> cr_exit
    // eg:  echo -> z_echo
    pub uid: String,
    // eg: Zenyx
    pub name: String,
    // eg: Zenyx internal commands
    pub description: String,
}

impl Category {
    pub fn new(uid: &str, name: &str, description: &str) -> Self {
        Self {
            uid: uid.to_string(),
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

pub trait Command: Send + Sync {
    fn execute(&self, args: Option<Vec<String>>) -> Result<(), anyhow::Error>;
    fn undo(&self);
    fn redo(&self);
    fn get_description(&self) -> String;
    fn get_name(&self) -> String;
    fn get_help(&self) -> String;
    fn get_params(&self) -> String;
}
