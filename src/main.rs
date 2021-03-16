mod commands;
mod event;

use serenity::Client;
use commands::get_framework;
use event::Events;

#[tokio::main]
async fn main() -> Result<(), serenity::Error> {
    let mut client = Client::builder("NzMwMDk0Mjg3MzQ1MTU2MTM2.XwSfSg.IsMQ6MggYhgrmfkLdYohdlzRHx0")
        .framework(get_framework())
        .event_handler(Events)
        .await?;

    client.start().await?;

    Ok(())
}
