#[derive(Clone, Ord, PartialEq, Eq, PartialOrd)]
pub enum FieldValue {
    U64(u64),
    I64(i64),
    Text(String),
}

impl ToString for FieldValue {
    fn to_string(&self) -> String {
        match self {
            FieldValue::U64(v) => v.to_string(),
            FieldValue::I64(v) => v.to_string(),
            FieldValue::Text(v) => v.to_string(),
        }
    }
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

#[derive(Clone)]
pub struct FieldParseError;

const FIELD_REF_JOINER: &str = "/";

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct FieldRef {
    doc_ref: String,
    field_name: String,
}

impl FieldRef {
    pub fn new(doc_ref: String, field_name: String) -> FieldRef {
        FieldRef {
            doc_ref,
            field_name,
        }
    }

    pub fn doc_ref(&self) -> &str {
        &self.doc_ref
    }

    pub fn field_name(&self) -> &str {
        &self.field_name
    }

    pub fn from_string(s: String) -> Result<FieldRef, FieldParseError> {
        let splitten: Vec<&str> = s.splitn(2, FIELD_REF_JOINER).collect();
        if splitten.len() < 2 {
            return Err(FieldParseError);
        }
        Ok(FieldRef {
            field_name: splitten[0].to_string(),
            doc_ref: splitten[1].into(),
        })
    }

    pub fn to_str(&self) -> String {
        self.field_name.to_string() + FIELD_REF_JOINER + &self.doc_ref.to_string()
    }
}
