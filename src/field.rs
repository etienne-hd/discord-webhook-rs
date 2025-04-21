use super::Error;

use serde_json::Value;

pub struct Field {
    name: Option<String>,
    value: Option<String>,
    inline: Option<bool>,
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
        } else {
            return Err(Error::MissingNameField);
        }

        if let Some(value) = &self.value {
            if value.len() > 1024 {
                return Err(Error::MaxContent);
            }
        } else {
            return Err(Error::MissingValueField);
        }

        Ok(())
    }

    pub fn build(&self) -> Result<Value, Error> {
        self.verify()?;

        let mut obj = serde_json::Map::new();

        if let Some(name) = &self.name {
            obj.insert("name".into(), Value::String(name.clone()));
        }

        if let Some(value) = &self.value {
            obj.insert("value".into(), Value::String(value.clone()));
        }

        if let Some(inline) = &self.inline {
            obj.insert("inline".into(), Value::Bool(inline.clone()));
        }

        Ok(Value::Object(obj))
    }
}
