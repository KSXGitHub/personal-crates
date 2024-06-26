use command_extra::CommandExtra;
use pipe_trait::*;
use std::process::Command;

#[test]
fn test() {
    let output = env!("CARGO_BIN_EXE_args-to-json")
        .pipe(Command::new)
        .with_arg("one")
        .with_arg("foo bar")
        .with_arg("abc def ghi")
        .output()
        .expect("get output");

    let stdout = output
        .stdout
        .pipe(String::from_utf8)
        .expect("decode stdout as UTF-8");

    let stderr = output
        .stderr
        .pipe(String::from_utf8)
        .expect("decode stderr as UTF-8");

    let mut expected = ["one", "foo bar", "abc def ghi"]
        .pipe_ref(serde_json::to_string)
        .expect("get expected json");
    expected += "\n";

    assert_eq!(
        (stdout, stderr, output.status.success()),
        (expected, "".to_string(), true,),
    )
}
