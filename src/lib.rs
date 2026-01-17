pub mod cli;

pub mod stats;

mod data;
pub use data::Uhms;

pub mod io;

pub mod plot;

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

                values.push((next - prev).num_milliseconds());
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

pub fn plot_uhm(uhm: &crate::Uhms, c: &mut plot::Canvas, options: &plot::PlotOptions) {
    let milliseconds = uhm.duration().num_milliseconds();
    let millisecond_width = options.second_width / 1000.;

    let name = match &uhm.name {
        Some(name) => name.clone(),
        None => format!("{}", uhm.start.format("%Y-%m-%d %H:%M:%S")),
    };

    let y = options.y;
    let mut x = 0.;

    c.draw(
        plot::Text::default()
            .content(format!("{} ({:.2} uhm/min)", name, crate::stats::per_minute(uhm.data.len(), &uhm.duration())))
            .anchor(plot::Anchor::East)
            .at((x - 0.5, y))
            .stroke(plot::Stroke::default().color(plot::Color::none()))
    );

    c.draw(
        plot::Line::default()
            .start((x, y))
            .end((x + ((milliseconds as f64) * millisecond_width), y)),
    );

    for offset in &uhm.data {
        x += (*offset as f64 / 1000.) * millisecond_width;
        c.draw(plot::Circle::default().at((x, y)).radius(0.05));
    }
}
