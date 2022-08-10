use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "break-line")]
struct CliArgs {
    #[clap(name = "text")]
    pub text: String,
    #[clap(name = "delimiter", default_value = ":")]
    pub delimiter: String,
}

fn main() {
    let CliArgs { text, delimiter } = CliArgs::parse();
    for component in text.split(&delimiter) {
        println!("{}", component);
    }
}
