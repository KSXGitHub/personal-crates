use std::{env::args, process::exit};

fn main() {
    let args: Vec<_> = args().skip(1).collect();
    if args.is_empty() {
        eprintln!("error: Not enough arguments");
        exit(1);
    }
    let separator = &args[0];
    let arguments = &args[1..];
    println!("{}", arguments.join(separator));
}
