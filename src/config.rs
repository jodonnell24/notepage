use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Config {
    // Website title, like "My Awesome Notes"
    pub title: String,

    // Base URL for the page, like "https://mynotes.com"
    pub base_url: String,
}

impl Config {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        // Read the entire file contents into a String
        let txt = std::fs::read_to_string(path)?;

        // Parse the TOML string into our Config struct
        Ok(toml::from_str(&txt)?)
    }
}