use reqwest::blocking::Client;
use serde::{Serialize,Deserialize};

pub struct Ctf {
    http_client: Client,
    api_url: String,
}

impl Ctf {
    pub fn new(api_url: String) -> Ctf {
        let construction = Ctf {
            http_client: Client::new(),
            api_url: api_url,
        };

        let version_url = format!("{}/stats/version/", &construction.api_url);

        // TODO: Check the API endpoint actually exists and returns valid JSON
        let body = construction.http_client.get(&version_url).send();

        construction
    }
}