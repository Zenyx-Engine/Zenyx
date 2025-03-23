use core::{
    panic::set_panic_hook,
    repl::setup,
    splash, workspace,
};

use colored::Colorize;
use log::info;
use tokio::runtime;
use winit::event_loop::EventLoop;

pub mod core;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    if !cfg!(debug_assertions) {
        println!("{}", "Debug mode disabled".bright_blue());
        set_panic_hook();
    }
    setup();
    splash::print_splash();
    info!("Type 'help' for a list of commands.");

    let repl_thread = std::thread::spawn(|| {
        let rt = runtime::Builder::new_current_thread().enable_all().build().unwrap();
        rt.block_on(core::repl::input::handle_repl())
    });

    let event_loop = EventLoop::new().unwrap();
    core::render::init_renderer(event_loop);

    if let Err(_) = repl_thread.join() {
        eprintln!("REPL thread panicked");
    }
    Ok(())
}
