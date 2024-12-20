use commands::{ClearCommand, CounterCommand, ExecFile, ExitCommand, HelpCommand};

use crate::commands;

pub mod commands;
pub mod input;
pub mod handler;


pub fn setup() {
    commands!(HelpCommand,ClearCommand,ExitCommand,ExecFile,CounterCommand);
}
