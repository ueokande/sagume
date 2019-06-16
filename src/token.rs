use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Eq, PartialEq, Clone)]
pub struct Token {
    pub index: usize,
    pub start: usize,
    pub value: String,
}

pub struct TokenSet {
    root: Rc<RefCell<TokenSetNode>>,
}

pub struct TokenSetNode {
    last: bool,
    edges: HashMap<char, Rc<RefCell<TokenSetNode>>>,
}

impl TokenSet {
    pub fn from_array(tokens: Vec<String>) -> TokenSet {
        let mut builder = TokenSetBuilder {};
        for token in tokens {
            builder.insert(token);
        }
        builder.build()
    }

    pub fn from_fuzzy_string(source: String, edit_distance: bool) -> TokenSet {
        panic!("Not implemented!!");
    }

    pub fn from_string(source: String) -> TokenSet {
        let mut root = Rc::new(RefCell::new(TokenSetNode {
            last: false,
            edges: HashMap::new(),
        }));
        let mut node = Rc::clone(&mut root);

        for (i, c) in source.chars().enumerate() {
            let last = i == source.len() - 1;
            if c == '*' {
                let mut r = node.borrow_mut();
                r.edges.insert(c, Rc::clone(&node));
                r.last = last;
            } else {
                let next = Rc::new(RefCell::new(TokenSetNode {
                    last: last,
                    edges: HashMap::new(),
                }));

                node.borrow_mut().edges.insert(c, Rc::clone(&next));
                node = next
            }
        }
        TokenSet { root }
    }

    pub fn intersect(&self, b: &Self) -> TokenSet {
        struct Frame {
            q_node: Rc<RefCell<TokenSetNode>>,
            output: Rc<RefCell<TokenSetNode>>,
            node: Rc<RefCell<TokenSetNode>>,
        }

        let mut output = Rc::new(RefCell::new(TokenSetNode {
            last: false,
            edges: HashMap::new(),
        }));
        let mut stack: Vec<Frame> = vec![Frame {
            q_node: Rc::clone(&b.root),
            output: Rc::clone(&output),
            node: Rc::clone(&self.root),
        }];

        while (stack.len() > 0) {
            let frame = stack.pop().unwrap();
            for (q_key, q_edge) in frame.q_node.borrow().edges.iter() {
                for (n_key, n_edge) in frame.node.borrow().edges.iter() {
                    if *n_key != *q_key && *q_key != '*' {
                        continue;
                    }
                    let node = Rc::clone(&frame.node.borrow().edges.get(&n_key).unwrap());
                    let q_node = Rc::clone(&frame.q_node.borrow().edges.get(&q_key).unwrap());
                    let last = node.borrow().last && q_node.borrow().last;
                    let next = if frame.output.borrow().edges.contains_key(&n_key) {
                        let next = Rc::clone(&frame.output.borrow().edges.get(&n_key).unwrap());
                        let mut next_mut = next.borrow_mut();
                        next_mut.last = next_mut.last || last;
                        Rc::clone(&next)
                    } else {
                        let next = Rc::new(RefCell::new(TokenSetNode {
                            last: last,
                            edges: HashMap::new(),
                        }));
                        frame
                            .output
                            .borrow_mut()
                            .edges
                            .insert(*n_key, Rc::clone(&next));
                        next
                    };
                    stack.push(Frame {
                        q_node: Rc::clone(&q_node),
                        output: next,
                        node: Rc::clone(&node),
                    });
                }
            }
        }
        TokenSet { root: output }
    }

    pub fn to_vec(&self) -> Vec<String> {
        struct Frame {
            prefix: String,
            node: Rc<RefCell<TokenSetNode>>,
        }
        let mut words: Vec<String> = Vec::new();
        let mut stack: Vec<Frame> = vec![Frame {
            prefix: "".into(),
            node: Rc::clone(&self.root),
        }];
        while stack.len() > 0 {
            let frame = stack.pop().unwrap();
            let node = frame.node.borrow();
            if node.last {
                words.push(frame.prefix.to_string())
            }

            let edge_keys = node.edges.keys();
            for edge_key in edge_keys {
                let mut prefix = String::from(frame.prefix.to_string());
                prefix.push(*edge_key);
                stack.push(Frame {
                    prefix,
                    node: Rc::clone(frame.node.borrow().edges.get(edge_key).unwrap()),
                })
            }
        }
        words
    }
}

struct TokenSetBuilder;

impl TokenSetBuilder {
    fn insert(&mut self, word: String) {}

    fn build(&self) -> TokenSet {
        panic!("Not implemented!")
    }
}

#[test]
fn test_from_string() {
    let root = TokenSet::from_string("a".into()).root;
    let a = Rc::clone(root.borrow().edges.get(&'a').unwrap());
    assert!(a.borrow().last);

    let root = TokenSet::from_string("a*".into()).root;
    let a = Rc::clone(root.borrow().edges.get(&'a').unwrap());
    assert!(a.borrow().last);
    let wild = Rc::clone(a.borrow().edges.get(&'*').unwrap());
    assert!(wild.borrow().last);
    let wild2 = Rc::clone(wild.borrow().edges.get(&'*').unwrap());
    assert!(Rc::ptr_eq(&wild, &wild2));
}
