use super::COMMAND_LIST;
use std::process::Command;

pub(crate) fn say_hello() {
    println!("Hello, World!");
}

pub(crate) fn echo(args: Vec<String>) {
    println!("{}", args.join(" "))
}

pub(crate) fn exit() {
    println!("Exiting...");
    std::process::exit(0)
}

pub(crate) fn clear() {
    println!("Clearing screen..., running command");
    let _result = if cfg!(target_os = "windows") {
        dbg!("target_os is windows");
        Command::new("cmd").args(["/c", "cls"]).spawn()
    } else {
        dbg!("target_os is unix");
        Command::new("clear").spawn()
    };
}

pub(crate) fn help() {
    println!("Commands:");
    for cmd in COMMAND_LIST.commands.read().iter() {
        println!("{:#}", cmd);
    }
}
