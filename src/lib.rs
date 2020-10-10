pub enum CmdResult {
    EntryAdded(String),
    CmdExtracted,
}

pub fn get_command(args: &[String]) -> CmdResult {
    //"hello world".to_string()
    CmdResult::EntryAdded("hello world".to_string())
}
