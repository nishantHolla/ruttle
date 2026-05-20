use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "Terus")]
#[command(version = "0.1.0")]
#[command(about = "Templating engine for HTML written in rust")]
#[command(long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub output: PathBuf,

    #[arg(required = true)]
    pub inputs: Vec<PathBuf>,
}
