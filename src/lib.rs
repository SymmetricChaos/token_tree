mod tests;
mod error;

use crate::error::TransitionError;
use std::collections::BTreeMap;
use itertools::Itertools;

#[derive(Debug)]
pub enum Token<T> {
    Token(T),
    Error(TransitionError),
}

#[derive(Debug, Clone)]
pub struct Node<T: Copy> {
    pub output: Option<T>,
    pub transitions: Option<Vec<(char, Node<T>)>>,
}
 
impl<T: Copy> Node<T> {
    /// Construct a leaf Node. It must have a output and cannot have any transitions.
    pub fn leaf(c: char, output: T) -> (char, Node<T>) {
        (
            c,
            Node {
                output: Some(output),
                transitions: None,
            },
        )
    }
 
    /// Construct a branch Node. It must have transitions and may have an output.
    /// This constructor will panic at runtime if given an empty vector of transitions.
    pub fn branch(
        c: char,
        output: Option<T>,
        transitions: Vec<(char, Node<T>)>,
    ) -> (char, Node<T>) {
        assert!(!transitions.is_empty());
        (
            c,
            Node {
                output,
                transitions: Some(transitions),
            },
        )
    }
 
    /// Construct a sorted tree from a vector of transitions.
    pub fn tree(transitions: Vec<(char, Node<T>)>) -> Node<T> {
        let mut tree = Node {
            output: None,
            transitions: Some(transitions),
        };
        tree.sort();
        tree
    }
 
    /// Try to get the next token from a slice of chars.
    pub fn get<'a>(&self, chars: &'a [char], idx: usize) -> Result<(T, usize), TransitionError> {
        let mut i = 0;
        let mut curr_node = self;
        let mut maybe_char = None;
        for ch in chars.iter() {
            // find the transition to the next node or break if there is no
            // transition
            // a lack of transition could be a leaf node or a could mean that
            // the character has no transition from this node
            if let Some(trans_node) = curr_node.find_transition_node(*ch) {
                curr_node = trans_node;
            } else {
                maybe_char = Some(*ch);
                break;
            }
            i += 1;
        }
 
        // if an output exists then provide it and the index reached
        // otherwise the string being parsed is invalid
        if let Some(output) = curr_node.output {
            Ok((output, i))
        } else {
            Err(TransitionError{
                consumed_str: chars[0..i].iter().collect(), 
                character: maybe_char, 
                start: idx,
                end: idx+i
            })
        }
    }
 
    /// Transition to the next node if possible.
    pub fn find_transition_node(&self, char: char) -> Option<&Node<T>> {
        // If transitions exist find one that acts on 'char' and return it, if
        // there is no such node return none. At a leaf return none.
        if let Some(t) = &self.transitions {
            t.binary_search_by_key(&char, |t| t.0)
                .ok()
                .map(|index| &t[index].1)
        } else {
            None
        }
    }
 
    /// Extract tokens. If a substring is encountered that cannot produce a token
    /// the extraction is stopped and the error is reported.
    pub fn extract_tokens(&self, text: &str) -> Result<Vec<T>, TransitionError> {
        let chars = text.chars().collect::<Vec<_>>();
        let mut output = Vec::new();
        let len = chars.len();
        let mut curr_pos = 0;
 
        while curr_pos != len {
            let result = self.get(&chars[curr_pos..],curr_pos)?;
            output.push(result.0.to_owned());
            curr_pos += result.1;
        }
        Ok(output)
    }

    /// Extract tokens. If a substring is encountered that cannot produce a token
    /// an error is inserted into the output instead and the process continues.
    pub fn extract_tokens_infallible(&self, text: &str) -> Vec<Token<T>> {
        let chars = text.chars().collect::<Vec<_>>();
        let mut output = Vec::new();
        let len = chars.len();
        let mut curr_pos = 0;
 
        while curr_pos < len {
            match self.get(&chars[curr_pos..],curr_pos) {
                Ok(result) => {
                    curr_pos += result.1;
                    output.push(Token::Token(result.0.to_owned()));
                },
                Err(error) => {
                    curr_pos += std::cmp::max(error.end,1);
                    output.push(Token::Error(error));
                },
            }
        }
        output
    }
 
    /// Sorts the tree by the transition characters. Called automatically by the Node::tree() constructor.
    /// An unsorted tree will not transition properly
    pub fn sort(&mut self) {
        if let Some(transitions) = &mut self.transitions {
            transitions.sort_by_key(|el| el.0);
            for el in transitions {
                el.1.sort();
            }
        }
    }
 
    /// Counts the number of paths through the tree that result in an output
    pub fn num_output_paths(&self) -> usize {
        match &self.transitions {
            Some(v) => {
                let mut sum = match self.output {
                    Some(_) => 1,
                    None => 0,
                };
                for (_, n) in v {
                    sum += n.num_output_paths()
                }
                sum
            }
            None => 1,
        }
    }
 
    /// Detect if any paths result in the same string
    pub fn validate(&self) -> Result<(), Vec<(String, Vec<T>)>> {
        let mut paths: Vec<(String, T)> = Vec::new();
        self.input_paths_inner(vec![], &mut paths);
        let mut map: BTreeMap<String, Vec<T>> = BTreeMap::new();
        for (k, v) in paths {
            map.entry(k).and_modify(|vec| vec.push(v));
        }
        let mut out = Vec::new();
        for (k, v) in map.into_iter() {
            if !v.is_empty() {
                out.push((k, v))
            }
        }
        if !out.is_empty() {
            Err(out)
        } else {
            Ok(())
        }
    }
 
    /// Documents every valid string that and the token T that it translates to. Results are sorted by tree order.
    pub fn input_paths(&self) -> Vec<(String, T)> {
        let mut paths: Vec<(String, T)> = Vec::new();
        self.input_paths_inner(vec![], &mut paths);
        paths
    }
 
    fn input_paths_inner(&self, chars: Vec<char>, paths: &mut Vec<(String, T)>) {
        if let Some(s) = self.output {
            paths.push((chars.iter().collect::<String>(), s))
        }
        if let Some(transitions) = &self.transitions {
            for (c, n) in transitions.iter() {
                let mut new_chars = chars.clone();
                new_chars.push(*c);
                n.input_paths_inner(new_chars, paths)
            }
        }
    }
}
 
impl<T: Copy + Ord> Node<T> {
    /// Documents every token T that can be produced and all the strings that produce it
    pub fn output_paths(&self) -> Vec<(T, Vec<String>)> {
        let mut map = BTreeMap::new();
        self.output_paths_inner(vec![], &mut map);
        let mut paths = map.iter().map(|(k, v)| (*k, v.clone())).collect_vec();
        paths.sort_by_key(|a| a.0);
        paths
    }
 
    fn output_paths_inner(&self, chars: Vec<char>, paths: &mut BTreeMap<T, Vec<String>>) {
        if let Some(s) = self.output {
            let input = chars.iter().collect::<String>();
            match paths.contains_key(&s) {
                true => {
                    paths.entry(s).and_modify(|e| e.push(input));
                }
                false => {
                    paths.insert(s, vec![input]);
                }
            };
        }
        if let Some(transitions) = &self.transitions {
            for (c, n) in transitions.iter() {
                let mut new_chars = chars.clone();
                new_chars.push(*c);
                n.output_paths_inner(new_chars, paths)
            }
        }
    }
}