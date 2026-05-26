use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long)]
    depth: u32,
}

fn main() {
    let args = Args::parse();

    println!("Generating to a depth of {}", args.depth);
}
