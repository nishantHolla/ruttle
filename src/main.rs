use clap::Parser;
use terus::{Args, TerusError};

fn main() {
    if let Err(e) = run() {
        e.print();
        std::process::exit(1);
    }
}

fn run() -> Result<(), TerusError> {
    let args = Args::parse();
    let args = Args::validate_and_transform(args)?;

    println!("Inputs: {:?}", args.inputs);
    println!("Output: {:?}", args.output);

    Ok(())
}
