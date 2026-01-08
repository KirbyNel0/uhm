pub mod cli;

pub mod stats;

mod data;
pub use data::Uhms;

pub mod io;

pub fn record(name: Option<String>, notes: Option<String>) -> Uhms {
    use chrono::Utc;
    use console::Term;
    use std::io::Write;

    let before = Utc::now();
    let mut values = Vec::new();
    let terminal = Term::stdout();

    let mut prev = before.clone();

    loop {
        let c = terminal.read_char();

        match c {
            Ok('\n') => break,
            _ => {
                let next = chrono::Utc::now();

                values.push((next - prev).num_milliseconds() as i64);
                prev = next;

                print!("\r=> {} ", values.len());
                #[allow(unused_must_use)]
                std::io::stdout().flush();
            }
        }
    }

    let after = Utc::now();
    Uhms {
        start: before,
        end: after,
        data: values,
        name: name,
        notes: notes,
    }
}
