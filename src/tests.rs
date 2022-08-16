#[cfg(test)]
mod test {
    use crate::Node;
    use lazy_static::lazy_static;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Output {
        Letter(char),
        Word(&'static str),
    }

    lazy_static! {
        pub static ref TOKEN_TREE: Node<Output> = Node::tree(vec![
            Node::branch('t', Some(Output::Letter('t')),
                vec![
                    Node::branch('h', None,
                        vec![
                            Node::branch('e', Some(Output::Word("the")),
                                vec![Node::leaf('e', Output::Word("thee")
                        )]
                    )]
                )]
            ),

            Node::branch('h', Some(Output::Letter('h')),
                vec![
                    Node::branch('e', Some(Output::Word("he")),
                        vec![
                            Node::branch('a', None,
                                vec![
                                    Node::leaf('t', Output::Word("heat")),

                                    Node::branch('r', Some(Output::Word("hear")),
                                        vec![
                                            Node::leaf('t', Output::Word("heart")),
                                        ]
                                    )
                                ]
                            )
                        ]
                    )
                ]
            ),
            Node::leaf('e', Output::Letter('e'))
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
        for sentence in ["t","thee","teh","th","art","tart","thj","hehearheatheart",] {
            println!("{}", sentence);
            println!("{:?}\n", TOKEN_TREE.extract_tokens(sentence));
        }
    }

    #[test]
    fn results_infallible() {
        print!("\n\n");
        for sentence in ["t","thee","teh","th","art","tart","thj","hehearheatheart",] {
            println!("{}", sentence);
            println!("{:?}\n", TOKEN_TREE.extract_tokens_infallible(sentence));
        }
    }
}
