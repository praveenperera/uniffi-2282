uniffi::setup_scaffolding!();

pub type NorthAmericanCountries = countries::NorthAmericanCountries;

#[uniffi::export]
pub fn hello_canada() -> String {
    countries::hello_canada()
}
