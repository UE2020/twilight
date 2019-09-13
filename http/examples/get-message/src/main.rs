use dawn_http::Client;
use dawn_model::id::ChannelId;
use futures::future;
use std::{
    error::Error,
    env,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    pretty_env_logger::init_timed();

    let client = Client::new(env::var("DISCORD_TOKEN")?);
    let channel_id = ChannelId(511004549075566613);

    let msgs = future::join_all((1u8..=10).map(|x| {
        client.create_message(channel_id).content(format!("Ping #{}", x))
    })).await;

    let me = client.current_user().await?;
    println!("Current user: {}#{}", me.name, me.discriminator);

    Ok(())
}