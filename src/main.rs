mod licenses;
mod utils;

fn main() {
    app()
}

fn app() {
    let licenses = licenses::Licenses::new();

    let license_name = utils::make_selection(&licenses.get_names());

    let license = &licenses.get_license_from_name(&license_name);

    utils::logic(license, true);
}
