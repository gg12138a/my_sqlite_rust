//!  # Responsibility
//!
//!  1. print prompt
//!  2. receive user input
//!

use std::{io, process};

use self::{cmd::parse_cmd, prompt::print_prompt};

pub mod cmd;
pub mod prompt;

pub fn read_input() -> String {
    let mut user_input = String::default();
    match io::stdin().read_line(&mut user_input) {
        Ok(cnt) => {
            // TODO: log this
        }
        Err(err) => {
            process::exit(1);
        }
    };

    user_input.trim().into()
}

pub fn start_cli_main() {
    loop {
        print_prompt();

        let user_input = read_input();

        match parse_cmd(&user_input) {
            Some(cmd) => match cmd {
                cmd::CliCommand::Exit => {
                    process::exit(0);
                }
            },
            None => {
                println!("Unrecognized command '{}'", user_input);
            }
        }
    }
}
