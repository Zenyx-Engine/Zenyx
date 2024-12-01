
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
pub mod engine;
pub mod editor;

#[derive(Parser)]
#[command(version, about, long_about = None,disable_version_flag = true,disable_help_flag = true)]
struct Cli {
    #[arg(short,long)]
    release: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg()]
        task: Task
    },
    Config,

}
#[derive(Clone,ValueEnum)]
enum Task {
    Engine, // Builds both editor and core
    Editor, // Builds editor only
    Core, // Builds engine core only
    Help, 
}



fn main() {
    let cli = Cli::parse();

    


    if cli.release {
        println!("Running in release mode")
    }

    match &cli.command {

        None => {
            Cli::command().print_help().map_err(|e| {
                println!("Could not run Xtask: {e}");
                
            }).unwrap();
        }
        Some(Commands::Run { task }) => {
            match task {
                Task::Engine => engine::build_engine(),
                Task::Editor => todo!("Editor is not being actively worked on"),
                Task::Core => {
                    engine::build_core();
                },
                Task::Help => {
                    println!("The following options are avalible to run");
                    todo!()
                },
            }
        }
        Some(Commands::Config) => {
            todo!()
        }
    }

}