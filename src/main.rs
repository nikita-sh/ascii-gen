use clap::Parser;
use ascii_gen::generator::{self, generate_ascii}; 

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    // input image to convert
    #[arg(long, short)]
    image: String,

    #[arg(long, short)]
    // output text file
    outfile: Option<String>
}

fn main() {
    let args = Args::parse();

    generate_ascii(&args.image[..]); 
    println!("{}", args.image);
}
