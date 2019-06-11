use crate::analysis::*;
use crate::document::Document;
use crate::store::Directory;

pub struct IndexWriterConfig {}

impl IndexWriterConfig {
    pub fn new<T>(analyzer: T) -> IndexWriterConfig
    where
        T: Analyzer,
    {
        return IndexWriterConfig {};
    }
}

pub struct IndexWriter {}

impl IndexWriter {
    pub fn new<T>(directory: T, config: IndexWriterConfig) -> IndexWriter
    where
        T: Directory,
    {
        return IndexWriter {};
    }

    pub fn add_document(&self, doc: Document) {
        panic!("Not implemented")
    }

    pub fn close(&self) {}
}
