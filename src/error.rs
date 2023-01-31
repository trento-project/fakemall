#[derive(Debug)]
pub enum FakeshError {
    IoError(std::io::Error),
    TomlError(toml::de::Error),
}

impl From<toml::de::Error> for FakeshError {
    fn from(v: toml::de::Error) -> Self {
        Self::TomlError(v)
    }
}

impl From<std::io::Error> for FakeshError {
    fn from(v: std::io::Error) -> Self {
        Self::IoError(v)
    }
}
