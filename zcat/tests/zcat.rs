use assert_cmd::Command;

#[test]
fn it_breaks_when_file_does_not_exists() {
    let mut cmd = Command::cargo_bin("zcat").unwrap();
    cmd.arg("unkown_file.log");
    cmd.assert().failure().stderr("zcat: unkown_file.log: No such file or directory\n");
}

#[test]
fn it_prints_files() {
    let mut cmd = Command::cargo_bin("zcat").unwrap();
    cmd.arg("tests/inputs/success.txt");
    cmd.assert().success().stdout("This file exists");
}

#[test]
fn it_prints_file_with_multiple_lines() {
    let mut cmd = Command::cargo_bin("zcat").unwrap();
    cmd.arg("tests/inputs/file_with_2_lines.txt");
    cmd.assert().success().stdout("Line 1\nLine 2");
}

#[test]
fn it_prints_files_that_exists() {
    let mut cmd = Command::cargo_bin("zcat").unwrap();
    cmd.arg("unkown_file.log");
    cmd.arg("tests/inputs/success.txt");
    cmd.assert()
        .failure()
        .stderr("zcat: unkown_file.log: No such file or directory\n")
        .stdout("This file exists");
}
