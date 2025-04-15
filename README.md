# discord-webhook-rs
**A Rust library for sending Discord webhooks easily.**

## Features
- Send rich embeds with customizable fields, authors, and footers.
- Set custom content, username, and avatar for your webhook.
- Simple and intuitive API for sending messages to Discord channels.

## Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
discord-webhook-rs = "1.0.3"
```

## Example Usage

```rust
use discord_webhook_rs::{Webhook, Error, Embed, Author, Field, Footer};

const WEBHOOK_URL: &str = "https://discord.com/api/webhooks/...";

fn main() -> Result<(), Error> {
    Webhook::new(WEBHOOK_URL)
        .content("Hello from Rust!")
        .username("Étienne")
        .avatar_url("https://avatars.githubusercontent.com/u/145381924")
        .add_embed(
            Embed::new()
                .title("Thanks")
                .description("Thanks for using **discord-webhook-rs**!")
                .author(Author::new().name("Crab"))
                .add_field(Field::new().name("Field 1").value("Value 1").inline(true))
                .add_field(Field::new().name("Field 2").value("Value 2").inline(false))
                .footer(Footer::new().text("footer")),
        )
        .send()?;

    Ok(())
}
```

## License
This project is licensed under the MIT License, see the [LICENSE](LICENSE) file for details.