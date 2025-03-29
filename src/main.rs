use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "shuttle")]
    name: String
}


fn main() {
    let args = Args::parse();
  
    println!("Hello, {}!", args.name);
}
