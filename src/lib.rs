use reqwest::{ClientBuilder, Method};
use anyhow::{anyhow, bail, ensure, Context, Result};
use tokio::{self, signal, task::JoinSet};
use tokio::time::{self, Duration};
mod config;
mod check;
mod arguments;

pub use arguments::Args;

type MyTestJoinSet = JoinSet<Result<()>>;


async fn my_periodic_task(now: time::Instant, seconds: u64) -> Result<()> {
    let mut interval = time::interval_at(
        now,
        Duration::from_secs(seconds)
    );
    let mut count = 0_u64;
    loop {
        interval.tick().await;
        count += 1;
        println!("periodic check {} every {} seconds",
                 count, seconds);
    }

    Ok(())
}

async fn join_all_test(mut set: MyTestJoinSet) {
    while let Some(i) = set.join_next().await {
        println!("joined {:?}", i);
    }
    println!("all joined");
}

async fn run_test() -> Result<()> {
    let mut set = MyTestJoinSet::new();
    let now = time::Instant::now();
    for i in 0..5 {
        let seconds = 1_u64 + i * 1_u64;
        println!("Spawn task for {}", seconds);
        let start_t = now.clone();
        set.spawn(async move {
            my_periodic_task(start_t, seconds).await
        });
    }

    tokio::select! {
        _ = join_all_test(set) => { println!("tasks"); },
        _ = signal::ctrl_c() => {
            println!("");
            println!("ctrl-c");
        },
    }

    Ok(())
}

pub async fn run(args: Args) -> Result<()> {
    //run_test().await
    let mut cfg = config::Config::from_yaml_file(args.file_path)?;
    if let Some(default_interval) = args.check_time_s {
        cfg.global.default_interval = default_interval;
    }
    let chk = check::WebsiteChecker::new(&cfg);
    let set = chk.watch().await;

    tokio::select! {
        _ = join_all(set) => { println!("tasks"); },
        _ = signal::ctrl_c() => {
            println!("");
            println!("ctrl-c");
        },
    }

    Ok(())
}

async fn join_all<T: 'static>(mut set: JoinSet<T>) {
    while let Some(_) = set.join_next().await { }
}

#[cfg(test)]
mod testutils;


#[cfg(test)]
mod tests {
    use tokio;
    use std::time::Duration;
    use std::str::FromStr;
    use reqwest;

    #[tokio::test]
    async fn simple_sleep() {
        tokio::time::sleep(Duration::from_millis(5)).await;
        assert_eq!(1u32, 1u32);
    }

    #[tokio::test]
    async fn test_simple_request() {
        let client = reqwest::ClientBuilder::new()
            .build()
            .expect("client");
        let r = client.head("https://www.rust-lang.org")
            .send()
            .await
            .unwrap();
        assert_eq!(r.status(), 200);
    }

    #[tokio::test]
    async fn test_simple_request2() {
        let url = "https://www.google.com";
        let method = reqwest::Method::from_str("HEAD").unwrap();
        let client = reqwest::ClientBuilder::new()
            .build()
            .expect("client");
        let r = client
            .request(method, url)
            .send()
            .await
            .unwrap();
        assert_eq!(r.status(), 200);
    }
}
