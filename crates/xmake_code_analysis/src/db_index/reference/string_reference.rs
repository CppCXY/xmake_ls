use rowan::TextRange;
use smol_str::SmolStr;
use std::collections::HashMap;

#[derive(Debug)]
pub struct StringReference {
    string_references: HashMap<SmolStr, Vec<TextRange>>,
}

impl StringReference {
    pub fn new() -> Self {
        Self {
            string_references: HashMap::new(),
        }
    }

    pub fn add_string_reference(&mut self, string: &str, range: TextRange) {
        self.string_references
            .entry(SmolStr::new(string))
            .or_insert_with(Vec::new)
            .push(range);
    }

    pub fn get_string_references(&self, string: &str) -> Vec<TextRange> {
        self.string_references
            .get(string)
            .cloned()
            .unwrap_or_default()
    }
}
