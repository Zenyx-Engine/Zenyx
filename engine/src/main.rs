#![deny(clippy::unwrap_in_result)]
#![feature(iter_collect_into)]
use anyhow::Result;
use log::LevelFilter;
use plugin_api::plugin_imports::*;
use plugin_api::{get_plugin, PluginManager};

pub mod core;
pub mod utils;

use utils::{logger::LOGGER, splash::print_splash};

#[tokio::main]
async fn main() -> Result<()> {
    // Load all plugins

    log::set_logger(&*LOGGER).ok();
    log::set_max_level(LevelFilter::Off);

    print_splash();
    let mut plugin_manager = PluginManager::new();
    let plugins = plugin_manager.load_all();
    println!("Plugins loaded: {:?}", plugins);

    // Get the player plugin
    let player_lib = get_plugin!(player_lib, plugins);
    player_lib.test();

    LOGGER.write_to_stdout();

    let shell_thread = tokio::task::spawn(async { core::repl::exec::handle_repl().await });

    core::init_renderer()?;
    shell_thread.await??;

    Ok(())
}
