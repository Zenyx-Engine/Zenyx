use commands::{
    ClearCommand, CounterCommand, ExecFile, ExitCommand, HelpCommand, PanicCommmand, ZLua,
};
use handler::{COMMAND_MANAGER, Category};

use crate::commands;

pub mod commands;
pub mod handler;
pub mod input;

pub fn setup() {
    commands!(
        HelpCommand,
        ClearCommand,
        ExitCommand,
        CounterCommand,
        PanicCommmand,
        ZLua
    );
    let cat = Category::new("cr", "Core", "Core commands");
    COMMAND_MANAGER.write().add_category(cat.clone());
    COMMAND_MANAGER
        .write()
        .add_command_with_category(Box::new(ExecFile), cat.clone());
}
