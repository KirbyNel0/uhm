use crate::{cli_exit, io::{ReadSource, WriteTarget}};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
    pub json: bool,
    #[arg()]
    pub name: Option<String>,
    #[arg(short = 'm', long = "message")]
    pub notes: Option<String>,
    #[clap(short = 'o', long = "output-file", default_value = "uhm.json")]
    pub to: String,
    #[clap(short = 's', long = "no-stats", action = clap::ArgAction::SetFalse)]
    pub print_stats: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self { json: false, name: None, notes: None, to: String::from("uhm.json"), print_stats: true }
    }
}

pub fn run(source: ReadSource, args: Args) {
    if let Some(name) = &args.name {
        println!("Recording for {}...", name);
    } else {
        println!("Recording...");
    };
    let new = crate::record(args.name, args.notes);
    
    // append to file
    let mut data = if let ReadSource::File(f) = &source {
        // Could not read file
        if !std::path::Path::new(f).is_file() {
            Vec::new()
        } else {
            match super::utils::read_file(&source) {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("{}", e);
                    println!("{:?}", new);
                    cli_exit!()
                }
            }
        }
    } else {
        match super::utils::read_file(&source) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("{}", e);
                println!("{:?}", new);
                cli_exit!()
            }
        }
    };
    
    println!();
    
    let target = WriteTarget::File(args.to);
    data.push(new.clone());
    super::utils::write_file(target, &data);

    if args.print_stats {
        let mut writer = std::io::stdout();
        let _ = super::utils::print_stats(&new, args.json, &mut writer);
    }
}
