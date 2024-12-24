use mlua::{Function, Lua, LuaOptions, MultiValue, Number, Value::Nil};
use rustyline::{error::ReadlineError, DefaultEditor};
use crate::core::repl::handler::Command;

#[derive(Default)]
pub struct ZLua;

impl Command for ZLua {
    fn execute(&self, _args: Option<Vec<String>>) -> Result<(), anyhow::Error> {
        let time = chrono::Local::now().format("%H:%M:%S.%3f").to_string();
        let prompt = format!("[{}/{}] {}", time, "ZLUA", ">>\t");
        let lua = Lua::new_with(
            mlua::StdLib::ALL_SAFE, 
            LuaOptions::default()
        )?;
        let globals = lua.globals();
        //This just adds 2 numbers together
        let add = lua.create_function(|_, (number1,number2):(i32,i32)|{
            let result = number1 + number2;
            println!("{result}");
            Ok(())
        })?;
        globals.set("add", add)?;

        let is_equal = lua.create_function(|_, (list1, list2): (i32, i32)| {
            // This function just checks whether two string lists are equal, and in an inefficient way.
            // Lua callbacks return `mlua::Result`, an Ok value is a normal return, and an Err return
            // turns into a Lua 'error'. Again, any type that is convertible to Lua may be returned.
            Ok(list1 == list2)
        })?;
        globals.set("is_equal", is_equal)?;

        let log = lua.create_function(|_, (msg,): (String,)| {
        println!("{}", msg);
        Ok(())
        })?;
        globals.set("log", log)?;

        
        let mut editor = DefaultEditor::new().expect("Failed to create editor");

        loop {
            let mut prompt = &prompt;
            let mut line = String::new();

            loop {
                match editor.readline(prompt) {
                    Ok(input) => line.push_str(&input),
                    Err(ReadlineError::Interrupted) => {
                        println!("Exiting ZLUA shell...");
                        return Ok(());
                    }
                    Err(_) => {}
                }

                match lua.load(&line).eval::<MultiValue>() {
                    Ok(values) => {
                        editor.add_history_entry(line).unwrap();
                        println!(
                            "{}",
                            values
                                .iter()
                                .map(|value| format!("{:#?}", value))
                                .collect::<Vec<_>>()
                                .join("\t")
                        );
                        break;
                    }
                    Err(mlua::Error::SyntaxError {
                        incomplete_input: true,
                        ..
                    }) => {
                        // continue reading input and append it to `line`
                        line.push_str("\n"); // separate input lines
                        prompt = prompt;
                    }

                    Err(e) => {
                        eprintln!("error: {}", e);
                        break;
                    }
                }
            }
        }
    }

    fn undo(&self) {}

    fn redo(&self) {}

    fn get_description(&self) -> String {
        String::from("Runs the ZLua interpreter")
    }

    fn get_name(&self) -> String {
        String::from("zlua")
    }

    fn get_help(&self) -> String {
        String::from("zlua")
    }

    fn get_params(&self) -> String {
        String::from("No parameters required.")
    }
}
