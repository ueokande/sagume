/*
use std::collections::HashMap;

pub enum IndexOptions {
    None,
    Docs,
    DocsAndFreqs,
    DocsAndFreqsAndPositions,
    DocsAndFreqsAndPositionsAndOffsets,
}

pub enum DocValuesType {
    None,
    Numeric,
    Binary,
    Sorted,
    SortedNumeric,
    SortedSet,
}

pub struct FieldType {
    stored: bool,
    tokenized: bool,
    store_term_vectors: bool,
    store_term_vector_offsets: bool,
    store_term_vector_positions: bool,
    store_term_vector_payloads: bool,
    omit_norms: bool,
    index_options: IndexOptions,
    frozen: bool,
    doc_values_type: DocValuesType,
    data_dimension_count: i64,
    index_dimension_count: i64,
    dimension_num_bytes: i64,
    attributes: HashMap<String, String>,
}

impl FieldType {
    pub fn freeze(&mut self) {
        self.frozen = true;
    }

    pub fn stored(&self) -> bool {
        self.stored
    }

    pub fn tokenized(&self) -> bool {
        self.tokenized
    }

    pub fn store_term_vectors(&self) -> bool {
        self.store_term_vectors
    }

    pub fn store_term_vector_offsets(&self) -> bool {
        self.store_term_vector_offsets
    }

    pub fn store_term_vector_positions(&self) -> bool {
        self.store_term_vector_positions
    }

    pub fn store_term_vector_payloads(&self) -> bool {
        self.store_term_vector_positions
    }

    pub fn omit_norms(&self) -> bool {
        self.omit_norms
    }

    pub fn index_options(&self) -> IndexOptions {
        self.index_options
    }

    pub fn doc_values_type(&self) -> DocValuesType {
        self.doc_values_type
    }

    pub fn point_data_dimension_count(&self) -> i64 {
        return 0;
    }

    pub fn point_index_dimension_count(&self) -> i64 {
        return 0;
    }

    pub fn point_num_bytes(&self) -> i64 {
        return 0;
    }

    pub fn get_attributes(&self) -> HashMap<String, String> {
        self.attributes
    }
}
*/
