use std::cell::RefCell;
use std::collections::HashMap;
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

    pub fn from_fuzzy_string(source: String, edit_distance: u64) -> TokenSet {
        struct Frame {
            node: Rc<RefCell<TokenSetNode>>,
            edits_remaining: u64,
            source: String,
        }
        let root = Rc::new(RefCell::new(TokenSetNode {
            last: false,
            edges: HashMap::new(),
        }));
        let mut stack: Vec<Frame> = vec![Frame {
            node: Rc::clone(&root),
            edits_remaining: edit_distance,
            source: source.to_string(),
        }];

        while stack.len() > 0 {
            let frame = stack.pop().unwrap();
            if frame.source.len() > 0 {
                let c = frame.source.chars().into_iter().next().unwrap();
                let no_edit_node: Rc<RefCell<TokenSetNode>>;
                if frame.node.borrow().edges.contains_key(&c) {
                    no_edit_node = Rc::clone(frame.node.borrow().edges.get(&c).unwrap());
                } else {
                    no_edit_node = Rc::new(RefCell::new(TokenSetNode {
                        last: false,
                        edges: HashMap::new(),
                    }));
                    frame
                        .node
                        .borrow_mut()
                        .edges
                        .insert(c, Rc::clone(&no_edit_node));
                }
                if frame.source.len() == 1 {
                    no_edit_node.borrow_mut().last = true;
                }
                stack.push(Frame {
                    node: no_edit_node,
                    edits_remaining: frame.edits_remaining,
                    source: frame.source[1..].to_string(),
                });
            }
            if frame.edits_remaining == 0 {
                continue;
            }
            let insertion_node = if frame.node.borrow().edges.contains_key(&'*') {
                Rc::clone(frame.node.borrow().edges.get(&'*').unwrap())
            } else {
                let n = Rc::new(RefCell::new(TokenSetNode {
                    edges: HashMap::new(),
                    last: false,
                }));
                frame.node.borrow_mut().edges.insert('*', Rc::clone(&n));
                n
            };
            if frame.source.len() == 0 {
                insertion_node.borrow_mut().last = true;
            }
            stack.push(Frame {
                node: insertion_node,
                edits_remaining: frame.edits_remaining - 1,
                source: frame.source.to_string(),
            });

            if frame.source.len() > 1 {
                stack.push(Frame {
                    node: Rc::clone(&frame.node),
                    edits_remaining: frame.edits_remaining - 1,
                    source: frame.source[1..].to_string(),
                });
            }
            if frame.source.len() == 1 {
                frame.node.borrow_mut().last = true;
            }

            if frame.source.len() >= 1 {
                let substitution_node = if frame.node.borrow().edges.contains_key(&'*') {
                    Rc::clone(frame.node.borrow().edges.get(&'*').unwrap())
                } else {
                    let n = Rc::new(RefCell::new(TokenSetNode {
                        last: false,
                        edges: HashMap::new(),
                    }));
                    frame.node.borrow_mut().edges.insert('*', Rc::clone(&n));
                    n
                };
                if frame.source.len() == 1 {
                    substitution_node.borrow_mut().last = true
                }
                stack.push(Frame {
                    node: substitution_node,
                    edits_remaining: frame.edits_remaining - 1,
                    source: frame.source[1..].to_string(),
                })
            }

            if frame.source.len() > 1 {
                let mut chars = frame.source.chars().into_iter();
                let c1 = chars.next().unwrap();
                let c2 = chars.next().unwrap();
                let transpose_node = if frame.node.borrow().edges.contains_key(&c2) {
                    Rc::clone(frame.node.borrow().edges.get(&c2).unwrap())
                } else {
                    let n = Rc::new(RefCell::new(TokenSetNode {
                        last: false,
                        edges: HashMap::new(),
                    }));
                    frame.node.borrow_mut().edges.insert(c2, Rc::clone(&n));
                    n
                };
                if frame.source.len() == 1 {
                    transpose_node.borrow_mut().last = true;
                }
                stack.push(Frame {
                    node: Rc::clone(&transpose_node),
                    edits_remaining: frame.edits_remaining - 1,
                    source: (c1.to_string() + &frame.source[2..]).to_string(),
                })
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

        let output = Rc::new(RefCell::new(TokenSetNode {
            last: false,
            edges: HashMap::new(),
        }));
        let mut stack: Vec<Frame> = vec![Frame {
            q_node: Rc::clone(&b.root),
            output: Rc::clone(&output),
            node: Rc::clone(&self.root),
        }];

        while stack.len() > 0 {
            let frame = stack.pop().unwrap();
            for q_key in frame.q_node.borrow().edges.keys() {
                for n_key in frame.node.borrow().edges.keys() {
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
