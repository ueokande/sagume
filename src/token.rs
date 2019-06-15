#[derive(Eq, PartialEq, Clone)]
pub struct Token {
    pub index: usize,
    pub start: usize,
    pub value: String,
}
