pub trait Directory {}

pub struct RAMDirectory {}

impl RAMDirectory {
    pub fn new() -> RAMDirectory {
        return RAMDirectory {};
    }
}

impl Directory for RAMDirectory {}
