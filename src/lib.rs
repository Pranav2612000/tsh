use std::fs;
use directories::UserDirs;
use std::path::PathBuf;
use std::path::Path;
use std::io::Write;

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
                match add_entry(entry_config) {
                    Ok(_) => {}
                    Err(err) => {
                        return CmdResult::DisplayHelp(err.to_string());
                    }
                }
            }
            Err(err) => {
                return CmdResult::DisplayHelp(err.to_string());
            }
        }
    }
    let command = &args[1];
    CmdResult::EntryAdded("entry added".to_string())
}

pub fn add_entry(params: AddEntryConfig) -> Result<String, &'static str>{
    // create /tsh directory if not exist

    /* Get users home directory */
    let dir = UserDirs::new();
    let user_dirs = match dir {
        Some(x) => {
            x
        }
        None => {
            return Err("No home directory defined");
        }
    };
    let home_dir = user_dirs.home_dir();


    /* create path of the ~/tsh directory to be added */
    let mut tsh_dir: PathBuf = home_dir.to_path_buf();
    tsh_dir.push("tsh");

    /* create the tsh directory if not exists*/
    match fs::create_dir_all(&tsh_dir) {
        Ok(_) => {}
        Err(err) => {
            return Err("Error creating directory");
        }
    };

    //let tsh_dir: PathBuf = [home_dir, "tsh"].iter().collect();


    // create /tsh/<name> folder. error if already exists
    tsh_dir.push(&params.entry_name);
    let entry_exists: bool = Path::new(&tsh_dir).is_dir();
    if entry_exists == true {
        return Err("Another entry with same name exists!");
    }
    match fs::create_dir_all(&tsh_dir) {
        Ok(_) => {}
        Err(err) => {
            return Err("Error creating directory");
        }
    };

    // copy key from given path to this directory
    let orig_key_path: PathBuf = [params.key_location].iter().collect();

    let mut key_name = (&params.entry_name).clone();
    key_name = key_name + ".pem";
    tsh_dir.push(key_name);
    match fs::copy(orig_key_path, &tsh_dir) {
        Ok(_) => {}
        Err(err) => {
            //return Err(&err.to_string()[..]);
            return Err("Error copying key");
        }
    }

    // add command to a txt file
    let entry_key_location = match tsh_dir.to_str() {
        Some(x) => {x}
        None => {
            return Err("Error constructing command");
        }
    };
    let cmd = "ssh -i ".to_string() + entry_key_location + " " + &params.endpoint;

    tsh_dir.pop();
    tsh_dir.push(params.entry_name + ".txt");
    let mut file = match fs::File::create(tsh_dir) {
        Ok(x) => {x}
        Err(err) => {
            return Err("Error opening file to save command");
        }
    };
    file.write_all(cmd.as_bytes());

    Ok("OK".to_string())
}
