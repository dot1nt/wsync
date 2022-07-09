use crate::{log, parser, Args, Cmd, Command, Sync};
use std::error;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

impl Sync {
    pub fn run(self) -> Result<bool, Box<dyn error::Error>> {
        
        let cmd = format!("/bin/rsync -av {} {} --delete", self.src, self.dest);

        let out = Command::new("sh")
                        .arg("-c")
                        .arg(cmd)
                        .output()?;

        if !out.status.success() {
            let stderr = &String::from_utf8(out.stderr)?;
            return Err(stderr.to_string().into())
        } else {
            let stdout = &String::from_utf8(out.stdout)?;

            if stdout.lines().count() > 4 {
                return Ok(true)
            }
        }

        Ok(false)
    }
}

impl Cmd {
    pub fn run(self) -> Result<(), Box<dyn error::Error>> {
        let out = Command::new("sh")
                            .arg("-c")
                            .arg(self.cmd)
                            .output()?;

        if !out.status.success() {
            let stderr = &String::from_utf8(out.stderr)?;
            return Err(format!("Command failed: {}", stderr).into())
        }

        Ok(())
    }
}


pub fn sync(file: &str, args: &Args) -> Result<(), Box<dyn error::Error>> { 
    let home_path = env::var("HOME")?.to_string();
    let file_path = &file.replace("~", &home_path);

    if !Path::new(file_path).exists() {
        return Err(format!("No such file or dictionary: {}", file).into())
    }

    let f = File::open(file_path)?;
    let r = BufReader::new(f);

    for line in r.lines() {

        match parser::parse_line(&line?) {
            Ok(parser::Action::Command(cmd)) => {
                log(&format!("Running command: {}", cmd.cmd), "log");
                cmd.run()?;
            }

            Ok(parser::Action::Import(file)) => { 
                log(&format!("Importing file: {}", file.file), "log");
                sync(&file.file, &args)?;
            }

            Ok(parser::Action::Sync(sync)) => {
                let src = sync.src.clone();
                let dest = sync.dest.clone();

                let res = sync.run()?;

                if res {
                    log(&format!("Syncing: {} -> {}", src, dest), "ch");
                } else {
                    log(&format!("Syncing: {} -> {}", src, dest), "nc");
                }
            }

            Ok(parser::Action::Nothing) => {}
        
            Err(e) => { return Err(e) }
        }
    }

    Ok(())
}