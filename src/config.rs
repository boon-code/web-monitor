use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::BTreeMap;
use anyhow::{anyhow, bail, ensure, Context, Result};
use serde::{Serialize, Deserialize};
use serde_yaml;


#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub global: Global,
    #[serde(default)]
    pub websites: BTreeMap<String, Website>,
    #[serde(default)]
    pub notifiers: BTreeMap<String, Notifier>,
}
impl Config {
    pub fn from_yaml_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let file = File::open(path)
            .with_context(|| format!("Expect yaml config file '{}' to exist", path.display()))?;
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader)
            .with_context(|| format!("Failed to parse yaml config file '{}'", path.display()))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Global {
    #[serde(default = "Global::get_default_interval")]
    pub default_interval: u64,
}
impl Global {
    fn get_default_interval() -> u64 {
        60_u64
    }
}
impl Default for Global {
    fn default() -> Self {
        Self {
            default_interval: Self::get_default_interval(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Website {
    pub url: String,
    #[serde(default)]
    pub method: Method,
    #[serde(default)]
    pub request: String,
    pub interval: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Method {
    HEAD,
    GET,
    POST,
}
impl Default for Method {
    fn default() -> Self {
        Self::HEAD
    }
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub cmd: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Notifier {
    Command(Command),
}
impl Notifier {
    pub fn as_command(&self) -> Option<&Command> {
        match self {
            Self::Command(cmd) => Some(cmd),
            _ => None,
        }
    }
}


#[cfg(test)]
mod tests;
