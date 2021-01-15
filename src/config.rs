// Used in the first prototype

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct License {
    pub key: String,
    pub name: String,
    pub spdx_id: String,
    pub url: String,
    pub node_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Licenses {
    pub license: Vec<License>,
}

impl Licenses {
    pub fn new() -> Licenses {
        let body: Vec<License> = match ureq::get("https://api.github.com/licenses").call() {
            Ok(response) => response.into_json().unwrap(),
            Err(error) => panic!("Unable to fetch licenses: {:?}", error),
        };

        Licenses { license: body }
    }

    pub fn get_names(&self) -> Vec<String> {
        self.license.iter().map(|l| String::from(&l.name)).collect()
    }
}
