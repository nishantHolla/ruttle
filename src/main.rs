use clap::Parser;
use terus::Args;

fn main() {
    let args = Args::parse();

    println!("Inputs: {:?}", args.inputs);
    println!("Output: {:?}", args.output);
}
