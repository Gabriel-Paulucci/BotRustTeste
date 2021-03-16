use serenity::{async_trait, client::{Context, EventHandler}, model::prelude::Ready};

pub struct Events;

#[async_trait]
impl EventHandler for Events {
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("Bot on");
    }
}