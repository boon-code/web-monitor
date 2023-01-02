use async_trait::async_trait;

pub struct TargetInfo {
    name: String,
    url: String,
}
impl TargetInfo {
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
        }
    }
}

pub struct ProxyNotifier {
    notifier: Vec<Box<dyn Notifier>>,
}

#[async_trait]
pub trait Notifier {
    async fn notify(&self, info: &TargetInfo);
}
