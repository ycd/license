use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::process::Command;

/// Looks for a user name
/// By looking up the global git config
fn get_username() -> Option<String> {
    let command = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("--get")
        .arg("user.name")
        .output()
        .expect("fail");

    let res: Option<String> = match command.status.success() {
        true => Option::from(String::from_utf8_lossy(&command.stdout).to_string()),
        false => Option::from(None),
    };

    res
}

pub fn make_selection(selections: &Vec<String>) -> String {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a license")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    selections[selection].clone()
}

pub fn get_name() -> String {
    let name: String = match get_username() {
        Some(name) => {
            let name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your name:")
                .default(name)
                .interact_text()
                .unwrap();
            name
        }
        None => {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Your name")
                .interact_text()
                .unwrap();

            input
        }
    };

    name
}
