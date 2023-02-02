use anyhow::Result;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Set {
    pub commands: Vec<Command>,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub matches: String,
    pub output: String,
}

pub fn parse_toml(toml_string: String) -> Result<Set> {
    let set: Set = toml::from_str(&toml_string)?;

    Ok(set)
}
