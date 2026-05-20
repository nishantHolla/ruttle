use clap::Parser;
use terus::{Args, Context, TerusError};

fn main() {
    if let Err(e) = run() {
        e.print();
        std::process::exit(1);
    }
}

fn run() -> Result<(), TerusError> {
    let args = Args::parse();
    let mut context = Context::new(&args)?;

    Ok(())
}
