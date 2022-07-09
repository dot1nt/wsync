pub mod parser;
pub mod sync;

use std::path::Path;
use std::env::args;
use std::process::Command;

#[derive(Clone)]
pub struct Args {
    quiet: bool,
    help: bool,
    files: Vec<String>
}

#[derive(Clone)]
pub struct Sync {
    pub src: String,
    pub dest: String,
}

pub struct Cmd {
    pub cmd: String,
}

pub struct Import {
    pub file: String
}


pub fn log(message: &str, status: &str) {
    let args = parser::parse_args();
    if args.quiet { return }

    let red = "\x1b[31m";
    let yellow = "\x1b[33m";
    let green = "\x1b[32m";
    let esc = "\x1b[0m";
    
    match status {
        "log"   => println!("{}[LOG]: {}{}", green, message, esc),
        "nc"    => println!("{}[NC]:  {}{}", green, message, esc), // not changed
        "ch"    => println!("{}[CH]:  {}{}", yellow, message, esc), // changed
        "error" => println!("{}[ERROR]: {}{}", red, message, esc),
        _=>        println!("[LOG]: {}", message)
    }
}

fn main() {
    if !Path::new("/bin/rsync").exists() {
        log("Rsync not found", "error");
        std::process::exit(1)
    }

    let args = parser::parse_args();

    if args.help {
        println!("");
        println!("Usage: wsync -q/--quiet | -h/--help | file");
        println!("");
        return
    }

    if args.files.len() == 0 {
        log("No file specified", "error");
        return
    }

    let files = args.files.clone();

    for file in files {
        let res = sync::sync(&file, &args);

        match res {
            Ok(_) => {},
            Err(e) => log(&e.to_string(), "error")
        }
    }
}
