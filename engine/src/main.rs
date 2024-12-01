use std::io;

use log2::info;

pub mod core;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let _log2 = log2::open("z.log").tee(true).level("trace").start();
    info!("Initalizing Engine");
    let shell_thread = tokio::task::spawn(async {
        info!("Shell thread started");
        core::repl::handle_repl().await;
    }
);

    core::splash::print_splash();
    info!("Engine Initalized");
    shell_thread.await?;
    Ok(())
}
