use std::collections::HashMap;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Config {
    pub global: Global,
    pub plugins: Option<HashMap<String, Plugin>>,
    pub route: HashMap<String, String>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Global {
    pub path_session: String,
    pub log_file: String,
    pub port: Option<u32>,
    pub plugins_directory: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Plugin {
    Simple(String),
    Detailed(DetailedPlugin),
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DetailedPlugin {
    pub load: String,
    pub options: Option<HashMap<String, String>>,
}
