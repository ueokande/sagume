pub trait Analyzer {}

pub struct StandardAnalyzer {}

impl StandardAnalyzer {
    pub fn new() -> StandardAnalyzer {
        return StandardAnalyzer {};
    }
}

impl Analyzer for StandardAnalyzer {}
