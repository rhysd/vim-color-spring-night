mod airline;
mod alacritty;
mod colorscheme;
mod palette;

use airline::AirlineTheme;
use alacritty::AlacrittyTheme;
use colorscheme::Colorscheme;
use palette::Palette;

use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

fn write_to_files(dir: &str) -> Result<()> {
    let palette = Palette::default();

    fn join(entries: &[&str]) -> PathBuf {
        let mut entries = entries.iter();
        let mut path = PathBuf::from(entries.next().unwrap());
        for entry in entries {
            path.push(entry);
        }
        path
    }

    let path = join(&[dir, "colors", "spring-night.vim"]);
    let file = File::create(&path)
        .with_context(|| format!("Could not create colorscheme file: {:?}", &path))?;
    Colorscheme::new(&palette)
        .write_to(&mut BufWriter::new(file))
        .with_context(|| format!("Could not write to colorscheme file {:?}", &path))?;

    let path = join(&[dir, "autoload", "airline", "themes", "spring_night.vim"]);
    let file = File::create(&path)
        .with_context(|| format!("Could not create airline theme file {:?}", &path))?;
    AirlineTheme::new(&palette)
        .write_to(&mut BufWriter::new(file))
        .with_context(|| format!("Could not write to airline theme file {:?}", &path))?;

    let path = join(&[dir, "alacritty", "spring_night.toml"]);
    let file = File::create(&path)
        .with_context(|| format!("Could not create alacritty theme file {:?}", &path))?;
    AlacrittyTheme::new(&palette)
        .write_to(&mut BufWriter::new(file))
        .with_context(|| format!("Could not write to alacritty theme file {:?}", &path))
}

fn write_to(w: &mut impl Write) -> Result<()> {
    let palette = Palette::default();
    Colorscheme::new(&palette).write_to(w)?;
    writeln!(w)?;
    AirlineTheme::new(&palette).write_to(w)?;
    writeln!(w)?;
    AlacrittyTheme::new(&palette).write_to(w)?;
    Ok(())
}

fn main() -> Result<()> {
    let (program, args) = {
        let mut argv = env::args();
        (argv.next().unwrap(), argv)
    };

    let mut opts = getopts::Options::new();
    opts.optopt("d", "dir", "repository root directory", "PATH");
    opts.optflag("h", "help", "print this help");
    let opts = opts;

    let matches = opts
        .parse(args)
        .context("Please try --help for more detail")?;

    if matches.opt_present("h") {
        let brief = &format!("Usage: {} [options]", program);
        eprintln!("{}", opts.usage(brief));
        return Ok(());
    }

    if let Some(dir) = matches.opt_str("d") {
        write_to_files(&dir)
    } else {
        write_to(&mut io::stdout().lock()).context("Could not write to stdout")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_to_stdout_successfully() {
        let mut stdout = vec![];
        write_to(&mut stdout).unwrap();
        assert!(!stdout.is_empty());
    }
}
