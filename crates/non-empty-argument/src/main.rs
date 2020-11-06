use std::env::args;

fn main() {
    if let Some(x) = args().skip(1).find(|x| !x.is_empty()) {
        println!("{}", x);
    }
}
