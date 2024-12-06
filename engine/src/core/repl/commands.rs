use std::{ffi::OsStr, process::Command};





use crate::core::repl::repl::evaluate_command;

use super::COMMAND_LIST;

pub(crate) fn say_hello() -> anyhow::Result<()> {
    println!("Hello, World!");
    Ok(())
}

pub(crate) fn echo(args: Vec<String>) -> anyhow::Result<()> {
    println!("{}", args.join(" "));
    Ok(())
}

pub(crate) fn exit() -> anyhow::Result<()> {
    println!("Exiting...");
    std::process::exit(0)
}

pub(crate) fn clear() -> anyhow::Result<()> {
    println!("Clearing screen..., running command");
    let _result = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/c", "cls"]).spawn()
    } else {
        Command::new("clear").spawn()
    };
    Ok(())
}

pub(crate) fn help() -> anyhow::Result<()> {
    println!("Commands:");
    for cmd in COMMAND_LIST.commands.read().iter() {
        println!("{:#}", cmd);
    }
    Ok(())
}
pub(crate) fn exec(args: Vec<String>) -> anyhow::Result<()> {
    let file_path_str = &args[0];
    let file_path = std::path::Path::new(file_path_str);
    println!("File path: {:#?}", file_path);

    if !file_path.is_file() {
        eprintln!("Error: File does not exist or is not a valid file: {}", file_path.display());
        return Ok(()); 
    }
    if file_path.extension() != Some(OsStr::new("zensh")) {
        eprintln!("Error: File is not a zenshell script: {:#?}", file_path.extension());
        //TODO: dont panic on this error
        return Ok(());
    }
    println!("Executing file: {:#?}", file_path);
    let file_content = std::fs::read_to_string(file_path)?;
    if file_content.is_empty() {
        eprintln!("Error: file has no content. Is this a valid zenshell script?");
        return Ok(());
    }
    println!("File contents:\n{file_content}");
    evaluate_command(file_content.trim())?;
    Ok(())
}
