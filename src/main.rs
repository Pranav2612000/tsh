use std::env;
use std::process::Command;

// tsh add <myname> endpoint <key-location>
// tsh myname
fn main() {
    let args: Vec<String> = env::args().collect();
    let command_to_execute = tsh::get_command(&args);
    match command_to_execute {
        tsh::CmdResult::EntryAdded(message) => {
            println!("{}", message);
        }
        tsh::CmdResult::CmdExtracted(cmd) => {
            let mut split = cmd.split(" ");
            if let Some(program) = split.next() {
                let args = split.collect::<Vec<&str>>(); 
                Command::new(program).args(&args).status()
                    .expect("command not found");
            }
        }
        tsh::CmdResult::DisplayHelp(message) => {
            println!("{}", message);
            println!("Syntax:");
            println!("To add keys:");
            println!("tsh add <nickname> <endpoint> <key-location>");
            println!("To ssh:");
            println!("tsh <nickname>");
        }
    }
}
