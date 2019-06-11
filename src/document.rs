pub struct IndexableField {
    pub name: String,
}

impl IndexableField {
    pub fn string_value(&self) -> &String {
        panic!("Not implemented");
    }

    pub fn binary_value(&self) -> &[u8] {
        panic!("Not implemented")
    }
}

pub struct Document {
    fields: Vec<IndexableField>,
}

impl Document {
    pub fn new() -> Document {
        Document { fields: Vec::new() }
    }

    pub fn add(&mut self, field: IndexableField) {
        self.fields.push(field)
    }

    pub fn remove_field(&mut self, name: String) {
        panic!("Not implemented")
    }

    pub fn remove_fields(&mut self, name: String) {
        self.fields.retain(|f| f.name == name)
    }

    pub fn get_binary_value(&mut self, name: String) -> Option<&[u8]> {
        return self.get_field(name).map(|f| f.binary_value());
    }

    pub fn get_binary_values(&mut self, name: String) -> Vec<&[u8]> {
        return self
            .get_fields(name)
            .iter()
            .map(|f| f.binary_value())
            .collect();
    }

    pub fn get_field(&self, name: String) -> Option<&IndexableField> {
        return Some(self.get_fields(name).first().unwrap());
    }

    pub fn get_fields(&self, name: String) -> Vec<&IndexableField> {
        return self.fields.iter().filter(|f| f.name == name).collect();
    }

    pub fn get_all_fields(&self) -> Vec<&IndexableField> {
        return self.fields.iter().collect();
    }

    pub fn get_values(&self, name: String) -> Vec<&String> {
        return self
            .fields
            .iter()
            .filter(|f| f.name == name)
            .map(|f| f.string_value())
            .collect();
    }

    pub fn get(&self, name: String) -> Option<&String> {
        return if let Some(f) = self.fields.iter().find(|f| f.name == name) {
            Some(&f.string_value())
        } else {
            None
        };
    }
}
