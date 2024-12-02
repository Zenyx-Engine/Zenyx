use anyhow::Result;
use colored::Colorize;
use log2::info;

pub mod core;

pub fn print_splash() {
    println!(
        r#"
         &&&&&&&&&&&         
      &&&&&&&&&&&&&&&&&      
    &&&&&&&&&&&&&&&&&&&&&    
  &&              &&&&&&&&&  
 &&                &&&&&&&&& 
&&&&&&&&&&&&      &&&&&&&&&&&
&&&&&&&&&&&&&    &&&&&&&&&&&&
&&&&&&&&&&&&&   &&&&&&&&&&&&&
&&&&&&&&&&&&    &&&&&&&&&&&&&
&&&&&&&&&&&      &&&&&&&&&&&&
 &&&&&&&&&                && 
  &&&&&&&&&              &&  
    &&&&&&&&&&&&&&&&&&&&&    
      &&&&&&&&&&&&&&&&&      
         &&&&&&&&&&&

        Version: {}
    "#,
        env!("CARGO_PKG_VERSION").yellow().italic().underline()
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    let _log2 = log2::open("z.log").tee(true).level("debug").start();
    info!("Initalizing Engine");
    let shell_thread = tokio::task::spawn(async {
        info!("Shell thread started");
        core::repl::repl::handle_repl().await;
    });

    print_splash();
    info!("Engine Initalized");
    core::init_renderer()?;
    shell_thread.await?;
    Ok(())
}
