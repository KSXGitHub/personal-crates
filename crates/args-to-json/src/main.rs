use std::{env::args, io::stdout};

fn main() -> Result<(), serde_json::Error> {
    let args = args().skip(1).collect::<Vec<_>>();
    serde_json::to_writer(stdout(), &args)?;
    println!();
    Ok(())
}
