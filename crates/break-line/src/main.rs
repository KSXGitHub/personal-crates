use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "break-line")]
struct CliArgs {
    /// String to be broken.
    pub text: String,
    /// Separator to be used.
    #[clap(default_value = ":")]
    pub delimiter: String,
}

fn main() {
    let CliArgs { text, delimiter } = CliArgs::parse();
    for component in text.split(&delimiter) {
        println!("{}", component);
    }
}
