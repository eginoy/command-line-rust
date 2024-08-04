use assert_cmd::Command;

#[test]
fn runs(){
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, World!\n");

    // let mut cmd = Command::cargo_bin("hello").unwrap();
    // let res = cmd.output();
    // assert!(res.is_ok());
}

#[test]
fn true_ok(){
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok(){
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn check_ffmpeg_installed(){
    let mut ffmpeg_cmd = Command::new("ffmpeg");
    let res = ffmpeg_cmd.output();
    assert!(res.is_ok());
}