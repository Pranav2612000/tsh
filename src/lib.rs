pub enum CmdResult {
    EntryAdded(String),
    CmdExtracted,
    DisplayHelp(String),
}

pub fn get_command(args: &[String]) -> CmdResult {
    let command = &args[1];
    println!("{}", command);
    CmdResult::EntryAdded("hello world".to_string())
}

/*
pub add_entry() -> {
    // Check if the 2nd argument is add

    // Extract name to be given to entry

    // create /tsh directory if not exist

    // create /tsh/<name> folder. error if already exists

    // copy key from given path to this directory

    // add endpoint to a txt file
}
*/
