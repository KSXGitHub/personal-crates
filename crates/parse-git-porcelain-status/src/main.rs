mod cli_args;
mod parse;
mod sub;

fn main() {
    use clap::Parser;
    use cli_args::CliArgs;
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    match CliArgs::parse() {
        CliArgs::Select { column } => sub::select(handle, column),
        CliArgs::All => sub::all(handle),
    }
}
