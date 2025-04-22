use countries::{hello_canada, NorthAmericanCountries};

uniffi::setup_scaffolding!("map_na");

#[derive(uniffi::Object)]
pub struct Travel {
    date: String,
    country: NorthAmericanCountries,
}

#[uniffi::export]
impl Travel {
    #[uniffi::constructor]
    pub fn new(date: String, country: NorthAmericanCountries) -> Self {
        Travel {
            date,
            country,
        }
    }

    pub fn hello(&self) -> String {
        match self.country {
            NorthAmericanCountries::Canada => hello_canada(),
            NorthAmericanCountries::UnitedStates => "It's all right but you should travel to Canada instead!".to_string(),
            NorthAmericanCountries::Mexico => "It's too hot you should probably travel to Canada instead!".to_string(),
        }
    }

    pub fn date_planned(&self) -> String {
        self.date.clone()
    }
}
