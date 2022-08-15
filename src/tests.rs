#[cfg(test)]
mod test {
    use crate::Node;
    use lazy_static::lazy_static;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Token<'a> {
        Letter(char),
        Word(&'a str),
    }

    lazy_static! {
        pub static ref TREE: Node<&'static str> = Node::tree(vec![
            Node::branch(
                't',
                Some("letter"),
                vec![Node::branch(
                    'h',
                    None,
                    vec![Node::branch(
                        'e',
                        Some("word"),
                        vec![Node::leaf('e', "word")]
                    )]
                )]
            ),
            Node::branch('h', Some(&"letter"), vec![Node::leaf('e', "word")]),
            Node::leaf('e', "letter")
        ]);
        
        pub static ref TOKEN_TREE: Node<Token<'static>> = Node::tree(vec![
            Node::branch(
                't',
                Some(Token::Letter('t')),
                vec![Node::branch(
                    'h',
                    None,
                    vec![Node::branch(
                        'e',
                        Some(Token::Word("the")),
                        vec![Node::leaf('e', Token::Word("thee"))]
                    )]
                )]
            ),
            Node::branch(
                'h',
                Some(Token::Letter('h')),
                vec![Node::leaf('e', Token::Word("he"))]
            ),
            Node::leaf('e', Token::Letter('e'))
        ]);
    }

    #[test]
    fn input_paths() {
        println!("\n\nInput Paths:");
        for (k, v) in &TOKEN_TREE.input_paths() {
            println!("{k} => {v:?}")
        }
    }

    #[test]
    fn output_paths() {
        println!("\n\nOutput Paths:");
        for (k, v) in &TOKEN_TREE.output_paths() {
            println!("{k:?} <= {v:?}")
        }
    }

    #[test]
    fn results() {
        print!("\n\n");
        for sentence in ["t","the","thee","teh","ethehe","art","thj","th",] {
            println!("{}", sentence);
            println!("{:?}\n", TOKEN_TREE.extract_tokens(sentence));
        }
    }
}
