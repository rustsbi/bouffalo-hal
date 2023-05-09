use clap::Parser;

/// Bouffalo ROM image helper
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input ROM image filename
    input: String,
    /// Write output to <filename>
    #[arg(short, long, value_name = "FILENAME")]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    println!("Input file name: {}!", args.input);
    println!("Output file name: {:?}!", args.output);
}
