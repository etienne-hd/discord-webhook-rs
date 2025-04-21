use super::Error;

use serde_json::Value;

pub struct Footer {
    text: Option<String>,
    icon_url: Option<String>,
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
        }

        if let Some(icon_url) = &self.icon_url {
            obj.insert("icon_url".into(), Value::String(icon_url.clone()));
        }

        Ok(Value::Object(obj))
    }
}
