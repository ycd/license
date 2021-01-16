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

    pub fn get_license_from_name(&self, name: &String) -> CompleteLicense {
        let _license = &self.license;

        let result = _license
            .into_iter()
            .filter(|l| l.name == name.clone())
            .map(|l| l.url.clone())
            .collect();

        CompleteLicense::from(&result)
    }
}

#[derive(Debug, Deserialize)]
pub struct CompleteLicense {
    pub key: String,
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
    pub conditions: Vec<String>,
    pub limitations: Vec<String>,
    pub body: String,
}

impl CompleteLicense {
    pub fn from(url: &String) -> CompleteLicense {
        let license: CompleteLicense = match ureq::get(&url).call() {
            Ok(response) => response.into_json().unwrap(),
            Err(error) => panic!("Unable to fetch license: {:?}", error),
        };

        license
    }
}
