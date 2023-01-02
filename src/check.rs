use reqwest::{self, ClientBuilder, Url, Method};
use anyhow::{anyhow, bail, ensure, Context, Result};
use tokio::time::{self, Duration, Instant};
use super::config;
use log::{self, debug, info, warn};


pub type MyJoinSet = tokio::task::JoinSet<()>;


pub struct WebsiteChecker {
    targets: Vec<Target>,
}
impl WebsiteChecker {
    pub fn new(cfg: &config::Config) -> Self {
        let targets = Self::collect_targets(cfg);
        Self {
            targets
        }
    }

    pub async fn watch(self) -> MyJoinSet {
        let mut set = MyJoinSet::new();
        let now = Instant::now();

        for target in self.targets {
            let t_start = now.clone();
            set.spawn(async move {
                target.watch(t_start).await;
            });
        }

        set
    }

    fn collect_targets(cfg: &config::Config) -> Vec<Target> {
        cfg.websites
            .iter()
            .filter_map(|(n, c)| {
                Target::from_config(n, c, &cfg.global)
                    .ok()
            })
            .collect()
    }
}



struct Target {
    url: Url,
    method: Method,
    interval: Duration,
}
impl Target {
    pub fn from_config(name: &str, cfg: &config::Website, global: &config::Global) -> Result<Self> {
        let url = Url::parse(&cfg.url)?;
        let method = parse_method(&cfg.method)?;
        let t_s = cfg.interval.unwrap_or(global.default_interval);
        let interval = Duration::from_secs(t_s);
        let obj = Self {
            url,
            method,
            interval,
        };

        Ok(obj)
    }

    pub async fn watch(self, t_start: Instant) {
        let mut interval = time::interval_at(
            t_start,
            self.interval,
        );
        let mut count = 0_u64;
        let mut last: Option<bool> = None;

        loop {
            interval.tick().await;
            let ret = self.check().await;
            debug!("{}: {:?}", self.url, ret);
            last = self.calc_state(last, ret.is_ok());
            if ret.is_ok() {
                count += 1;
            } else {
                count = 0;
            }
        }
    }

    fn calc_state(&self, last: Option<bool>, ok: bool) -> Option<bool> {
        if let Some(last) = last {
            if last != ok {
                self.report(ok);
            }
        } else {
            self.report(ok);
        }

        Some(ok)
    }

    fn report(&self, ok: bool) {
        if ok {
            self.report_success();
        } else {
            self.report_failure();
        }
    }

    fn report_success(&self) {
        info!("{} is up", self.url);
    }

    fn report_failure(&self) {
        warn!("{} is down", self.url);
    }

    async fn check(&self) -> Result<()> {
        let client = reqwest::ClientBuilder::new()
            .build()
            .expect("client");
        let r = client
            .request(
                self.method.clone(),
                self.url.clone()
            )
            .send()
            .await?;
        let status = r.status();
        if status != 200 {
            bail!("Url {} not reachable (status={})",
                  self.url, status);
        }

        Ok(())
    }
}

fn parse_method(m: &config::Method) -> Result<Method> {
    let method = match m {
        config::Method::HEAD => Method::HEAD,
        config::Method::GET => Method::GET,
        config::Method::POST => Method::POST,
        _ => bail!("Unsupported method: {:?}", m),
    };
    Ok(method)
}
