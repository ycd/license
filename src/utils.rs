use std::process::Command;

pub fn get_username() -> String {
    let command = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("--get")
        .arg("user.name")
        .output()
        .expect("fail");

    let res = match command.status.success() {
        true => String::from_utf8_lossy(&command.stdout).to_string(),
        false => String::new(),
    };

    res
}
