use clap::Parser;
use owo_colors::{AnsiColors, OwoColorize as _, Stream::Stdout};
use std::{
    io::{BufReader, Read as _, StdoutLock, Write as _},
    path::PathBuf,
};

#[derive(Parser)]
struct Args {
    filename: Option<PathBuf>,

    #[arg(short, long, default_value = "16")]
    width: usize,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let contents = match args.filename {
        Some(filename) => BufReader::new(std::fs::File::open(filename)?)
            .bytes()
            .collect::<Result<Vec<_>, _>>(),
        None => BufReader::new(std::io::stdin()).bytes().collect(),
    }?;

    let chunks = contents.chunks(args.width);

    let mut stdout = std::io::stdout().lock();

    for (offset, chunk) in chunks.enumerate() {
        write_chunk(&mut stdout, args.width, offset, chunk)?;
    }

    Ok(())
}

fn write_chunk(
    stdout: &mut StdoutLock,
    width: usize,
    offset: usize,
    chunk: &[u8],
) -> std::io::Result<()> {
    write!(stdout, "{:08x} | ", offset * width)?;

    for b in chunk {
        write!(
            stdout,
            "{:02x} ",
            b.if_supports_color(Stdout, |b| b.color(match b {
                0..=31 => AnsiColors::Blue,
                32..=127 => AnsiColors::Green,
                128..=255 => AnsiColors::Yellow,
            }))
        )?;
    }

    for _ in chunk.len()..width {
        write!(stdout, "   ")?;
    }

    write!(stdout, "| ")?;

    for b in chunk {
        let (c, color) = match *b {
            0..=31 => ('.', AnsiColors::Blue),
            32..=127 => (*b as char, AnsiColors::Green),
            128..=255 => ('.', AnsiColors::Yellow),
        };

        write!(
            stdout,
            "{}",
            c.if_supports_color(Stdout, |c| c.color(color))
        )?;
    }

    writeln!(stdout)
}
