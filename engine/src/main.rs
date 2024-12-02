use anyhow::Result;
use log2::info;

pub mod core;

#[tokio::main]
async fn main() -> Result<()> {
    let _log2 = log2::open("z.log").tee(true).level("debug").start();
    info!("Initalizing Engine");
    let shell_thread = tokio::task::spawn(async {
        info!("Shell thread started");
        core::repl::handle_repl().await;
    });

    core::splash::print_splash();
    info!("Engine Initalized");
    core::init_renderer()?;
    shell_thread.await?;
    Ok(())
}
