use sagume::analysis::StandardAnalyzer;
use sagume::document::Document;
use sagume::field::Field;
use sagume::index::{IndexWriter, IndexWriterConfig};
use sagume::store::RAMDirectory;

fn main() {
    let analyzer = StandardAnalyzer::new();
    let index = RAMDirectory::new();

    let config = IndexWriterConfig::new(analyzer);

    let w = IndexWriter::new(index, config);

    let mut doc = Document::new();
    doc.add(Field::new_text("title".into(), "Lucene in Action".into()));
    doc.add(Field::new_u64("isbn".into(), 193398817));
    w.add_document(doc);
    w.close();
}
