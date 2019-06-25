use crate::token::Token;

pub struct Tokenizer;

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer
    }

    pub fn tokenize(&self, source: String) -> Vec<Token> {
        let source = source.trim_end().to_lowercase();
        let mut start = 0;
        let mut tokens: Vec<Token> = Vec::new();

        let chars: Vec<char> = source.chars().collect();
        let len = source.len();
        for end in 0..len + 1 {
            if end == len || chars[end].is_whitespace() || chars[end] == '-' {
                if end - start > 0 {
                    let value = &source[start..end];
                    let token = Token {
                        index: tokens.len(),
                        start,
                        value: value.to_string(),
                    };
                    tokens.push(token)
                }
                start = end + 1;
            }
        }
        tokens
    }
}
