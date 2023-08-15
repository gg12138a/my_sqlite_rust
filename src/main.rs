//! # Reference
//!
//! - [Let's Build a Simple Database(Let's Build a Simple Database)] https://cstack.github.io/db_tutorial/
//!
//! # Log Level
//!
//! Run the program with the environment variable RUST_LOG=trace. for example:
//! ```shell
//! RUST_LOG=trace cargo run
//! ```

pub mod cli;

fn main() {
    pretty_env_logger::init();
    cli::start_cli_main();
}
