use crate::field::{Field, FieldValue};

pub struct Document {
    fields: Vec<Field>,
}

impl Document {
    pub fn new() -> Document {
        Document { fields: Vec::new() }
    }

    pub fn add(&mut self, field: Field) {
        self.fields.push(field)
    }

    pub fn get_field(&self, name: String) -> Option<&Field> {
        self.get_fields(name).first().map(|f| *f)
    }

    pub fn get_fields(&self, name: String) -> Vec<&Field> {
        self.fields
            .iter()
            .filter(|field| field.name() == name)
            .collect()
    }

    pub fn remove_field(&mut self, name: String) {
        if let Some(index) = self.fields.iter().position(|f| f.name() == name) {
            self.fields.remove(index);
        }
    }

    pub fn remove_fields(&mut self, name: String) {
        self.fields.retain(|f| f.name() == name)
    }

    pub fn get_all_fields(&self) -> Vec<&Field> {
        return self.fields.iter().collect();
    }

    pub fn get_values(&self, name: String) -> Vec<FieldValue> {
        self.fields
            .iter()
            .filter(|field| field.name() == name)
            .map(|field| field.value().clone())
            .collect()
    }

    pub fn get(&self, name: String) -> Option<FieldValue> {
        self.get_field(name).map(|field| field.value().clone())
    }
}
