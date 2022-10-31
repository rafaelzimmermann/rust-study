use assert_cmd::Command;

#[test]
fn it_breaks_when_no_args_are_provided() {
    Command::cargo_bin("zecho")
        .unwrap()
        .assert()
        .failure();
}

#[test]
fn it_output_given_argumntes() {
    let mut cmd = Command::cargo_bin("zecho").unwrap();
    cmd.arg("test");
    cmd.assert().success().stdout("test\n");
}