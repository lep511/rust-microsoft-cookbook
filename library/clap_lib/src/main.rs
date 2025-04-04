use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Subname of the person to greet
    #[arg(short, long)]
    subname: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {} {}!", args.name, args.subname);
    }
}