use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PoeVersions {
    pub poe: String,
    pub poe2: String,
}

pub fn get_versions() -> Result<PoeVersions> {
    let client = Client::new();
    let versions_url = "https://poe-versions.obsoleet.org/";
    client
        .get(versions_url)
        .send()
        .context("Failed to fetch versions")?
        .json()
        .context("Failed to parse versions json")
}
