use anyhow::{anyhow, bail, ensure, Context, Result};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

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
    // Default polling interval to check the status of a website in seconds
    pub default_interval: u64,
    #[serde(default = "Global::get_default_timeout")]
    // Default timeoutfor requesting a website in milliseconds
    pub default_timeout: u64,
}
impl Global {
    fn get_default_interval() -> u64 {
        60_u64
    }

    fn get_default_timeout() -> u64 {
        10000_u64
    }
}
impl Default for Global {
    fn default() -> Self {
        Self {
            default_interval: Self::get_default_interval(),
            default_timeout: Self::get_default_timeout(),
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
    pub timeout: Option<u64>,
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
pub struct Telegram {
    pub token: String,
    pub chat: String,
}

#[derive(Serialize, Deserialize)]
pub enum Notifier {
    Command(Command),
    Telegram(Telegram),
}
impl Notifier {
    pub fn as_command(&self) -> Option<&Command> {
        match self {
            Self::Command(cmd) => Some(cmd),
            _ => None,
        }
    }

    pub fn as_telegram(&self) -> Option<&Telegram> {
        match self {
            Self::Telegram(tb) => Some(tb),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests;
