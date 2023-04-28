use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("taskmaster").unwrap();
    cmd.assert().success();
}

#[test]
fn output() {
    let mut cmd = Command::cargo_bin("taskmaster").unwrap();
    cmd.assert().success().stdout("This is Taskmaster\n");
}

#[test]
fn taskmasterctl_runs() {
    let mut cmd = Command::cargo_bin("taskmasterctl").unwrap();
    cmd.assert().success();
}

#[test]
fn taskmasterd_runs() {
    let mut cmd = Command::cargo_bin("taskmasterd").unwrap();
    cmd.assert().success();
}
