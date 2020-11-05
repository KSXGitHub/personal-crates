use structopt::*;
use structopt_utilities::StructOptUtils;

#[derive(Debug, StructOpt)]
#[structopt(name = "break-line")]
struct CliArgs {
    #[structopt(name = "text")]
    pub text: String,
    #[structopt(name = "delimiter", default_value = ":")]
    pub delimiter: String,
}

fn main() {
    let CliArgs { text, delimiter } = CliArgs::strict_from_args();
    for component in text.split(&delimiter) {
        println!("{}", component);
    }
}
