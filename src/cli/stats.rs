use std::fmt::Write;

use crate::{cli_exit, io::{ConsoleWriter, ReadSource}};

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

#[allow(unused_must_use)] // TODO handle this
pub fn run(source: ReadSource, args: Args) {
    let data = match super::utils::read_file(&source) {
        Ok(s) => s,
        Err(e) => cli_exit!("{}", e),
    };
    
    let mut writer = ConsoleWriter;
    if args.json {writer.write_str("[\n");}
    for (i, uhm) in data.iter().enumerate() {
        if args.json {writer.write_char('\t');}
        super::utils::print_stats(uhm, args.json, &mut writer);
        if i < data.len() && args.json {
            writer.write_char(',');
        }
        writer.write_char('\n');
    }
    if args.json {
        writer.write_char(']');
    }
    writer.write_char('\n');
}
