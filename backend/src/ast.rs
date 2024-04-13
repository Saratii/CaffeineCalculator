use std::collections::VecDeque;
use crate::tokens::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum ASTNode {
    BinaryNode(BinaryNode),
    UnaryNode(UnaryNode),
    NumberNode(f64),
    UnfinishedNode(UnfinishedNode),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryNode {
    pub priority: u64,
    pub left: Box<ASTNode>,
    pub right: Box<ASTNode>,
    pub operation: BinaryOperation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryNode {
    pub priority: u64,
    pub child: Box<ASTNode>,
    pub operation: UnaryOperation
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperation {
    Negate,
    Parens
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnfinishedNode {
    Plus,
    Minus,
    Times,
    Divide,
    LeftParen,
}

pub fn build_ast(mut tokens: VecDeque<Token>) -> ASTNode {
    let mut stack = vec![];
    while !tokens.is_empty() {
        let token = tokens.pop_front().unwrap();
        match token {
            Token::Number(a) => {
                stack.push(ASTNode::NumberNode(a));
                combine_finished_val(&mut stack);
            }
            Token::Plus => {
                stack.push(ASTNode::UnfinishedNode(UnfinishedNode::Plus))
            }
            Token::Minus => {
                stack.push(ASTNode::UnfinishedNode(UnfinishedNode::Minus))
            }
            Token::Times => {
                stack.push(ASTNode::UnfinishedNode(UnfinishedNode::Times))
            }
            Token::Divide => {
                stack.push(ASTNode::UnfinishedNode(UnfinishedNode::Divide))
            }
            Token::LeftParen => {
                stack.push(ASTNode::UnfinishedNode(UnfinishedNode::LeftParen))
            }
            Token::RightParen => {
                if stack.len() > 1 {
                    let val = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    match (val, left) {
                        (ASTNode::UnfinishedNode(_), _) => {
                            panic!();
                        }
                        (val, ASTNode::UnfinishedNode(UnfinishedNode::LeftParen)) => {
                            stack.push(ASTNode::UnaryNode(UnaryNode {
                                priority: 10,
                                operation: UnaryOperation::Parens,
                                child: Box::new(val),
                            }));
                        }
                        _ => {
                            panic!();
                        }
                    }
                    combine_finished_val(&mut stack);
                }
            }
        }
    }
    return stack.pop().unwrap();
}

fn combine_finished_val(stack: &mut Vec<ASTNode>) {
    if stack.len() == 1 {
        return
    }
    let right= stack.pop().unwrap();
    let op = stack.pop().unwrap();
    match op {
        ASTNode::UnfinishedNode(UnfinishedNode::LeftParen) => {
            stack.push(op);
            stack.push(right);
        }
        ASTNode::UnfinishedNode(op) => {
            let left = stack.pop().unwrap();
            match left {
                ASTNode::UnfinishedNode(_) => {
                    panic!();
                }
                _ => {
                    stack.push(apply_priority(match op {
                        UnfinishedNode::Plus => {
                            ASTNode::BinaryNode(BinaryNode {
                                priority: 1,
                                left: Box::new(left),
                                right: Box::new(right),
                                operation: BinaryOperation::Plus,
                            })
                        }
                        UnfinishedNode::Minus => {
                            ASTNode::BinaryNode(BinaryNode {
                                priority: 1,
                                left: Box::new(left),
                                right: Box::new(right),
                                operation: BinaryOperation::Minus,
                            })
                        }
                        UnfinishedNode::Times => {
                            ASTNode::BinaryNode(BinaryNode {
                                priority: 2,
                                left: Box::new(left),
                                right: Box::new(right),
                                operation: BinaryOperation::Times,
                            })
                        }
                        UnfinishedNode::Divide => {
                            ASTNode::BinaryNode(BinaryNode {
                                priority: 2,
                                left: Box::new(left),
                                right: Box::new(right),
                                operation: BinaryOperation::Divide,
                            })
                        }
                        _ => {
                            panic!();
                        }
                    }));
                    combine_finished_val(stack);
                }
            }
        }
        _ => {
            panic!();
        }
    }
}

fn apply_priority(node: ASTNode) -> ASTNode {
    return match node.clone() {
        ASTNode::BinaryNode(a) => {
            match *a.left {
                ASTNode::BinaryNode(b) => {
                    if a.priority > b.priority {
                        ASTNode::BinaryNode(BinaryNode {
                            priority: b.priority,
                            left: Box::new(*b.left),
                            right: Box::new(apply_priority(ASTNode::BinaryNode(BinaryNode {
                                priority: a.priority,
                                left: Box::new(*b.right),
                                right: Box::new(*a.right),
                                operation: a.operation,
                            }))),
                            operation: b.operation,
                        })
                    } else {
                        node
                    }
                }
                _ => {
                    node
                }
            }
        }
        ASTNode::UnaryNode(_) | ASTNode::NumberNode(_) | ASTNode::UnfinishedNode(_) => {
            node
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTNode, BinaryNode, BinaryOperation, build_ast, UnaryNode, UnaryOperation};
    use crate::ast::ASTNode::NumberNode;
    use crate::tokens::tokenize;

    #[test]
    fn addition_ast() {
        let tokens = tokenize("1+2".to_string());
        let ast = build_ast(tokens);
        assert_eq!(ast, ASTNode::BinaryNode(BinaryNode {
            priority: 1,
            left: Box::new(ASTNode::NumberNode(1.0)),
            right: Box::new(ASTNode::NumberNode(2.0)),
            operation: BinaryOperation::Plus,
        }));
    }

    #[test]
    fn subtraction_ast() {
        let tokens = tokenize("1-2".to_string());
        let ast = build_ast(tokens);
        assert_eq!(ast, ASTNode::BinaryNode(BinaryNode {
            priority: 1,
            left: Box::new(ASTNode::NumberNode(1.0)),
            right: Box::new(ASTNode::NumberNode(2.0)),
            operation: BinaryOperation::Minus,
        }));
    }

    #[test]
    fn multiplication_ast() {
        let tokens = tokenize("1 * 2".to_string());
        let ast = build_ast(tokens);
        assert_eq!(ast, ASTNode::BinaryNode(BinaryNode {
            priority: 2,
            left: Box::new(ASTNode::NumberNode(1.0)),
            right: Box::new(ASTNode::NumberNode(2.0)),
            operation: BinaryOperation::Times,
        }));
    }

    #[test]
    fn addition_and_multiplication_ast() {
        let tokens = tokenize("1 + 2 * 3 + 4".to_string());
        let ast = build_ast(tokens);
        assert_eq!(ast, ASTNode::BinaryNode(BinaryNode {
            priority: 1,
            left: Box::new(
                ASTNode::BinaryNode(BinaryNode {
                    priority: 1,
                    left: Box::new(ASTNode::NumberNode(1.0)),
                    right: Box::new(ASTNode::BinaryNode(BinaryNode {
                        priority: 2,
                        left: Box::new(ASTNode::NumberNode(2.0)),
                        right: Box::new(ASTNode::NumberNode(3.0)),
                        operation: BinaryOperation::Times,
                    })),
                    operation: BinaryOperation::Plus,
                })
            ),
            right: Box::new(ASTNode::NumberNode(4.0)),
            operation: BinaryOperation::Plus,
        }));
    }

    #[test]
    fn parenthetical_test_ast() {
        let tokens = tokenize(" 1 + 2 * 3 + ( 4 - 5 ) * 6 ".to_string());
        let ast = build_ast(tokens);
        assert_eq!(ast, ASTNode::BinaryNode(BinaryNode {
            priority: 1,
            left: Box::new(ASTNode::BinaryNode(BinaryNode {
                priority: 1,
                left: Box::new(ASTNode::NumberNode(1.0)),
                right: Box::new(ASTNode::BinaryNode(BinaryNode {
                    priority: 2,
                    left: Box::new(ASTNode::NumberNode(2.0)),
                    right: Box::new(ASTNode::NumberNode(3.0)),
                    operation: BinaryOperation::Times,
                })),
                operation: BinaryOperation::Plus,
            })),
            right: Box::new(ASTNode::BinaryNode(BinaryNode {
                priority: 2,
                left: Box::new(ASTNode::UnaryNode(UnaryNode {
                    priority: 10,
                    child: Box::new(ASTNode::BinaryNode(BinaryNode {
                        priority: 1,
                        left: Box::new(ASTNode::NumberNode(4.0)),
                        right: Box::new(ASTNode::NumberNode(5.0)),
                        operation: BinaryOperation::Minus,
                    })),
                    operation: UnaryOperation::Parens,
                })),
                right: Box::new(ASTNode::NumberNode(6.0)),
                operation: BinaryOperation::Times,
            })),
            operation: BinaryOperation::Plus,
        }));
    }
}