use std::io::Write;

use crate::{cli_exit, io::{ReadSource, WriteTarget}};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
    json: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self { json: false }
    }
}

pub fn run(source: ReadSource, args: Args) {
    let data = match super::utils::read_file(&source) {
        Ok(s) => s,
        Err(e) => cli_exit!("{}", e),
    };
    let target = WriteTarget::Stdout;
    
    let mut buf = Vec::new();

    if args.json {
        let _ = buf.write("[\n".as_bytes());
        for (i, uhm) in data.iter().enumerate() {
            let _ = buf.write("\t".as_bytes());
            let _ = super::utils::print_stats(uhm, true, &mut buf);
            
            if i < data.len() {
                let _ = buf.write(",".as_bytes());
            }
            let _ = buf.write("\n".as_bytes());
        }
        let _ = buf.write("]".as_bytes());
    } else {
        for uhm in &data {
            let _ = super::utils::print_stats(uhm, false, &mut buf);
            let _ = buf.write("\n".as_bytes());
        }
    }
    let output = match String::from_utf8(buf) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Cannot format: {}", e);
            return;
        }
    };

    let _ = target.write(&output);
}
