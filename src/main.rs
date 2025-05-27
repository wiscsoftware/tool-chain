use crate::Tools::{Elf, Hexdump, Ldd, Macho, Str, Strip, Unknown};
use crate::hexdump::Hexer;
use crate::macho::MachFile;
use clap::Parser;
use std::fs::File;
use std::io::Read;

mod elf;
mod experiments;
mod hexdump;
mod macho;

#[derive(Parser, Debug)]
#[command(about, long_about=None)]
struct Args {
    #[arg(short, default_value = "hexdump")]
    tool: String,

    #[arg(short)]
    file: String,
}

#[derive(PartialEq, Eq)]
enum Tools {
    Hexdump,
    Elf,
    Macho,
    Ldd,
    Strip,
    Str,
    Unknown,
}

impl Tools {
    fn from(op: &str) -> Tools {
        if op == "hexdump" {
            Hexdump
        } else if op == "elf" {
            Elf
        } else if op == "macho" {
            Macho
        } else if op == "ldd" {
            Ldd
        } else if op == "strip" {
            Strip
        } else if op == "string" {
            Str
        } else {
            Unknown
        }
    }

    fn str(op: Tools) -> &'static str {
        if op == Hexdump {
            "hexdump"
        } else if op == Ldd {
            "ldd"
        } else if op == Strip {
            "strip"
        } else if op == Str {
            "str"
        } else if op == Elf {
            "elf"
        } else if op == Macho {
            "macho"
        } else {
            "unknown"
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut buf = Vec::new();
    match File::open(&args.file) {
        Ok(mut file) => match file.read_to_end(&mut buf) {
            Ok(_) => {
                buf.reverse();
            }
            Err(e2) => {
                eprintln!("error reading file content: {}", e2);
                std::process::exit(1)
            }
        },
        Err(e) => {
            eprintln!("error opening file: {}", e);
            std::process::exit(1)
        }
    }

    let tool = Tools::from(args.tool.trim());

    let width = 8;
    if tool == Hexdump {
        let hex = Hexer::new(buf);
        let s = hex.hex(width);
        println!("{}", s)
    } else if tool == Macho {
        buf.reverse();
        match MachFile::parse(buf.as_slice()) {
            Ok(r) => {
                println!("{}", r.str(args.file.as_str()))
            }
            Err(e) => {
                eprintln!("macho tool error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        println!("not a supported tool: {}", args.tool.trim());
        return;
    }
}
