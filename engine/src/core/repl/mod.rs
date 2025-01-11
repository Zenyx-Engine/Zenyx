use commands::{ClearCommand, CounterCommand, ExecFile, ExitCommand, HelpCommand, PanicCommmand};

use crate::commands;

pub mod commands;
pub mod handler;
pub mod input;
pub mod zlua;

pub fn setup() {
    commands!(
        HelpCommand,
        ExecFile,
        ClearCommand,
        ExitCommand,
        CounterCommand,
        PanicCommmand,
        zlua::ZLua
    );
}
