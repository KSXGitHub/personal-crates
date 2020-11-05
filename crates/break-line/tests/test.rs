use command_extra::CommandExtra;
use pipe_trait::*;
use std::process::Command;

const PROGRAM: &str = env!("CARGO_BIN_EXE_break-line");

fn program() -> Command {
    Command::new(PROGRAM)
}

fn output(mut command: Command) -> (String, String, bool) {
    let output = command.output().expect("get output from a command");
    let stdout = output
        .stdout
        .pipe(String::from_utf8)
        .expect("decode stdout as UTF-8");
    let stderr = output
        .stderr
        .pipe(String::from_utf8)
        .expect("decode stderr as UTF-8");
    (stdout, stderr, output.status.success())
}

#[test]
fn default_delimiter() {
    assert_eq!(
        program().with_arg("abc:def:ghi").pipe(output),
        (
            vec!["abc", "def", "ghi", ""].join("\n"),
            "".to_string(),
            true
        ),
    )
}

#[test]
fn specified_delimiter() {
    assert_eq!(
        program()
            .with_arg("abc,def:ghi,jkl:mno")
            .with_arg(",")
            .pipe(output),
        (
            vec!["abc", "def:ghi", "jkl:mno", ""].join("\n"),
            "".to_string(),
            true
        ),
    )
}
