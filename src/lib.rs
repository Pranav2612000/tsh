pub enum CmdResult {
    EntryAdded(String),
    CmdExtracted,
    DisplayHelp(String),
}

#[derive(Debug)]
pub struct AddEntryConfig {
    pub entry_name: String,
    pub endpoint: String,
    pub key_location: String,
}

impl AddEntryConfig {
    fn new(args: &[String]) -> Result<AddEntryConfig, &'static str> {
        // Check if number of arguments match
        if args.len() < 5 {
            return Err("not enough arguments to add");
        }
        let entry_name = args[2].clone();
        let endpoint = args[3].clone();
        let key_location = args[4].clone();
        Ok(AddEntryConfig { entry_name, endpoint, key_location })
    }
}

pub fn get_command(args: &[String]) -> CmdResult {
    if args.len() < 2 {
        return CmdResult::DisplayHelp("error".to_string());
    }
    if args[1] == "add".to_string() {
        let entry_config = AddEntryConfig::new(&args);
        match entry_config {
            Ok(entry_config) => {
                println!("{:?}", entry_config);
                add_entry(entry_config);
            }
            Err(err) => {
                return CmdResult::DisplayHelp(err.to_string());
            }
        }
    }
    let command = &args[1];
    println!("{}", command);
    CmdResult::EntryAdded("entry added".to_string())
}

pub fn add_entry(params: AddEntryConfig) -> Result<String, &'static str>{
    println!("{}", params.entry_name);
    Ok("OK".to_string())
    // Check if the 2nd argument is add

    // Extract name to be given to entry

    // create /tsh directory if not exist

    // create /tsh/<name> folder. error if already exists

    // copy key from given path to this directory

    // add endpoint to a txt file
}
