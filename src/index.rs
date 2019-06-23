use crate::field::FieldRef;
use crate::query::{Presence, Query};
use crate::token::TokenSet;
use crate::vector::Vector;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Clone)]
pub struct InvertedIndex {
    pub index: u64,
    pub documents: HashMap<String, HashSet<String>>, // field_name -> []document_ref
}

pub struct Index {
    inverted_index: HashMap<String, InvertedIndex>,
    field_vectors: HashMap<FieldRef, Vector>,
    token_set: TokenSet,
    field_names: HashSet<String>,

    complete_doc_refs: HashSet<String>,
}

impl Index {
    pub fn new(
        inverted_index: HashMap<String, InvertedIndex>,
        field_vectors: HashMap<FieldRef, Vector>,
        token_set: TokenSet,
        field_names: HashSet<String>,
    ) -> Index {
        let mut complete_doc_refs: HashSet<String> = HashSet::new();
        for ri in inverted_index.values() {
            for doc_refs in ri.documents.values() {
                for doc_ref in doc_refs {
                    complete_doc_refs.insert(doc_ref.to_string());
                }
            }
        }
        return Index {
            inverted_index,
            field_vectors,
            token_set,
            field_names,
            complete_doc_refs,
        };
    }

    pub fn inverted_index(&self) -> &HashMap<String, InvertedIndex> {
        &self.inverted_index
    }

    pub fn field_vectors(&self) -> &HashMap<FieldRef, Vector> {
        &self.field_vectors
    }

    pub fn token_set(&self) -> &TokenSet {
        &self.token_set
    }

    pub fn query(&self, query: &Query) -> Vec<MatchResult> {
        let mut query_vectors: HashMap<String, Vector> = HashMap::new();
        for field_ref in self.field_vectors.keys() {
            query_vectors.insert(field_ref.field_name().to_string(), Vector::new());
        }

        let mut required_matches: HashMap<String, HashSet<String>> = HashMap::new();
        let mut prohibited_matches: HashMap<String, HashSet<String>> = HashMap::new();
        let mut matching_fields: HashMap<FieldRef, MatchData> = HashMap::new();

        for clause in &query.clauses {
            let mut clause_matches = self.complete_doc_refs.clone();
            let term_token_set = TokenSet::from_clause(&clause);
            let expanded_terms = term_token_set.intersect(&term_token_set);

            if expanded_terms.to_vec().is_empty() && clause.option.presence == Presence::Required {
                for field in &clause.option.fields {
                    required_matches.insert(field.to_string(), HashSet::new());
                }
                break;
            }

            for expanded_term in expanded_terms.to_vec() {
                let ri = self.inverted_index.get(&expanded_term).unwrap();
                for field in &clause.option.fields {
                    let matching_docs = ri.documents.get(field.into()).unwrap();
                    if clause.option.presence == Presence::Required {
                        for doc in matching_docs {
                            clause_matches.insert(doc.to_string());
                        }
                        if !required_matches.contains_key(&field.to_string()) {
                            required_matches
                                .insert(field.to_string(), self.complete_doc_refs.clone());
                        }
                    }

                    if clause.option.presence == Presence::Prohibited {
                        if !prohibited_matches.contains_key(&field.to_string()) {
                            prohibited_matches.insert(field.to_string(), HashSet::new());
                        }
                        for doc in matching_docs {
                            prohibited_matches
                                .get_mut(&field.to_string())
                                .unwrap()
                                .insert(doc.to_string());
                        }
                        continue;
                    }

                    let boost = query_vectors
                        .get(&field.to_string())
                        .unwrap()
                        .get(ri.index as usize)
                        .unwrap_or(0.0);
                    query_vectors
                        .get_mut(&field.to_string())
                        .unwrap()
                        .upsert(ri.index as usize, boost + clause.option.boost as f64);

                    for doc_ref in matching_docs {
                        let field_ref = FieldRef::new(doc_ref.to_string(), field.to_string());
                        if let Some(matching) = matching_fields.get_mut(&field_ref) {
                            matching.add(expanded_term.to_string(), field.to_string())
                        } else {
                            let mut match_data = MatchData::new();
                            match_data.add(expanded_term.to_string(), field.to_string());
                            matching_fields.insert(field_ref, match_data);
                        }
                    }
                }
            }

            if clause.option.presence == Presence::Required {
                for field in &clause.option.fields {
                    let old = required_matches.remove(field).unwrap();
                    required_matches.insert(
                        field.to_string(),
                        old.intersection(&clause_matches)
                            .map(|f| f.to_string())
                            .collect(),
                    );
                }
            }
        }

        let mut all_required_matches = self.complete_doc_refs.clone();
        let mut all_prohibited_matches = HashSet::new();
        for field in &self.field_names {
            if required_matches.contains_key(field) {
                let set = required_matches.get(field).unwrap();
                all_required_matches.retain(|doc_ref| set.contains(doc_ref));
            }
            if prohibited_matches.contains_key(field) {
                for doc_ref in prohibited_matches.get(field).unwrap() {
                    all_prohibited_matches.insert(doc_ref);
                }
            }
        }

        let matching_field_refs: HashSet<&FieldRef> = if query.is_negated() {
            for field_ref in self.field_vectors.keys() {
                matching_fields.insert(field_ref.clone(), MatchData::new());
            }
            self.field_vectors.keys().collect()
        } else {
            matching_fields.keys().collect()
        };

        let mut doc_matches: HashMap<String, MatchResult> = HashMap::new();
        for field_ref in matching_field_refs {
            let doc_ref = field_ref.doc_ref();
            if !all_required_matches.contains(&doc_ref.to_string()) {
                continue;
            }
            if all_prohibited_matches.contains(&doc_ref.to_string()) {
                continue;
            }

            let field_vector = self.field_vectors.get(&field_ref).unwrap();
            let score = query_vectors
                .get(field_ref.field_name())
                .unwrap()
                .similarity(field_vector);
            if let Some(mut m) = doc_matches.remove(&doc_ref.to_string()) {
                m.score += score;
                doc_matches.insert(doc_ref.to_string(), m);
            } else {
                let m = MatchResult {
                    doc_ref: doc_ref.to_string(),
                    score,
                };
                doc_matches.insert(doc_ref.to_string(), m);
            }
        }

        let mut results: Vec<MatchResult> = doc_matches.values().map(|m| m.clone()).collect();
        results.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(Ordering::Less));
        return results;
    }
}

pub struct MatchData {
    metadata: HashMap<String, HashSet<String>>,
}

impl MatchData {
    pub fn new() -> MatchData {
        MatchData {
            metadata: HashMap::new(),
        }
    }

    pub fn combine(&mut self, other: &MatchData) {
        for (term, fields) in &other.metadata {
            for field in fields {
                self.add(term.clone(), field.clone());
            }
        }
    }

    pub fn add(&mut self, term: String, field: String) {
        if let Some(fields) = self.metadata.get_mut(&term) {
            fields.insert(field);
        } else {
            let mut set = HashSet::new();
            set.insert(field);
            self.metadata.insert(term, set);
        }
    }
}

#[derive(Clone)]
pub struct MatchResult {
    doc_ref: String,
    score: f64,
}
