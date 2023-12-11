use teloxide::{prelude::*, update_listeners::webhooks};
use std::env;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting ngrok ping-pong bot...");

    let bot = Bot::from_env();

    // Access environment variables for port and webhook url
    const PORT: u16 = env::var("PORT").unwrap().parse().unwrap();
    const WEBHOOK_URL: &str = env::var("WEBHOOK_URL").unwrap();

    let addr = ([127, 0, 0, 1], 8443).into();
    let url = "https://139.59.198.186".parse().unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    // Confirm that this point was reached with logging and flushing STDOUT
    log::info!("Listening for updates...");
    std::io::stdout().flush().unwrap();

    teloxide::repl_with_listener(
        bot,
        |bot: Bot, msg: Message| async move {
            bot.send_message(msg.chat.id, "pong").await?;
            Ok(())
        },
        listener,
    )
    .await;
}
