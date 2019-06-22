use crate::token::TokenSet;
use crate::vector::Vector;
use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Clone)]
pub struct InvertedIndex {
    pub index: u64,
    pub documents: HashMap<String, HashSet<String>>, // field_name -> []document_ref
}

pub struct Index {
    pub inverted_index: HashMap<String, InvertedIndex>,
    pub field_vectors: HashMap<String, Vector>,
    pub token_set: TokenSet,
    pub field_names: HashSet<String>,
}
