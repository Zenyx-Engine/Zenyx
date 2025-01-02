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
            if list1 == 0 || list2 == 0 {
                return Err(mlua::Error::RuntimeError("Zero values not allowed".to_string()));
            }
            Ok(list1 == list2)
        })?;
        globals.set("isEqual", is_equal)?;

        let log = lua.create_function(|_, (msg,): (String,)| {
        println!("{}", msg);
        Ok(())
        })?;
        globals.set("log", log)?;

        let fail_safe = lua.create_function(|_,()|{
            println!("Failed");
            Ok(())
        })?;
        globals.set("failSafe",fail_safe)?;
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
                        for value in &values {
                            match value {
                                mlua::Value::Nil => println!("Got nil value"),
                                mlua::Value::Number(n) => println!("Got number: {}", n),
                                mlua::Value::String(s) => println!("Got string: {}", s.to_str()?),
                                mlua::Value::Boolean(b) => println!("Got boolean: {}", b),
                                _ => eprintln!("Got unexpected type: {:#?}", value)
                            }
                        }
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
                        eprintln!("Error: {} at line {}", e, line.lines().count());
                        eprintln!("Input that caused error: {}", line);
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
