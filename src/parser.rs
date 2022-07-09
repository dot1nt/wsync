use crate::*;
use std::error;

pub enum Action {
    Sync(Sync),
    Command(Cmd),
    Import(Import),
    Nothing
}

pub fn parse_args() -> Args  {
    let args = args();

    let mut cmd_args = Args {
                        quiet: false,
                        help: false,
                        files: Vec::new()};

    for arg in args.skip(1) {
        match arg.as_str() {
            "-q" | "--quiet"  =>  cmd_args.quiet = true,
            "-h" | "--help"   =>  cmd_args.help = true,
            _                 =>  cmd_args.files.push(arg)
        }
    }

    cmd_args
}

pub fn parse_line(line: &str) -> Result<Action, Box<dyn error::Error>> {
    if line.starts_with("//") { // // comment
        Ok(Action::Nothing)
    } else if line.starts_with("<-- ") && line.ends_with(" -->") { // <-- Message -->
        let msg = &line[4..line.len()-4].trim();

        log(msg, "");
        Ok(Action::Nothing)
    } else if line.starts_with("$(") && line.ends_with(")") { // $(command)
        let cmd = &line[2..line.len()-1].trim();

        Ok(Action::Command(Cmd {cmd: cmd.to_string()}))
    } else if line.starts_with("-import") { // -import file.sync
        let file = line[7..].trim();

        if file.is_empty() {
            return Err("No import file specified".into());
        }

        Ok(Action::Import(Import {file: file.to_string()}))
    } else if line.contains(" -> ") { // /home/user -> /home/user/Backup
        let paths: Vec<String> = line.split(" -> ").map(|i| i.trim()).map(str::to_string).collect();
    
        if paths.len() != 2 {
            return Err(format!("Line formatted wrong: {}", line).into());
        }

        Ok(Action::Sync(Sync {src: paths[0].to_string(), dest: paths[1].to_string()}))
    } else if line.is_empty() {
        Ok(Action::Nothing)
    } else {
        Err(format!("Line formatted wrong: {}", line).into())
    }
}