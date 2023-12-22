use std::collections::HashMap;
use std::fmt;

pub struct Header {
    fields: HashMap<String, String>,
}

impl Header {
    pub fn new() -> Self {
        Header {
            fields: HashMap::new(),
        }
    }

    pub fn parse(raw_headers: &str) -> Result<Self, &'static str> {
        let mut header = Header::new();
        for line in raw_headers.lines() {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() != 2 {
                return Err("Invalid header format");
            }
            let key = parts[0].trim().to_lowercase(); // HTTP headers are case-insensitive
            let value = parts[1].trim().to_string();
            header.fields.insert(key, value);
        }
        Ok(header)
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.fields.insert(key.to_lowercase(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.fields.get(&key.to_lowercase())
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (key, value) in &self.fields {
            write!(f, "{}: {}\r\n", key, value)?;
        }
        Ok(())
    }
}