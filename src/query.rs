#[derive(Eq, PartialEq, Clone)]
pub enum WildcardMode {
    None,
    Leading,
    Traling,
}

#[derive(Eq, PartialEq, Clone)]
pub enum Presence {
    Optional,
    Required,
    Prohibited,
}

#[derive(Eq, PartialEq, Clone)]
pub struct Clause {
    pub term: String,
    pub fields: Option<Vec<String>>,
    pub boost: u64,
    pub use_pipeline: bool,
    pub wildcard: WildcardMode,
    pub presence: Presence,
}

impl Clause {
    pub fn new(term: String) -> Clause {
        Clause {
            term,
            fields: None,
            boost: 1,
            use_pipeline: true,
            wildcard: WildcardMode::None,
            presence: Presence::Optional,
        }
    }

    pub fn term(&self) -> &str {
        &self.term
    }

    pub fn fields(&self) -> &Option<Vec<String>> {
        &self.fields
    }

    pub fn boost(&self) -> u64 {
        self.boost
    }

    pub fn use_pipeline(&self) -> bool {
        self.use_pipeline
    }

    pub fn wildcard(&self) -> WildcardMode {
        self.wildcard.clone()
    }

    pub fn presence(&self) -> Presence {
        self.presence.clone()
    }

    pub fn set_fields(&mut self, fields: Vec<String>) {
        self.fields = Some(fields);
    }

    pub fn set_boost(&mut self, boost: u64) {
        self.boost = boost;
    }

    pub fn set_use_pipeline(&mut self, use_pipeline: bool) {
        self.use_pipeline = use_pipeline;
    }

    pub fn set_wildcard(&mut self, wildcard: WildcardMode) {
        self.wildcard = wildcard;
    }

    pub fn set_presence(&mut self, presence: Presence) {
        self.presence = presence;
    }
}

pub struct Query {
    pub clauses: Vec<Clause>,
}

impl Query {
    pub fn new() -> Query {
        Query {
            clauses: Vec::new(),
        }
    }

    pub fn add_clause(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }

    pub fn is_negated(&self) -> bool {
        self.clauses
            .iter()
            .all(|c| c.presence == Presence::Prohibited)
    }
}
