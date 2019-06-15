extern crate sagume;

use sagume::token::Token;
use sagume::tokenizer::Tokenizer;

#[test]
fn test_splitting_into_tokens() {
    let tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize("foo bar baz".into());
    assert_eq!(
        tokens.iter().map(|t| &t.value).collect::<Vec<&String>>(),
        vec!["foo", "bar", "baz"]
    );

    let tokens = tokenizer.tokenize("Foo Bar BAZ".into());
    assert_eq!(
        tokens.iter().map(|t| &t.value).collect::<Vec<&String>>(),
        vec!["foo", "bar", "baz"]
    );

    let tokens = tokenizer.tokenize("foo    bar - baz".into());
    assert_eq!(
        tokens.iter().map(|t| &t.value).collect::<Vec<&String>>(),
        vec!["foo", "bar", "baz"]
    );

    let tokens = tokenizer.tokenize("foo--bar-baz".into());
    assert_eq!(
        tokens.iter().map(|t| &t.value).collect::<Vec<&String>>(),
        vec!["foo", "bar", "baz"]
    );
}

#[test]
fn test_token_index() {
    let tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize("foo bar".into());
    assert_eq!(tokens[0].index, 0);
    assert_eq!(tokens[1].index, 1);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[1].start, 4);
}
