use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::query::Clause;

#[derive(Eq, PartialEq, Clone)]
pub struct Token {
    pub index: usize,
    pub start: usize,
    pub value: String,
}

impl Token {
    pub fn value(&self) -> &str {
        &self.value
    }
}

pub struct TokenSet {
    root: Rc<RefCell<TokenSetNode>>,
}

// id is a unique id for a TokenSetNode
static mut GLOBAL_ID: usize = 0;

#[derive(Debug)]
pub struct TokenSetNode {
    last: bool,
    edges: HashMap<char, Rc<RefCell<TokenSetNode>>>,
    id: usize,
}

impl TokenSet {
    pub fn from_array(tokens: &Vec<String>) -> TokenSet {
        let mut builder = TokenSetBuilder::new();
        for token in tokens {
            builder.insert(&token);
        }
        builder.finish();
        TokenSet { root: builder.root }
    }

    pub fn from_string(source: &str) -> TokenSet {
        let mut root = Rc::new(RefCell::new(TokenSetNode::new()));
        let mut node = Rc::clone(&mut root);

        for (i, c) in source.chars().enumerate() {
            let last = i == source.len() - 1;
            if c == '*' {
                let mut r = node.borrow_mut();
                r.edges.insert(c, Rc::clone(&node));
                r.last = last;
            } else {
                let mut n = TokenSetNode::new();
                n.last = last;
                let next = Rc::new(RefCell::new(n));
                node.borrow_mut().edges.insert(c, Rc::clone(&next));
                node = next
            }
        }
        TokenSet { root }
    }

    pub fn from_fuzzy_string(source: &str, edit_distance: u64) -> TokenSet {
        struct Frame {
            node: Rc<RefCell<TokenSetNode>>,
            edits_remaining: u64,
            source: String,
        }
        let root = Rc::new(RefCell::new(TokenSetNode::new()));
        let mut stack: Vec<Frame> = vec![Frame {
            node: Rc::clone(&root),
            edits_remaining: edit_distance,
            source: source.to_string(),
        }];

        while let Some(frame) = stack.pop() {
            if let Some(c) = frame.source.chars().into_iter().next() {
                let no_edit_node: Rc<RefCell<TokenSetNode>>;
                if frame.node.borrow().edges.contains_key(&c) {
                    no_edit_node = Rc::clone(frame.node.borrow().edges.get(&c).unwrap());
                } else {
                    no_edit_node = Rc::new(RefCell::new(TokenSetNode::new()));
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
                let n = Rc::new(RefCell::new(TokenSetNode::new()));
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
                    let n = Rc::new(RefCell::new(TokenSetNode::new()));
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
                    let n = Rc::new(RefCell::new(TokenSetNode::new()));
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

    pub fn from_clause(clause: &Clause) -> TokenSet {
        TokenSet::from_string(&clause.term)
    }

    pub fn intersect(&self, b: &Self) -> TokenSet {
        struct Frame {
            q_node: Rc<RefCell<TokenSetNode>>,
            output: Rc<RefCell<TokenSetNode>>,
            node: Rc<RefCell<TokenSetNode>>,
        }

        let output = Rc::new(RefCell::new(TokenSetNode::new()));
        let mut stack: Vec<Frame> = vec![Frame {
            q_node: Rc::clone(&b.root),
            output: Rc::clone(&output),
            node: Rc::clone(&self.root),
        }];

        while let Some(frame) = stack.pop() {
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
                        let mut n = TokenSetNode::new();
                        n.last = last;
                        let next = Rc::new(RefCell::new(n));
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

        while let Some(frame) = stack.pop() {
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

impl TokenSetNode {
    pub fn new() -> TokenSetNode {
        unsafe {
            GLOBAL_ID += 1;
            TokenSetNode {
                edges: HashMap::new(),
                last: false,
                id: GLOBAL_ID,
            }
        }
    }

    fn to_str(&self) -> String {
        let mut id = if self.last { "1".into() } else { "0".into() };
        for (c, child) in self.edges.iter() {
            id = format!("{}{}{}", id, c, child.borrow().id);
        }
        id
    }
}

#[derive(Debug)]
struct UncheckedNodes {
    parent: Rc<RefCell<TokenSetNode>>,
    c: char,
    child: Rc<RefCell<TokenSetNode>>,
}

struct TokenSetBuilder {
    prev_word: String,
    root: Rc<RefCell<TokenSetNode>>,
    unchecked_nodes: Vec<UncheckedNodes>,
    minimized_nodes: HashMap<String, Rc<RefCell<TokenSetNode>>>,
}

impl TokenSetBuilder {
    fn new() -> TokenSetBuilder {
        let root = Rc::new(RefCell::new(TokenSetNode::new()));
        TokenSetBuilder {
            prev_word: "".into(),
            root: root,
            unchecked_nodes: Vec::new(),
            minimized_nodes: HashMap::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        if word < &self.prev_word {
            panic!("Out of order word insertion")
        }

        let mut common_prefix = 0;
        let mut r1 = word.chars().into_iter();
        let mut r2 = self.prev_word.chars().into_iter();
        loop {
            let n1 = r1.next();
            let n2 = r2.next();
            if n1.is_none() || n2.is_none() || n1.unwrap() != n2.unwrap() {
                break;
            }
            common_prefix += 1;
        }
        self.minimize(common_prefix);

        let mut node = if self.unchecked_nodes.is_empty() {
            Rc::clone(&self.root)
        } else {
            Rc::clone(&self.unchecked_nodes.last().unwrap().child)
        };

        for c in word[common_prefix..].chars() {
            let next_node = Rc::new(RefCell::new(TokenSetNode::new()));
            node.borrow_mut().edges.insert(c, Rc::clone(&next_node));

            self.unchecked_nodes.push(UncheckedNodes {
                parent: node,
                c,
                child: Rc::clone(&next_node),
            });
            node = next_node;
        }

        node.borrow_mut().last = true;
        self.prev_word = word.to_string();
    }

    fn finish(&mut self) {
        self.minimize(0);
    }

    fn minimize(&mut self, down_to: usize) {
        for i in (down_to..self.unchecked_nodes.len()).rev() {
            let node = &self.unchecked_nodes.get(i).unwrap();
            let child_id = node.child.borrow().to_str();
            if self.minimized_nodes.contains_key(&child_id) {
                node.parent.borrow_mut().edges.insert(
                    node.c,
                    Rc::clone(self.minimized_nodes.get(&child_id).unwrap()),
                );
            } else {
                self.minimized_nodes
                    .insert(child_id, Rc::clone(&node.child));
            }
            self.unchecked_nodes.pop();
        }
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
    assert_eq!(&wild.borrow().id, &wild2.borrow().id);
}

#[test]
fn test_to_str() {
    let non_last = TokenSetNode::new();
    let mut last = TokenSetNode::new();
    let mut other_last = TokenSetNode::new();
    last.last = true;
    other_last.last = true;

    assert_ne!(non_last.to_str(), last.to_str());
    assert_eq!(last.to_str(), other_last.to_str());

    let zero_edges = TokenSetNode::new();
    let mut one_edge = TokenSetNode::new();
    let mut two_edges = TokenSetNode::new();
    one_edge.edges.insert(
        'a',
        Rc::new(RefCell::new(TokenSetNode {
            edges: HashMap::new(),
            id: 99999,
            last: false,
        })),
    );
    two_edges.edges.insert(
        'a',
        Rc::new(RefCell::new(TokenSetNode {
            edges: HashMap::new(),
            id: 99999,
            last: false,
        })),
    );
    two_edges.edges.insert(
        'b',
        Rc::new(RefCell::new(TokenSetNode {
            edges: HashMap::new(),
            id: 99999,
            last: false,
        })),
    );

    assert_ne!(zero_edges.to_str(), one_edge.to_str());
    assert_ne!(two_edges.to_str(), one_edge.to_str());
    assert_ne!(two_edges.to_str(), zero_edges.to_str());

    let child_a = Rc::new(RefCell::new(TokenSetNode::new()));
    let child_b = Rc::new(RefCell::new(TokenSetNode::new()));
    let parent_a = Rc::new(RefCell::new(TokenSetNode::new()));
    let parent_b = Rc::new(RefCell::new(TokenSetNode::new()));
    let parent_c = Rc::new(RefCell::new(TokenSetNode::new()));

    parent_a.borrow_mut().edges.insert('a', Rc::clone(&child_a));
    parent_b.borrow_mut().edges.insert('a', Rc::clone(&child_b));
    parent_c.borrow_mut().edges.insert('a', Rc::clone(&child_b));

    assert_eq!(parent_b.borrow().to_str(), parent_c.borrow().to_str());
    assert_ne!(parent_a.borrow().to_str(), parent_c.borrow().to_str());
    assert_ne!(parent_a.borrow().to_str(), parent_b.borrow().to_str());
}

#[test]
fn test_from_array() {
    let s = TokenSet::from_array(&vec!["ac".into(), "dc".into()]);
    let ac_node = s
        .root
        .borrow()
        .edges
        .get(&'a')
        .unwrap()
        .borrow()
        .edges
        .get(&'c')
        .unwrap()
        .borrow()
        .id;
    let dc_node = s
        .root
        .borrow()
        .edges
        .get(&'d')
        .unwrap()
        .borrow()
        .edges
        .get(&'c')
        .unwrap()
        .borrow()
        .id;
    assert_eq!(ac_node, dc_node);
}
