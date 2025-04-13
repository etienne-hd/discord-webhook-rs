use super::discord::Error;
use serde_json::{Value, json};

pub struct Author {
    name: Option<String>,
    url: Option<String>,
    icon_url: Option<String>,
}

pub struct Field {
    name: Option<String>,
    value: Option<String>,
    inline: Option<bool>,
}

pub struct Footer {
    text: Option<String>,
    icon_url: Option<String>,
}

pub struct Embed {
    author: Option<Author>,
    title: Option<String>,
    url: Option<String>,
    description: Option<String>,
    color: Option<u32>,
    fields: Vec<Field>,
    thumbnail: Option<String>,
    image: Option<String>,
    footer: Option<Footer>,
}

impl Author {
    pub fn new() -> Self {
        Author {
            name: None,
            url: None,
            icon_url: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }

    fn verify(&self) -> Result<(), Error> {
        if let Some(name) = &self.name {
            if name.len() > 256 {
                return Err(Error::MaxContent);
            }
        }

        Ok(())
    }

    pub fn build(&self) -> Result<Value, Error> {
        self.verify()?;

        let mut obj = serde_json::Map::new();

        if let Some(name) = &self.name {
            obj.insert("name".into(), Value::String(name.clone()));
        };

        if let Some(url) = &self.url {
            obj.insert("url".into(), Value::String(url.clone()));
        };

        Ok(Value::Object(obj))
    }
}

impl Field {
    pub fn new() -> Self {
        Field {
            name: None,
            value: None,
            inline: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn inline(mut self, inline: bool) -> Self {
        self.inline = Some(inline);
        self
    }

    fn verify(&self) -> Result<(), Error> {
        if let Some(name) = &self.name {
            if name.len() > 256 {
                return Err(Error::MaxContent);
            }
        }

        if let Some(value) = &self.value {
            if value.len() > 1024 {
                return Err(Error::MaxContent);
            }
        }

        Ok(())
    }

    pub fn build(&self) -> Result<Value, Error> {
        self.verify()?;

        let mut obj = serde_json::Map::new();

        if let Some(name) = &self.name {
            obj.insert("name".into(), Value::String(name.clone()));
        } else {
            return Err(Error::MissingNameField);
        };

        if let Some(value) = &self.value {
            obj.insert("value".into(), Value::String(value.clone()));
        } else {
            return Err(Error::MissingValueField);
        };

        if let Some(inline) = &self.inline {
            obj.insert("inline".into(), Value::Bool(inline.clone()));
        };

        Ok(Value::Object(obj))
    }
}

impl Footer {
    pub fn new() -> Self {
        Footer {
            text: None,
            icon_url: None,
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }

    fn verify(&self) -> Result<(), Error> {
        if let Some(text) = &self.text {
            if text.len() > 2048 {
                return Err(Error::MaxContent);
            }
        }

        Ok(())
    }

    pub fn build(&self) -> Result<Value, Error> {
        self.verify()?;

        let mut obj = serde_json::Map::new();

        if let Some(text) = &self.text {
            obj.insert("text".into(), Value::String(text.clone()));
        };

        if let Some(icon_url) = &self.icon_url {
            obj.insert("icon_url".into(), Value::String(icon_url.clone()));
        };

        Ok(Value::Object(obj))
    }
}

impl Embed {
    pub fn new() -> Self {
        Embed {
            author: None,
            title: None,
            url: None,
            description: None,
            color: None,
            fields: Vec::new(),
            thumbnail: None,
            image: None,
            footer: None,
        }
    }

    pub fn author(mut self, author: Author) -> Self {
        self.author = Some(author);
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn add_field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    pub fn thumbnail(mut self, thumbnail: impl Into<String>) -> Self {
        self.thumbnail = Some(thumbnail.into());
        self
    }

    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.image = Some(image.into());
        self
    }

    pub fn footer(mut self, footer: Footer) -> Self {
        self.footer = Some(footer);
        self
    }

    fn verify(&self) -> Result<(), Error> {
        if let Some(title) = &self.title {
            if title.len() > 256 {
                return Err(Error::MaxContent);
            }
        }

        if let Some(description) = &self.description {
            if description.len() > 4096 {
                return Err(Error::MaxContent);
            }
        }

        if let Some(color) = self.color {
            // Max decimal color (R: 255, G: 255, B: 255)
            if color > (1 << 8 * 3) - 1 {
                return Err(Error::InvalidColor);
            }
        }

        if self.fields.len() > 25 {
            return Err(Error::MaxField);
        }

        Ok(())
    }

    pub fn build(&self) -> Result<Value, Error> {
        self.verify()?;

        let mut obj = serde_json::Map::new();

        if let Some(author) = &self.author {
            obj.insert("author".into(), author.build()?);
        }

        if let Some(footer) = &self.footer {
            obj.insert("footer".into(), footer.build()?);
        }

        if let Some(title) = &self.title {
            obj.insert("title".into(), Value::String(title.clone()));
        }

        if let Some(url) = &self.url {
            obj.insert("url".into(), Value::String(url.clone()));
        }

        if let Some(description) = &self.description {
            obj.insert("description".into(), Value::String(description.clone()));
        }

        if let Some(color) = &self.color {
            obj.insert("color".into(), json!(self.color));
        }

        if let Some(thumbnail) = &self.thumbnail {
            obj.insert("thumbnail".into(), json!({"url": thumbnail}));
        }

        if let Some(image) = &self.image {
            obj.insert("image".into(), json!({"url": image}));
        }

        obj.insert("fields".into(), Value::Array(Vec::new()));
        if let Value::Array(ref mut fields) = obj["fields"] {
            for field in &self.fields {
                fields.push(field.build()?);
            }
        };

        // An embed can be sent if it contains at least one of these key
        if obj.contains_key("author")
            || obj.contains_key("footer")
            || obj.contains_key("title")
            || obj.contains_key("description")
            || obj.contains_key("thumbnail")
            || obj.contains_key("image")
            || self.fields.len() > 0
        {
            Ok(Value::Object(obj))
        } else {
            Err(Error::InvalidEmbed)
        }
    }
}
