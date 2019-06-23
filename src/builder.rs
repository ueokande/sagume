use crate::document::Document;
use crate::field::FieldRef;
use crate::index::{Index, InvertedIndex};
use crate::pipeline::Pipeline;
use crate::token::TokenSet;
use crate::tokenizer::Tokenizer;
use crate::vector::Vector;
use std::collections::{HashMap, HashSet};

pub struct Builder {
    field_names: HashSet<String>,
    inverted_index: HashMap<String, InvertedIndex>,
    field_term_frequencies: HashMap<FieldRef, HashMap<String, usize>>,
    field_lengths: HashMap<FieldRef, usize>,
    tokenizer: Tokenizer,
    pipeline: Pipeline,
    b: f64,
    k1: f64,
    term_index: u64,
    document_count: usize,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            field_names: HashSet::new(),
            inverted_index: HashMap::new(),
            field_term_frequencies: HashMap::new(),
            field_lengths: HashMap::new(),
            tokenizer: Tokenizer::new(),
            pipeline: Pipeline::new(),
            b: 0.75,
            k1: 1.2,
            term_index: 0,
            document_count: 0,
        }
    }

    pub fn add_field(&mut self, name: String) {
        self.field_names.insert(name);
    }

    pub fn add_document(&mut self, doc: Document) {
        let doc_ref = doc.doc_ref();

        self.document_count += 1;

        for field in doc.get_all_fields().iter() {
            let field_value = field.value().to_string();
            let tokens = self.tokenizer.tokenize(field_value);
            let terms = self.pipeline.run(tokens);
            let field_ref = FieldRef::new(doc_ref.into(), field.name().into());

            self.field_lengths.insert(
                field_ref.clone(),
                self.field_lengths.get(&field_ref).unwrap_or(&0) + terms.len(),
            );

            let mut field_terms: HashMap<String, usize> = HashMap::new();
            for term in terms.iter() {
                field_terms.insert(
                    term.value().to_string(),
                    field_terms.get(term.value()).unwrap_or(&0) + 1,
                );

                if !self.inverted_index.contains_key(term.value()) {
                    self.term_index += 1;
                }
                let mut ridx = self
                    .inverted_index
                    .remove(term.value())
                    .unwrap_or(InvertedIndex {
                        index: self.term_index,
                        documents: HashMap::new(),
                    });
                let mut doc_set = ridx
                    .documents
                    .remove(field.name())
                    .unwrap_or(HashSet::new());
                doc_set.insert(doc_ref.to_string());
                ridx.documents.insert(field.name().to_string(), doc_set);
                self.inverted_index.insert(term.value().to_string(), ridx);
            }
            self.field_term_frequencies.insert(field_ref, field_terms);
        }
    }

    pub fn b(&mut self, value: f64) {
        self.b = value.max(0.0).min(1.0);
    }

    pub fn k1(&mut self, value: f64) {
        self.k1 = value;
    }

    pub fn build(&mut self) -> Index {
        Index::new(
            self.inverted_index.clone(),
            self.create_field_vectors(),
            self.create_token_set(),
            self.field_names.clone(),
        )
    }

    fn calculate_average_field_length(&self) -> HashMap<String, f64> {
        let mut accumulator: HashMap<String, f64> = HashMap::new();
        let mut docs_with_field: HashMap<String, usize> = HashMap::new();
        for (field_ref, len) in self.field_lengths.iter() {
            let field_name = field_ref.field_name().to_string();

            let count = docs_with_field.get(&field_name).unwrap_or(&0) + 1;
            docs_with_field.insert(field_name.to_string(), count);

            let count = *accumulator.get(&field_name).unwrap_or(&0.0)
                + *self.field_lengths.get(&field_ref).unwrap() as f64;
            accumulator.insert(field_name, count);
        }

        for (field_name, val) in accumulator.iter_mut() {
            *val = *val / *docs_with_field.get(field_name).unwrap() as f64;
        }
        return accumulator;
    }

    fn create_field_vectors(&self) -> HashMap<FieldRef, Vector> {
        let mut field_vectors: HashMap<FieldRef, Vector> = HashMap::new();
        let average_field_length = self.calculate_average_field_length();

        for field_ref in self.field_term_frequencies.keys() {
            let field_name = field_ref.field_name();
            let field_length = *self.field_lengths.get(field_ref).unwrap();
            let term_frequencies = self.field_term_frequencies.get(field_ref).unwrap();
            let terms = term_frequencies.keys();

            let mut field_vector = Vector::new();
            for term in terms {
                let term_freq = *term_frequencies.get(term).unwrap();
                let term_index = self.inverted_index.get(term).unwrap().index;

                let idf =
                    Builder::idf(&self.inverted_index.get(term).unwrap(), self.document_count);

                let k1 = self.k1;
                let b = self.b;
                let score = idf * ((k1 + 1.0) * term_freq as f64)
                    / (k1
                        * (1.0 - b
                            + b * (field_length as f64
                                / average_field_length.get(field_name).unwrap()))
                        + term_freq as f64);

                // TODO need to reduce the precision?
                let score_wth_precision = (score * 1000.0).round() / 1000.0;
                field_vector.insert(term_index as usize, score_wth_precision);
            }
            field_vectors.insert(field_ref.clone(), field_vector);
        }
        return field_vectors;
    }

    fn create_token_set(&self) -> TokenSet {
        let mut tokens: Vec<String> = self
            .inverted_index
            .keys()
            .map(|token| token.to_string())
            .collect();
        tokens.sort();
        TokenSet::from_array(tokens)
    }

    fn idf(idx: &InvertedIndex, doc_count: usize) -> f64 {
        let mut documents_with_term = 0;
        for doc_refs in idx.documents.values() {
            documents_with_term += doc_refs.len()
        }
        let x =
            ((doc_count - documents_with_term) as f64 + 0.5) / (documents_with_term as f64 + 0.5);
        return (x.abs() + 1.0).log(std::f64::consts::E);
    }
}

#[cfg(test)]
use crate::field::Field;

#[test]
fn test_add() {
    let mut doc = Document::new("1".into());
    doc.add_field(Field::new_text("title".into(), "constructor".into()));
    doc.add_field(Field::new_text("memo".into(), "good".into()));

    let mut b = Builder::new();
    b.add_field("title".into());
    b.add_document(doc);

    assert!(b
        .inverted_index
        .get("constructor")
        .unwrap()
        .documents
        .get("title")
        .unwrap()
        .contains("1"));
    assert!(!b.inverted_index.contains_key("missing".into()));
    assert_eq!(
        *b.field_term_frequencies
            .get(&FieldRef::new("1".into(), "title".into()))
            .unwrap()
            .get("constructor".into())
            .unwrap(),
        1
    );
    assert_eq!(b.document_count, 1);
}

#[test]
fn test_define_field() {
    let mut b = Builder::new();
    b.add_field("title".into());
    b.add_field("isbn".into());
    b.add_field("author/age".into());
    assert!(b.field_names.contains("title".into()));
    assert!(b.field_names.contains("isbn".into()));
    assert!(b.field_names.contains("author/age".into()));
}

#[test]
fn test_b() {
    let mut b = Builder::new();
    assert_eq!(b.b, 0.75);

    b.b(0.5);
    assert_eq!(b.b, 0.5);

    b.b(-1.0);
    assert_eq!(b.b, 0.0);

    b.b(1.5);
    assert_eq!(b.b, 1.0);
}

#[test]
fn test_k1() {
    let mut b = Builder::new();
    assert_eq!(b.k1, 1.2);

    b.k1(0.5);
    assert_eq!(b.k1, 0.5);
}
