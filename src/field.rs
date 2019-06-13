#[derive(Clone, Ord, PartialEq, Eq, PartialOrd)]
pub enum FieldValue {
    U64(u64),
    I64(i64),
    Text(String),
}

pub struct Field {
    name: String,
    value: FieldValue,
}

impl Field {
    pub fn new_text(name: String, value: String) -> Field {
        Field {
            name,
            value: FieldValue::Text(value),
        }
    }

    pub fn new_u64(name: String, value: u64) -> Field {
        Field {
            name,
            value: FieldValue::U64(value),
        }
    }

    pub fn new_i64(name: String, value: i64) -> Field {
        Field {
            name,
            value: FieldValue::I64(value),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &FieldValue {
        &self.value
    }
}
