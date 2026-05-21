use clap::Parser;
use terus::{Args, Context, TerusError};

fn main() {
    if let Err((e, context)) = run() {
        e.print(context);
        std::process::exit(1);
    }
}

fn run() -> Result<(), (TerusError, Option<Context>)> {
    // Parse arguments
    let args = Args::parse();

    // Validate and transform arguments
    let args = match Args::validate_and_transform(args) {
        Ok(args) => Ok(args),
        Err(e) => Err((TerusError::Args(e), None)),
    }?;

    // Initialize the context
    let mut context = match Context::new(&args) {
        Ok(ctx) => Ok(ctx),
        Err(e) => Err((TerusError::Context(e), None)),
    }?;

    // Complete the context
    let result = context.complete();
    if let Err(e) = result {
        return Err((TerusError::Context(e), Some(context)));
    }

    // Save output of the context
    let result = context.finalize();
    if let Err(e) = result {
        return Err((TerusError::Context(e), Some(context)));
    }

    // Done
    context.debug_with_ast();
    Ok(())
}
