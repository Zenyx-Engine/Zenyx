use commands::{
    ClearCommand, CounterCommand, ExitCommand, HelpCommand, PanicCommmand,ExecFile
};

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
