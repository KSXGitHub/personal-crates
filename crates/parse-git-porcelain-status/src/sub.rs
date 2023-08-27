use crate::{cli_args::Column, parse::Value};
use pipe_trait::Pipe;
use std::io::BufRead;

fn read_stdin_lines(stdin: impl BufRead, sub: impl Fn(Value)) {
    for line in stdin.lines() {
        let line = line.expect("Read line");
        Value::parse_single_line(&line)
            .unwrap_or_else(|| panic!("Failed to parse {line:?} as a Value"))
            .pipe(&sub);
    }
}

pub fn select(stdin: impl BufRead, column: Column) {
    read_stdin_lines(stdin, |value| match column {
        Column::Staged => println!("{}", value.staged),
        Column::Unstaged => println!("{}", value.unstaged),
        Column::Path => println!("{}", value.path),
    });
}

pub fn all(stdin: impl BufRead) {
    read_stdin_lines(stdin, |value| println!("{value:#?}"));
}
