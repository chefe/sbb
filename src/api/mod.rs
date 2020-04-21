extern crate reqwest;
extern crate serde;

mod models;

pub use self::models::Connection;
pub use self::models::Journey;
pub use self::models::Location;
pub use self::models::Section;
pub use self::models::Stop;
pub use self::models::Walk;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct LocationsResponse {
    stations: Vec<Location>,
}

#[derive(Deserialize, Debug)]
struct ConnectionsResponse {
    connections: Vec<Connection>,
}

pub fn search_location(query: &str) -> Result<Vec<String>, reqwest::Error> {
    let url = format!(
        "http://transport.opendata.ch/v1/locations?query={query}",
        query = query
    );

    let response = reqwest::blocking::get(&url)?.json::<LocationsResponse>()?;
    let locations = response.stations.into_iter().map(|s| s.name).collect();

    Ok(locations)
}

pub fn search_connection(
    from: &str,
    to: &str,
    vias: Vec<String>,
) -> Result<Vec<Connection>, reqwest::Error> {
    let vias = vias
        .iter()
        .map(|via| format!("&via[]={}", via))
        .collect::<Vec<String>>()
        .join("");

    let url = format!(
        "http://transport.opendata.ch/v1/connections?from={from}&to={to}{vias}",
        from = from,
        to = to,
        vias = vias,
    );

    let response = reqwest::blocking::get(&url)?.json::<ConnectionsResponse>()?;

    Ok(response.connections)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_a_non_empty_list_for_a_valid_location() {
        let stations = search_location("Basel").unwrap();
        assert!(stations.len() > 0);
    }

    #[test]
    fn it_returns_an_empty_list_for_an_empty_location() {
        let stations = search_location("").unwrap();
        assert_eq!(stations.len(), 0);
    }

    #[test]
    fn it_returns_an_empty_list_for_a_invalid_location() {
        let stations = search_location("ABCDEFG").unwrap();
        assert_eq!(stations.len(), 0);
    }

    #[test]
    fn it_returns_a_non_empty_list_for_a_valid_connection() {
        let connections = search_connection("Zug", "Chur", vec![]).unwrap();
        assert!(connections.len() > 0);
    }
}
