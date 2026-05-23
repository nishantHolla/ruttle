use super::error::ArgsError;
use crate::config;
use crate::util;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "Terus")]
#[command(version = "0.2.0")]
#[command(about = "Templating engine for HTML written in rust")]
#[command(long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub output: PathBuf,

    #[arg(required = true)]
    pub inputs: Vec<PathBuf>,
}

impl Args {
    pub fn validate_and_transform(args: Args) -> Result<Args, ArgsError> {
        // Transform output argument to canonical form
        let output = args.output.canonicalize().map_err(|e| {
            let s = format!(
                "Failed to transform output argument {}\n{}",
                args.output.display(),
                e.to_string()
            );
            ArgsError::InvalidArgument(s)
        })?;

        // Check if output argument is a directory
        if !output.is_dir() {
            let s = format!("Output argument {} is not a directory", output.display());
            return Err(ArgsError::InvalidArgument(s));
        }

        // Transform all input arguments to canonical form
        let mut inputs: Vec<PathBuf> = vec![];

        for input in &args.inputs {
            let r = input.canonicalize().map_err(|e| {
                let s = format!(
                    "Failed to transform input argument {}\n{}",
                    input.display(),
                    e.to_string()
                );
                ArgsError::InvalidArgument(s)
            })?;

            // Check if input argument is a file
            if !r.is_file() {
                let s = format!("Argument input {} is not a file", r.display());
                return Err(ArgsError::InvalidArgument(s));
            }

            // Check if input argument has the required extension
            if !util::file::has_extension(&r, &config::PART_EXTENSION) {
                let s = format!(
                    "Input file {} does not end with extension {}",
                    r.display(),
                    config::PART_EXTENSION
                );
                return Err(ArgsError::InvalidArgument(s));
            }

            inputs.push(r);
        }

        Ok(Args { inputs, output })
    }
}
