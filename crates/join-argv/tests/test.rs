use command_extra::CommandExtra;
use pipe_trait::*;
use std::process::Command;

const PROGRAM: &str = env!("CARGO_BIN_EXE_join-argv");

fn output(mut command: Command) -> (String, String, i32) {
    let output = command.output().expect("get output from process");
    let stdout = output
        .stdout
        .pipe(String::from_utf8)
        .expect("decode stdout as UTF-8");
    let stderr = output
        .stderr
        .pipe(String::from_utf8)
        .expect("decode stderr as UTF-8");
    let status = output.status.code().expect("get status code");
    (stdout, stderr, status)
}

#[test]
fn separator_and_arguments() {
    assert_eq!(
        PROGRAM
            .pipe(Command::new)
            .with_arg(",")
            .with_arg("abc")
            .with_arg("def")
            .with_arg("foo bar")
            .pipe(output),
        ("abc,def,foo bar\n".to_string(), "".to_string(), 0),
    );
}

#[test]
fn just_separator() {
    assert_eq!(
        PROGRAM.pipe(Command::new).with_arg(",").pipe(output),
        ("\n".to_string(), "".to_string(), 0),
    );
}

#[test]
fn zero_arguments() {
    assert_eq!(
        PROGRAM.pipe(Command::new).pipe(output),
        (
            "".to_string(),
            "error: Not enough arguments\n".to_string(),
            1
        ),
    );
}
