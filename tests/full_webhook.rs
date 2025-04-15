use discord_webhook_rs::{Author, Embed, Error, Field, Footer, Webhook};
use reqwest::blocking::Client;

#[test]
fn full_webhook() -> Result<(), Error> {
    Webhook::new("https://discord.com/api/webhooks/...")
        .set_client(Client::new())
        .username("test")
        .avatar_url("https://avatars.githubusercontent.com/u/145381924")
        .content("test")
        .add_embed(
            Embed::new()
                .author(
                    Author::new()
                        .name("test")
                        .icon_url("https://avatars.githubusercontent.com/u/145381924")
                        .url("https://github.com/etienne-hd"),
                )
                .add_field(Field::new().name("test").value("test").inline(true))
                .add_field(Field::new().name("test").value("test").inline(false))
                .color((1 << 8 * 3) - 1)
                .description("test")
                .footer(
                    Footer::new()
                        .text("test")
                        .icon_url("https://avatars.githubusercontent.com/u/145381924"),
                )
                .thumbnail("https://avatars.githubusercontent.com/u/145381924")
                .image("https://avatars.githubusercontent.com/u/145381924")
                .url("https://github.com/etienne-hd"),
        )
        .send()?;

    Ok(())
}
