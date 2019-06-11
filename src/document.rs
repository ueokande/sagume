pub trait Field {}

pub struct StringField {}
pub struct TextField {}

impl StringField {
    pub fn new(name: String, value: String) -> StringField {
        StringField {}
    }
}

impl TextField {
    pub fn new(name: String, value: String) -> TextField {
        TextField {}
    }
}

impl Field for TextField {}
impl Field for StringField {}

pub struct Document {}

impl Document {
    pub fn new() -> Document {
        Document {}
    }

    pub fn add<T>(&self, field: T)
    where
        T: Field,
    {
        panic!("Not implemented")
    }
}
