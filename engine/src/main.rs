use core::{repl::{handler::COMMAND_MANAGER, input::handle_repl, setup}, splash};

use anyhow::Ok;


pub mod core;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    setup();
    splash::print_splash();
    COMMAND_MANAGER.read().execute("help", None)?;
    let t = tokio::spawn(handle_repl());

    t.await??;

    Ok(())
    
}
