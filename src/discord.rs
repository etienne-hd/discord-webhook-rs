use super::Embed;

use reqwest::blocking::{Client, Response};
use serde_json::{Map, Value};

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    MaxContent,
    MaxEmbed,
    MaxField,
    InvalidColor,
    InvalidEmbed,
    MissingNameField,
    MissingValueField,
}

pub struct Webhook {
    url: String,
    username: Option<String>,
    avatar_url: Option<String>,
    content: Option<String>,
    embeds: Vec<Embed>,
}

impl Webhook {
    pub fn new(url: impl Into<String>) -> Self {
        Webhook {
            url: url.into(),
            username: None,
            avatar_url: None,
            content: None,
            embeds: Vec::new(),
        }
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn avatar_url(mut self, avatar_url: impl Into<String>) -> Self {
        self.avatar_url = Some(avatar_url.into());
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn add_embed(mut self, embed: Embed) -> Self {
        self.embeds.push(embed);
        self
    }

    fn verify(&self) -> Result<(), Error> {
        if let Some(content) = &self.content {
            if content.len() > 2000 {
                return Err(Error::MaxContent);
            }
        }

        if self.embeds.len() > 10 {
            return Err(Error::MaxEmbed);
        }

        Ok(())
    }

    fn build(&self) -> Result<Value, Error> {
        self.verify()?;

        let mut obj = Map::new(); // Construct Body

        if let Some(content) = &self.content {
            obj.insert("content".into(), Value::String(content.clone()));
        } else {
            obj.insert("content".into(), Value::String("Hello from Rust!".into()));
        }

        if let Some(username) = &self.username {
            obj.insert("username".into(), Value::String(username.clone()));
        }

        if let Some(avatar_url) = &self.avatar_url {
            obj.insert("avatar_url".into(), Value::String(avatar_url.clone()));
        }

        obj.insert("embeds".into(), Value::Array(vec![]));
        for embed in &self.embeds {
            if let Value::Array(ref mut embeds) = obj["embeds"] {
                embeds.push(embed.build()?);
            }
        }

        Ok(Value::Object(obj))
    }

    pub fn send(self) -> Result<Response, Error> {
        let client = Client::new();
        let req = client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .body(self.build()?.to_string());

        match req.send() {
            Ok(response) => Ok(response),
            Err(error) => Err(Error::Request(error)),
        }
    }
}
