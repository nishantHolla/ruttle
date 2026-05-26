use clap::Parser;
use ruttle::{AppError, Args, Context};

fn main() {
    if let Err((e, context)) = run() {
        e.print(context);
        std::process::exit(1);
    }
}

fn run() -> Result<(), (AppError, Option<Context>)> {
    // Parse arguments
    let args = Args::parse();

    // Validate and transform arguments
    let args = match Args::validate_and_transform(args) {
        Ok(args) => Ok(args),
        Err(e) => Err((AppError::Args(e), None)),
    }?;

    // Initialize the context
    let mut context = match Context::new(&args) {
        Ok(ctx) => Ok(ctx),
        Err(e) => Err((AppError::Context(e), None)),
    }?;

    // Complete the context
    let result = context.complete();
    if let Err(e) = result {
        return Err((AppError::Context(e), Some(context)));
    }

    // Save output of the context
    let result = context.finalize();
    if let Err(e) = result {
        return Err((AppError::Context(e), Some(context)));
    }

    // Done
    if args.debug {
        context.debug_with_ast();
    }
    Ok(())
}
