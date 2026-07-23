use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Command {
    name: String,
    arguments: HashMap<String, String>,
}

impl Command {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            arguments: HashMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arguments(&self) -> &HashMap<String, String> {
        &self.arguments
    }

    pub fn insert_argument(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.arguments.insert(key.into(), value.into());
    }
}
