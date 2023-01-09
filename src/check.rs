use super::config;
use anyhow::{anyhow, bail, ensure, Context, Result};
use log::{self, debug, info, warn};
use reqwest::{self, ClientBuilder, Method, Url};
use std::sync::Arc;
use tokio::time::{self, Duration, Instant};
use crate::website::{WebsiteState, WebsiteInfo};

pub type MyJoinSet = tokio::task::JoinSet<()>;

pub struct WebsiteChecker {
    targets: Vec<Target>,
}
impl WebsiteChecker {
    pub fn new(cfg: &config::Config) -> Self {
        let targets = Self::collect_targets(cfg);
        Self { targets }
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
            .filter_map(|(n, c)| Target::from_config(n, c, &cfg.global).ok())
            .collect()
    }
}

struct Target {
    url: Url,
    method: Method,
    interval: Duration,
    timeout: Duration,
    info: Arc<WebsiteInfo>,
    last: Option<WebsiteState>,
}
impl Target {
    pub fn from_config(name: &str, cfg: &config::Website, global: &config::Global) -> Result<Self> {
        let url = Url::parse(&cfg.url)?;
        let method = parse_method(&cfg.method)?;
        let interval = cfg.interval.unwrap_or(global.default_interval);
        let interval = Duration::from_secs(interval);
        let timeout = cfg.timeout.unwrap_or(global.default_timeout);
        let timeout = Duration::from_millis(timeout);
        let info = Arc::new(WebsiteInfo::new(name, &cfg.url));
        let obj = Self {
            url,
            method,
            interval,
            timeout,
            info,
            last: None,
        };

        Ok(obj)
    }

    pub async fn watch(mut self, t_start: Instant) {
        let mut interval = time::interval_at(t_start, self.interval);
        let mut count = 0_u64;

        info!(
            "Start watching {}, interval={:?}, timeout={:?}",
            self.url, self.interval, self.timeout
        );

        loop {
            interval.tick().await;
            let ret = self.check().await;
            let state = WebsiteState::new(&ret);
            if ret.is_ok() {
                count += 1;
            } else {
                count = 0;
            }
            debug!("{}: {} {}", self.url, state, count);
            self.do_report(state);
        }
    }

    fn do_report(&mut self, state: WebsiteState) {
        if let Some(last) = &self.last {
            if !last.is_same_kind(&state) {
                self.report_state(&state);
            }
        } else {
            self.report_state(&state);
        }
        self.last.replace(state);
    }

    fn report_state(&self, state: &WebsiteState) {
        match state {
            WebsiteState::Up(t) => self.report_success(t),
            WebsiteState::Timeout => self.report_timeout(),
            WebsiteState::Down => self.report_failure(),
        }
    }

    fn report_success(&self, t: &Duration) {
        info!("{} is up ({:?})", self.url, t);
    }

    fn report_timeout(&self) {
        warn!(
            "{} request timed out (timeout={:?})",
            self.url, self.timeout
        );
    }

    fn report_failure(&self) {
        warn!("{} is down", self.url);
    }

    async fn check(&self) -> Result<Duration> {
        let t_start = Instant::now();
        let client = reqwest::ClientBuilder::new()
            .timeout(self.timeout)
            .build()
            .expect("client");
        let r = client
            .request(self.method.clone(), self.url.clone())
            .send()
            .await?;
        let dt = Instant::now().duration_since(t_start);
        let status = r.status();
        if status != 200 {
            bail!("Url {} not reachable (status={})", self.url, status);
        }

        Ok(dt)
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
