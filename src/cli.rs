use clap::{Parser, Subcommand};

use crate::io::{ReadSource};

pub mod record;
pub mod stats;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'f', long = "file", default_value = "uhm.json")]
    from: String,
    #[arg(long = "stdin", action = clap::ArgAction::SetTrue)]
    stdin: bool,
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Record(record::Args),
    Stats(stats::Args),
}

pub fn run(args: Args) {
    let source = if args.stdin {
        ReadSource::Stdin
    } else {
        ReadSource::File(args.from)
    };
    match args.command {
        None => {
            Args::parse_from(["--help"]);
        },
        Some(command) => match command {
            Commands::Record(args) => record::run(source, args),
            Commands::Stats(args) => stats::run(source, args),
        },
    };
}


mod utils {
    use crate::io::{ReadSource, WriteTarget};
    use crate::Uhms;
    
    #[macro_export]
    macro_rules! cli_exit {
        () => {
            ::std::process::exit(1)
        };
        ($($arg:tt)*) => {{
            println!($($arg)*);
            std::process::exit(1)
        }};
    }

    pub fn read_file(source: &ReadSource) -> Result<Vec<Uhms>, String> {
        let content = match source.read() {
            Ok(content) => content,
            Err(e) => cli_exit!("Cannot read from {}: {}", source.map("stdin", |f| f), e),
        };

        match serde_json::from_str(&content) {
            Ok(content) => Ok(content),
            Err(e) => cli_exit!("Cannot parse input from {}: {}", source.map("stdin", |f| f), e),
        }
    }

    pub fn write_file(target: WriteTarget, items: &Vec<Uhms>) {
        let formatted = match serde_json::to_string_pretty(items) {
            Ok(formatted) => formatted,
            Err(e) => cli_exit!("Cannot format items: {}", e),
        };

        match target.write(&formatted) {
            Ok(_) => {},
            Err(e) => cli_exit!("Cannot write to {}: {}", target.map("stdout", |f| f), e),
        }
    }

    pub fn print_stats<W: std::io::Write>(uhm: &Uhms, json: bool, writer: &mut W) -> Result<(), std::io::Error> {
        let stats = uhm.stats();
        if json {
            match serde_json::to_writer(std::io::stdout(), &stats) {
                Ok(()) => Ok(()),
                Err(e) => crate::cli_exit!("Cannot format json: {}", e),
            }
        } else {
            if let Some(name) = &uhm.name {
                writeln!(writer, "For {}", name)?;
            } else {
                writeln!(writer, "{}", uhm.start.format("%Y-%m-%d at %H:%M:%S"))?;
            }

            writeln!(writer, " > Count     {} uhm", stats.count)?;
            writeln!(writer, " > Duration  {}:{:02} min", stats.min_sec.0, stats.min_sec.1)?;
            writeln!(writer, " > Mean      {:.2} s", stats.delay_mean / 1000.)?;
            writeln!(writer, " > Deviation {:.2} s", stats.delay_std / 1000.)?;
            writeln!(writer, " > Score     {} uhm/min", stats.per_minute)?;

            Ok(())
        }
    }
}
