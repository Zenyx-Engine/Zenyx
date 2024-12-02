use super::{commands, Callable, COMMAND_LIST};
use chrono::Local;
use reedline::{Prompt, Reedline, Signal};
use regex::Regex;
use std::{borrow::Borrow, collections::HashMap, sync::Arc};

fn register_commands() {
    COMMAND_LIST.add_command(
        "hello",
        Some("Displays \"Hello World\"!"),
        Callable::Simple(commands::say_hello),
        None,
    );

    COMMAND_LIST.add_command(
        "exit",
        Some("Exits the application gracefully."),
        Callable::Simple(commands::exit),
        None,
    );

    COMMAND_LIST.add_command(
        "clear",
        Some("Clears the terminal screen."),
        Callable::Simple(commands::clear),
        None,
    );

    COMMAND_LIST.add_command(
        "echo",
        Some("Prints the provided arguments back to the terminal."),
        Callable::WithArgs(commands::echo),
        Some(1), // Requires at least one argument
    );

    COMMAND_LIST.add_command(
        "help",
        Some("Displays a list of all available commands."),
        Callable::Simple(commands::help),
        None,
    );

    // EXAMPLE
    // Adding aliases for commands
    COMMAND_LIST.add_alias("cls".to_string(), "clear".to_string()); // Likely unintended; consider removing or renaming.
}

struct ZPrompt {
    left_text: String,
    right_text: String,
}

impl Prompt for ZPrompt {
    fn render_prompt_left(&self) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed(&self.left_text)
    }

    fn render_prompt_right(&self) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed(&self.right_text)
    }

    fn render_prompt_history_search_indicator(
        &self,
        _history_search: reedline::PromptHistorySearch,
    ) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed("")
    }

    fn render_prompt_indicator(
        &self,
        prompt_mode: reedline::PromptEditMode,
    ) -> std::borrow::Cow<str> {
        match prompt_mode {
            reedline::PromptEditMode::Default => std::borrow::Cow::Borrowed(">>"),
            reedline::PromptEditMode::Emacs => {
                let timestamp = Local::now().format("[%H:%M:%S.%3f/SHELL] >>\t").to_string();
                std::borrow::Cow::Owned(timestamp)
            }
            reedline::PromptEditMode::Vi(_) => std::borrow::Cow::Borrowed("vi>>"),
            reedline::PromptEditMode::Custom(_) => std::borrow::Cow::Borrowed("custom>>"),
        }
    }

    fn render_prompt_multiline_indicator(&self) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed("><")
    }
}

fn evaluate_command(input: &str) {
    if input.trim().is_empty() {
        return;
    }

    let pattern = Regex::new(r"[;|\n]").unwrap();
    let commands: Vec<&str> = pattern.split(input).collect();

    for command in commands {
        let command = command.trim();
        if command.is_empty() {
            println!("Empty command, skipping.");
            continue;
        }

        let tokens: Vec<&str> = command.split_whitespace().collect();
        if tokens.is_empty() {
            return;
        }

        let cmd_name = tokens[0];
        let args: Vec<String> = tokens[1..].iter().map(|&s| s.to_string()).collect();

        COMMAND_LIST.execute_command(
            cmd_name.to_string(),
            if args.is_empty() { None } else { Some(args) },
        );
    }
}

pub async fn handle_repl() {
    let mut line_editor = Reedline::create();
    register_commands();

    loop {
        let sig = line_editor.read_line(&ZPrompt {
            left_text: String::new(),
            right_text: "<<".to_string(),
        });

        match sig {
            Ok(Signal::Success(buffer)) => {
                if buffer == "exit" {
                    std::process::exit(0);
                } else {
                    evaluate_command(&buffer);
                }
            }
            Ok(Signal::CtrlC) => {
                println!("\nCONTROL+C RECEIVED, TERMINATING");
                std::process::exit(0);
            }
            err => {
                eprintln!("Error: {:?}", err);
            }
        }
    }
}
