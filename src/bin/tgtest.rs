use std::env;
use anyhow::Result;
use tokio::time::{self, Duration, Instant};
use tokio::signal;
use teloxide::prelude::*;

struct TelBot {
    chat_id: ChatId,
    bot: Bot,
}
impl TelBot {
    fn new(bot: Bot) -> Result<Self> {
        let chat_id = env::var("CHAT_ID")?;
        let chat_id = ChatId(chat_id.parse::<i64>()?);

        Ok(Self {
            chat_id,
            bot,
        })
    }

    async fn send_message(&self, text: String) -> Result<()> {
        self.bot.send_message(self.chat_id, text)
            .await?;
        Ok(())
    }
}

async fn send_interval(bot: TelBot, seconds: u64) {
    let t_start = Instant::now();
    let dur = Duration::from_secs(seconds);
    let mut interval = time::interval_at(t_start, dur);
    let mut count = 0_u64;

    loop {
        interval.tick().await;
        count += 1;
        let ret = bot.send_message(
            format!("Send message {}", count)
        ).await;
        println!("Send count {}: {:?}", count, ret);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let chat_id = env::var("CHAT_ID")?;
    let bot = Bot::from_env();
    let chat_id = ChatId(chat_id.parse::<i64>()?);
    bot.send_message(chat_id, "Hello from cli").await?;

    let bot2 = TelBot::new(bot.clone())?;
    let f1 = send_interval(bot2, 60_u64);

    let f2 = teloxide::repl(
        bot, |bot: Bot, msg: Message| async move {
            let txt = format!(
                "Chat id: {}",
                msg.chat.id
            );
            bot.send_message(msg.chat.id, txt)
                .await?;
                
            Ok(())
        }
    );

    tokio::select! {
        _ = f1 => { },
        _ = f2 => { },
        _ = signal::ctrl_c() => {
            println!("");
            println!("ctrl-c");
        },
    }

    Ok(())
}
