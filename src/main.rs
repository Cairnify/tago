use chrono::Utc;
use clap::{Parser, Subcommand};
use file::{load_config, write_config};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Write};

pub mod file;
pub mod time;

#[derive(Serialize, Deserialize, Debug, Default)]
struct Config {
    entries: HashMap<String, String>,
}

#[derive(Parser)]
#[command(name = "haru")]
struct HaruCli {
    #[command(subcommand)]
    command: HaruCommand,
}

#[derive(Subcommand)]
enum HaruCommand {
    Init { arg: Option<String> },
    Days { arg: Option<String> },
    All,
    Clean,
}

fn prompt_for_confirmation(arg: &str) -> bool {
    print!(
        "The argument '{}' already exists. Are you sure you want to overwrite? (y/n): ",
        arg
    );
    io::stdout().flush().unwrap();

    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");

    response.trim().eq_ignore_ascii_case("y")
}

fn prompt_for_delete() -> bool {
    print!("This will delete all saved times. Are you sure you want to continue? (y/n): ");
    io::stdout().flush().unwrap();

    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");

    response.trim().eq_ignore_ascii_case("y")
}

fn insert_arg_into_config(arg: &str) {
    let mut config = load_config();
    if config.contains_key(arg) {
        let Some(_) = config.get(arg) else { return };

        if prompt_for_confirmation(arg) {
            config.insert(arg.to_string(), Utc::now());
            write_config(&config);
        }
    } else {
        config.insert(arg.to_string(), Utc::now());
        write_config(&config);
    }
}

fn main() {
    let cli = HaruCli::parse();

    match &cli.command {
        HaruCommand::Init { arg } => {
            if let Some(arg) = arg {
                insert_arg_into_config(arg);
            } else {
                insert_arg_into_config("default");
            }
        }
        HaruCommand::Days { arg } => {
            let arg = arg.clone().unwrap_or_else(|| "default".to_string());

            let config = load_config();
            match config.get(&arg) {
                Some(arg) => {
                    println!("{}", time::print_elapsed_time(arg, &Utc::now()));
                }
                None => println!("Could not find: {}", arg),
            }
        }
        HaruCommand::Clean => {
            if !prompt_for_delete() {
                return;
            }
            write_config(&HashMap::new());
        }
        HaruCommand::All => {
            let config = load_config();
            let now = Utc::now();
            for (key, value) in &config {
                println!("{}: {}", key, time::print_elapsed_time(value, &now));
            }
        }
    }
}