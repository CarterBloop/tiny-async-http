use std::collections::HashMap;

pub struct Header {
    pub fields: HashMap<String, String>,
}

impl Header {
    // Create a new empty Header object
    pub fn new() -> Self {
        Header {
            fields: HashMap::new(),
        }
    }

    // Parse raw header string into a Header object
    pub fn parse(raw_headers: &str) -> Result<Self, &'static str> {
        let mut header = Header::new();
        for line in raw_headers.lines() {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() != 2 {
                return Err("Invalid header format");
            }
            let key = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();
            header.fields.insert(key, value);
        }
        Ok(header)
    }

    // Add a single header field
    pub fn add(&mut self, key: &str, value: &str) {
        self.fields.insert(key.to_string(), value.to_string());
    }

    // Convert Header object to a formatted string for response
    pub fn to_string(&self) -> String {
        self.fields
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>()
            .join("\r\n")
    }
}