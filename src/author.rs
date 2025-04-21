use super::Error;

use serde_json::Value;

pub struct Author {
    name: Option<String>,
    url: Option<String>,
    icon_url: Option<String>,
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
        }

        if let Some(url) = &self.url {
            obj.insert("url".into(), Value::String(url.clone()));
        }

        if let Some(icon_url) = &self.icon_url {
            obj.insert("icon_url".into(), Value::String(icon_url.clone()));
        }

        Ok(Value::Object(obj))
    }
}
