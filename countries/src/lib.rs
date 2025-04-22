uniffi::setup_scaffolding!("countries");

#[derive(uniffi::Enum)]
pub enum NorthAmericanCountries {
    UnitedStates,
    Canada,
    Mexico,
}

#[derive(uniffi::Enum)]
pub enum EuropeanCountries {
    France,
    Spain,
    Italy,
}

#[uniffi::export]
pub fn hello_canada() -> String {
    "Hello, Canada!".to_string()
}

#[uniffi::export]
pub fn hello_ireland() -> String {
    "Hello, Ireland!".to_string()
}
