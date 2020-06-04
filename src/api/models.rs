use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Location {
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Walk {
    pub duration: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Journey {
    pub name: String,
    pub category: String,
    pub number: String,
    pub operator: String,
    pub to: String,

    #[serde(rename = "passList")]
    pub pass_list: Vec<Stop>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Stop {
    pub station: Location,
    pub arrival: Option<String>,
    pub departure: Option<String>,
    pub delay: Option<u16>,
    pub platform: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Section {
    pub departure: Stop,
    pub arrival: Stop,
    pub journey: Option<Journey>,
    pub walk: Option<Walk>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Connection {
    pub from: Stop,
    pub to: Stop,
    pub duration: String,
    pub sections: Vec<Section>,
}
