use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("taskmaster").unwrap();
    cmd.assert().success();
}
