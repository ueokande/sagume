use crate::token::Token;

type PipelineFunction = Fn(i32) -> i32;

pub struct Pipeline {
    registered: Vec<Box<PipelineFunction>>,
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            registered: Vec::new(),
        }
    }

    pub fn add<T>(&mut self, f: T)
    where
        T: Fn(i32) -> i32 + 'static,
    {
        self.registered.push(Box::new(f));
    }

    pub fn run(&self, tokens: Vec<Token>) -> Vec<Token> {
        tokens
    }
}
