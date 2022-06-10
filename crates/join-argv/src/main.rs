use std::{env::args, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<_> = args().skip(1).collect();
    if args.is_empty() {
        eprintln!("error: Not enough arguments");
        return ExitCode::FAILURE;
    }
    let separator = &args[0];
    let arguments = &args[1..];
    println!("{}", arguments.join(separator));
    ExitCode::SUCCESS
}
