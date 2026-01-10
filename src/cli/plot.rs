use crate::{
    io::{ReadSource, WriteTarget},
    plot::Artist,
};

#[derive(clap::Args, Debug, Clone)]
pub struct Args {
    #[arg()]
    name: Option<String>,
    #[arg(short = 'o')]
    outfile: Option<String>,
    #[arg(name = "FORMAT", long = "format", default_value = "tikz")]
    artist: ArtistChoice,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ArtistChoice {
    #[value(name = "tikz")]
    TikZ,
    #[value(name = "cetz")]
    CeTZ,
    #[value(name = "tikz-small")]
    TikZSmall,
    #[value(name = "cetz-small")]
    CeTZSmall,
}

pub fn run(source: ReadSource, args: Args) {
    let data = match super::utils::read_file(&source) {
        Ok(uhms) => uhms,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let target = match args.outfile.as_deref() {
        Some("-") | None => WriteTarget::Stdout,
        Some(fname) => WriteTarget::File(fname.to_string()),
    };

    let filtered = if let Some(name) = args.name {
        let mut result = Vec::with_capacity(data.len() / 2);

        for uhm in data {
            if let Some(n) = &uhm.name {
                if n == &name {
                    result.push(uhm);
                }
            }
        }

        result
    } else {
        data
    };

    if filtered.len() == 0 {
        println!("No entries to plot");
        return;
    }

    let mut plot = crate::plot::Canvas::new();
    let mut options = crate::plot::PlotOptions::default();

    for uhm in filtered {
        crate::plot_uhm(&uhm, &mut plot, &options);
        options.y += 1.;
    }

    let rendered = match args.artist {
        ArtistChoice::TikZ => crate::plot::TikZ::render_doc(plot).unwrap(),
        ArtistChoice::CeTZ => crate::plot::CeTZ::render_doc(plot).unwrap(),
        ArtistChoice::TikZSmall => crate::plot::TikZ::render(plot).unwrap(),
        ArtistChoice::CeTZSmall => crate::plot::CeTZ::render(plot).unwrap(),
    };

    let _ = target.write(&rendered);
}
