use sagume::analysis::StandardAnalyzer;
use sagume::document::*;
use sagume::index::{IndexWriter, IndexWriterConfig};
use sagume::store::RAMDirectory;

fn main() {
    let analyzer = StandardAnalyzer::new();
    let index = RAMDirectory::new();

    let config = IndexWriterConfig::new(analyzer);

    let w = IndexWriter::new(index, config);

    let doc = Document::new();
    doc.add(TextField::new(
        "title".to_string(),
        "Lucene in Action".to_string(),
    ));
    doc.add(StringField::new(
        "isbn".to_string(),
        "193398817".to_string(),
    ));
    w.add_document(doc);
    w.close();
}
