use anyhow::Result;
use clap::Parser;
use log::{info, warn, LevelFilter};

pub mod core;
pub mod utils;

use utils::{logger::LOGGER, splash::print_splash};

#[derive(Parser)]
struct Cli {
    #[arg(long, short, help = "Enable logging output")]
    log: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    log::set_logger(&*LOGGER).unwrap();
    log::set_max_level(LevelFilter::Debug);

    print_splash();

    if cli.log {
        info!("Initializing Engine with logging to stdout enabled");

        core::init_renderer()?;
    } else {
        LOGGER.write_to_stdout();
        info!("Initializing Engine with logging to stdout disabled");
        info!("Writing all logs to file z.log");

        LOGGER.write_to_file("z.log");
        info!("Logging back to file z.log");

        let shell_thread = tokio::task::spawn(async {
            core::repl::repl::handle_repl().await
        });

        core::init_renderer()?;
        shell_thread.await??; // LOL - Caz
    }

    Ok(())
}
