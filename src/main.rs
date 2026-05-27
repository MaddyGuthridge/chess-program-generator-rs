mod display;
mod python;
mod rust;

use std::io::stdout;
use std::time::Instant;

use clap::Parser;
use humantime::format_duration;
use strum::EnumString;

use crate::display::show_board;
use crate::python::write_python_program;
use crate::rust::write_rust_program;

const TAB: &str = "    ";

#[derive(Debug, Clone, EnumString)]
enum OutputLanguage {
    Rust,
    Python,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long)]
    depth: usize,
    #[arg(short, long)]
    language: OutputLanguage,
}

fn main() {
    let args = Args::parse();
    let mut out = stdout().lock();

    let t_start = Instant::now();

    eprintln!("Generating to a depth of {}", args.depth);

    match args.language {
        OutputLanguage::Python => write_python_program(args.depth, &mut out),
        OutputLanguage::Rust => write_rust_program(args.depth, &mut out),
    }

    let t_end = Instant::now();

    eprintln!("\nDone in {}", format_duration(t_end - t_start));
}
