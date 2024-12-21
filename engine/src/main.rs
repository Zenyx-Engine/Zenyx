#![feature(panic_payload_as_str)]
use core::{
    panic::set_panic_hook,
    repl::{handler::COMMAND_MANAGER, setup},
    splash, workspace,
};

use colored::Colorize;
use mlua::Lua;
use tokio::runtime;

pub mod core;

fn main() -> anyhow::Result<()> {
    if !cfg!(debug_assertions) {
        println!("{}", "Debug mode disabled".bright_blue());
        set_panic_hook();
    }
    let runtime = runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    runtime.block_on(async {
        setup();
        splash::print_splash();
        COMMAND_MANAGER.read().execute("help", None)?;
        let t = tokio::spawn(core::repl::input::handle_repl());
        t.await??;
        Ok::<(), anyhow::Error>(())
    })?;

    Ok(())
}
