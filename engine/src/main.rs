#![deny(clippy::unwrap_in_result)]

use anyhow::Result;
use log::LevelFilter;

pub mod core;
pub mod utils;

use utils::{logger::LOGGER, splash::print_splash};

#[tokio::main]
async fn main() -> Result<()> {
    let t = zephyr::add(0, 2);
    println!("{}", t);

    log::set_logger(&*LOGGER).ok();
    log::set_max_level(LevelFilter::Debug);

    print_splash();

    LOGGER.write_to_stdout();

    let shell_thread = tokio::task::spawn(async { core::repl::repl::handle_repl().await });

    core::init_renderer()?;
    let _ = shell_thread.await?;

    Ok(())
}
