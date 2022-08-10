use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "break-line")]
struct CliArgs {
    /// String to be broken.
    pub text: String,
    /// Separator to be used.
    #[clap(default_value = ":")]
    pub separator: String,
}

fn main() {
    let CliArgs { text, separator } = CliArgs::parse();
    for component in text.split(&separator) {
        println!("{component}");
    }
}
