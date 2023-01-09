use anyhow::Result;
use reqwest;
use std::fmt::Display;
use tokio::time::Duration;

#[derive(Debug)]
pub struct WebsiteInfo {
    pub name: String,
    pub url: String,
}
impl WebsiteInfo {
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WebsiteState {
    Up(Duration),
    Timeout,
    Down,
}

impl WebsiteState {
    pub fn new(ret: &Result<Duration>) -> Self {
        match ret {
            Ok(t) => Self::Up(*t),
            Err(e) => {
                if is_reqwest_timeout(e) {
                    Self::Timeout
                } else {
                    Self::Down
                }
            }
        }
    }

    pub fn is_up(&self) -> bool {
        match self {
            WebsiteState::Up(_) => true,
            _ => false,
        }
    }

    pub fn is_down(&self) -> bool {
        match self {
            WebsiteState::Down => true,
            _ => false,
        }
    }

    pub fn is_timeout(&self) -> bool {
        match self {
            WebsiteState::Timeout => true,
            _ => false,
        }
    }

    pub fn is_same_kind(&self, other: &Self) -> bool {
        match self {
            WebsiteState::Up(_) => other.is_up(),
            WebsiteState::Timeout => other.is_timeout(),
            WebsiteState::Down => other.is_down(),
        }
    }
}

impl Display for WebsiteState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebsiteState::Up(t) => write!(f, "Up ({:?})", t),
            WebsiteState::Timeout => write!(f, "Timeout"),
            WebsiteState::Down => write!(f, "Down"),
        }
    }
}

fn is_reqwest_timeout(err: &anyhow::Error) -> bool {
    if let Some(e) = err.downcast_ref::<reqwest::Error>() {
        e.is_timeout()
    } else {
        false
    }
}
