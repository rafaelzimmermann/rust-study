use assert_cmd::Command;

#[test]
fn it_breaks_when_file_does_not_exists() {
    let mut cmd = Command::cargo_bin("zcat").unwrap();
    cmd.arg("unkown_file.log");
    cmd.assert().failure().stderr("zcat: No such file or directory (os error 2)\n");
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
    cmd.assert().success().stdout("Line 1\n\nLine 2");
}

#[test]
fn it_prints_line_counter_on_front_of_each_line() {
    let mut cmd = Command::cargo_bin("zcat").unwrap();
    cmd.arg("-n");
    cmd.arg("tests/inputs/file_with_2_lines.txt");
    cmd.assert().success().stdout("1 Line 1\n2 \n3 Line 2");
}

#[test]
fn it_prints_line_counter_on_front_of_each_non_empty_line() {
    let mut cmd = Command::cargo_bin("zcat").unwrap();
    cmd.arg("-b");
    cmd.arg("tests/inputs/file_with_2_lines.txt");
    cmd.assert().success().stdout("1 Line 1\n\n2 Line 2");
}

#[test]
fn it_prints_files_that_exists() {
    let mut cmd = Command::cargo_bin("zcat").unwrap();
    cmd.arg("unkown_file.log");
    cmd.arg("tests/inputs/success.txt");
    cmd.assert()
        .failure()
        .stderr("zcat: No such file or directory (os error 2)\n")
        .stdout("This file exists");
}
