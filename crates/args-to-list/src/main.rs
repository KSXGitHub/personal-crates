use std::env::args;

fn main() {
    for arg in args().skip(1) {
        println!("{}", arg);
    }
}
