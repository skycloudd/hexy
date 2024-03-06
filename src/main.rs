use clap::Parser;
use owo_colors::OwoColorize as _;
use std::{
    io::{Read as _, StdoutLock, Write as _},
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
        Some(filename) => std::fs::read(filename)?,
        None => {
            let mut buffer = Vec::new();

            std::io::stdin().read_to_end(&mut buffer)?;

            buffer
        }
    };

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
        match b {
            0..=31 => write!(stdout, "{:02x} ", b.blue())?,
            32..=127 => write!(stdout, "{:02x} ", b.green())?,
            128..=255 => write!(stdout, "{:02x} ", b.yellow())?,
        };
    }

    for _ in chunk.len()..width {
        write!(stdout, "   ")?;
    }

    write!(stdout, "| ")?;

    for b in chunk {
        match b {
            0..=31 => write!(stdout, "{}", ".".blue()),
            32..=127 => write!(stdout, "{}", (*b as char).green()),
            128..=255 => write!(stdout, "{}", ".".yellow()),
        }?;
    }

    writeln!(stdout)
}
