use std::sync::Arc;

use crate::website::{WebsiteInfo, WebsiteState};
use async_trait::async_trait;

pub struct ProxyNotifier {
    notifier: Vec<Box<dyn Notifier>>,
}

#[async_trait]
pub trait Notifier {
    async fn notify(&self, info: Arc<WebsiteInfo>, state: &WebsiteState);
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::sync::Arc;
    use tokio::sync::mpsc;

    struct TestObj {
        info: Arc<WebsiteInfo>,
    }

    impl TestObj {
        fn new(name: &str, url: &str) -> Self {
            let info = Arc::new(WebsiteInfo::new(name, url));
            Self { info }
        }
    }

    #[tokio::test]
    async fn test_proxy_notification() -> Result<()> {
        let (tx, mut rx) = mpsc::channel::<Arc<WebsiteInfo>>(20);
        let jh = tokio::spawn(async move {
            while let Some(i) = rx.recv().await {
                println!("{:?}", i);
            }
        });
        let obj = TestObj::new(&"blup", &"http://blup.org");
        //let ti = Arc::new(TargetInfo::new(&"bla", &"http://127.0.0.1:80"));
        let ti = Arc::clone(&obj.info);
        let ta = Arc::new(WebsiteInfo::new(&"bla", &"http://127.0.0.1:80"));
        let otx = tx.clone();
        let oti = Arc::clone(&ti);
        let jh2 = tokio::spawn(async move {
            for _i in 0..5 {
                let oti = Arc::clone(&oti);
                otx.send(oti).await.expect("Must succeed");
                otx.send(Arc::clone(&ta)).await.expect("must succeeed");
            }
        });
        println!("{:?}", obj.info);
        drop(obj);
        tx.send(ti).await?;
        drop(tx);
        jh.await?;
        jh2.await?;
        Ok(())
    }
}
