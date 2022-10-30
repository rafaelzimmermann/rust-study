use std::process::Command;

#[test]
fn works() {
    let mut cmd = Command::new("./target/debug/rust-cmd");
    let res = cmd.output();
    assert!(res.is_ok());
}