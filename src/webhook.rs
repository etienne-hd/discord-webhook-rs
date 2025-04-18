use super::Embed;

use reqwest::blocking::{
    Client, Response,
    multipart::{Form, Part},
};
use serde_json::{Map, Value};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    File(std::io::Error),
    MaxFile,
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
    files: Vec<String>,
    client: Client,
}

impl Webhook {
    pub fn new(url: impl Into<String>) -> Self {
        Webhook {
            url: url.into(),
            username: None,
            avatar_url: None,
            content: None,
            embeds: Vec::new(),
            files: Vec::new(),
            client: Client::new(),
        }
    }

    pub fn set_client(mut self, client: Client) -> Self {
        self.client = client;
        self
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

    pub fn add_file(mut self, file: impl Into<String>) -> Self {
        self.files.push(file.into());
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

    fn build_form(&self) -> Result<Form, Error> {
        if self.files.len() > 10 {
            return Err(Error::MaxFile);
        }

        let mut form = Form::new();

        for (i, file_path) in self.files.iter().enumerate() {
            let file_bytes = match fs::read(file_path) {
                Err(why) => return Err(Error::File(why)),
                Ok(data) => data,
            };

            let filename = Path::new(file_path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("file");

            let part = Part::bytes(file_bytes.to_vec()).file_name(filename.to_string());

            let part = match part.mime_str("application/octet-stream") {
                Err(why) => return Err(Error::Request(why)),
                Ok(data) => data,
            };

            form = form.part(format!("files[{}]", i), part);
        }

        Ok(form)
    }

    fn build_body(&self) -> Result<Value, Error> {
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
        let mut form = self.build_form()?;
        form = form.text("payload_json", self.build_body()?.to_string());

        let req = self.client.post(&self.url).multipart(form);

        match req.send() {
            Ok(response) => Ok(response),
            Err(error) => Err(Error::Request(error)),
        }
    }
}
