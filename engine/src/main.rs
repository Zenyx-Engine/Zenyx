#![feature(panic_payload_as_str)]
use core::{
    panic::set_panic_hook,
    repl::{handler::COMMAND_MANAGER, setup},
    splash, workspace,
};

use colored::Colorize;
use log::info;
use mlua::Lua;
use parking_lot::Mutex;
use tokio::runtime;
use winit::event_loop::EventLoop;

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
        info!("Type 'help' for a list of commands.");

        let repl_handle = tokio::spawn(core::repl::input::handle_repl());
        let event_loop = EventLoop::new().unwrap();

            core::render::init_renderer(event_loop);


        // Await the REPL
        if let Err(e) = repl_handle.await {
            eprintln!("REPL error: {:?}", e);
        }

        // Wait for the renderer to finish (if needed)


        Ok::<(), anyhow::Error>(())
    })?;

    Ok(())
}
