use std::io::Write;

pub fn print_prompt() {
    print!("db > ");
    std::io::stdout().flush().unwrap();
}
