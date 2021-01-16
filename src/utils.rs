use dialoguer::console::Style;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use licenses::CompleteLicense;
use std::io::Write;
use std::{fs, process::Command};

use crate::licenses;

pub fn make_selection(selections: &Vec<String>) -> String {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a license")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    selections[selection].clone()
}

pub fn logic(license: &CompleteLicense, interactive: bool) {
    let name = get_name();
    let year = get_year();

    let body = license
        .body
        .replace("[year]", &year)
        .replace("[fullname]", &name)
        .replace("<year>", &year)
        .replace("<name of author>", &name);

    match write_to_file("LICENSE", &body) {
        Ok(_) => println!(
            "{}",
            Style::new()
                .for_stderr()
                .green()
                .apply_to("✔ license created successfully!")
        ),
        Err(err) => println!(
            "{} {}",
            Style::new()
                .for_stderr()
                .red()
                .apply_to("✘ an error occured"),
            err
        ),
    };
}

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

fn get_name() -> String {
    let name: String = match get_username() {
        Some(name) => {
            let name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your name")
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

fn get_year() -> String {
    let year: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter year")
        .default("2021".to_string())
        .interact_text()
        .unwrap();

    year
}

fn write_to_file(file_path: &str, to_write: &str) -> Result<(), std::io::Error> {
    let result = match !fs::metadata(file_path).is_ok() {
        false => {
            let file_path: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("LICENSE exists, enter a new name or it will be overriden!")
                .default(file_path.to_string())
                .interact_text()
                .unwrap();

            fs::write(file_path, to_write)
        }
        true => fs::write(file_path, to_write),
    };
    result
}
