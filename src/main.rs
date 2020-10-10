use std::env;

// tsh add <myname> endpoint <key-location>
// tsh myname
fn main() {
    let args: Vec<String> = env::args().collect();
    let command_to_execute = tsh::get_command(&args);
    //println!("{}", command_to_execute);
    match command_to_execute {
        tsh::CmdResult::EntryAdded(message) => {
            println!("{}", message);
        }
        tsh::CmdResult::CmdExtracted => {
            println!("ssh command rcvd");
        }
        tsh::CmdResult::DisplayHelp(message) => {
            println!("{}", message);
            println!("Display help output here");
        }
    }
}
