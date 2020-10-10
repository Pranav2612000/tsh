use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command_to_execute = tsh::get_command(&args);
    println!("{}", command_to_execute);
}
