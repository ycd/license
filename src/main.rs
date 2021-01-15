mod config;
use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    let licenses = &config::Licenses::new();

    let license = make_selection(&licenses.get_names());
    // TODO ask for year, with default current year
    // TODO ask for name, default name --git config user.name

    println!("{}", license)
}

fn make_selection(selections: &Vec<String>) -> String {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a license")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    selections[selection].clone()
}
