use std::process::Command;

use log2::{debug, info};

use crate::core::repl::COMMAND_LIST;

pub fn say_hello() {
    println!("Hello from your new command!");
}

pub fn echo(args: Vec<String>) {
    debug!("{}", args.join(" "));
    println!("{}", args.join(" "))
}

pub fn exit() {
    debug!("Exiting...");
    std::process::exit(0)
}
pub fn clear() {
    info!("Clearing screen..., running command");
    let _result = if cfg!(target_os = "windows") {
        debug!("target_os is windows");
        Command::new("cmd").args(["/c", "cls"]).spawn()
    } else {
        debug!("target_os was unix");
        // "clear" or "tput reset"
        Command::new("tput").arg("reset").spawn()
    };
}
pub fn cmds() {
    println!("Commands:");
    for cmd in COMMAND_LIST.commands.read().iter() {
        println!("{:#}", cmd);
    }
}
