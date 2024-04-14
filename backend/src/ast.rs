use std::collections::VecDeque;
use crate::tokens::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum ASTNode {
    BinaryNode(BinaryNode),
    UnaryNode(UnaryNode),
    NumberNode(f64),
    UnfinishedNode(UnfinishedNode),
    FunctionCall(FunctionCall),
    Comma,
    Variable(String),
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
pub struct FunctionCall {
    pub inputs: VecDeque<ASTNode>,
    pub operation: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Times,
    Divide,
    Exponent,
    Modulus,
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
    Modulus,
    Exponent,
    LeftParen,
    Negate,
    FunctionCall(String),
}

pub fn build_ast(mut tokens: VecDeque<Token>) -> Result<ASTNode, String> {
    let mut stack = vec![];
    while !tokens.is_empty() {
        let token = tokens.pop_front().unwrap();
        match token {
            Token::Number(a) => {
                stack.push(ASTNode::NumberNode(a));
                let _ = combine_finished_val(&mut stack);
            }
            Token::Plus => {
                stack.push(ASTNode::UnfinishedNode(UnfinishedNode::Plus))
            }
            Token::Minus => {
                if stack.len() > 0 {
                    let pred = stack.pop().unwrap();
                    match pred{
                        ASTNode::UnfinishedNode(_) => {
                            stack.push(pred);
                            stack.push(ASTNode::UnfinishedNode(UnfinishedNode::Negate))
                        },
                        _ => {
                            stack.push(pred);
                            stack.push(ASTNode::UnfinishedNode(UnfinishedNode::Minus))
                        }
                    }
                } else {
                    stack.push(ASTNode::UnfinishedNode(UnfinishedNode::Negate));
                }
            }
            Token::Times => {
                stack.push(ASTNode::UnfinishedNode(UnfinishedNode::Times))
            }
            Token::Divide => {
                stack.push(ASTNode::UnfinishedNode(UnfinishedNode::Divide))
            }
            Token::Modulus => {
                stack.push(ASTNode::UnfinishedNode(UnfinishedNode::Modulus))
            }
            Token::Exponent => {
                stack.push(ASTNode::UnfinishedNode(UnfinishedNode::Exponent))
            }
            Token::LeftParen => {
                if stack.len() > 0 {
                    let pred = stack.pop().unwrap();
                    match pred.clone() {
                        ASTNode::BinaryNode(_) | ASTNode::NumberNode(_) | ASTNode::Variable(_) | ASTNode::FunctionCall(_) => {
                            stack.push(pred);
                            stack.push(ASTNode::UnfinishedNode(UnfinishedNode::Times));
                        },
                        ASTNode::UnaryNode(a) => {
                            match a.operation{
                                UnaryOperation::Negate => {
                                    stack.push(pred);
                                },
                                UnaryOperation::Parens => {
                                    stack.push(pred);
                                    stack.push(ASTNode::UnfinishedNode(UnfinishedNode::Times));
                                },
                            }
                        },
                        ASTNode::UnfinishedNode(_) => {
                            stack.push(pred);
                        },
                        ASTNode::Comma => {
                            stack.push(pred);
                        }
                    }
                }
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
                        (val, ASTNode::Comma) => {
                            let mut children = VecDeque::new();
                            children.push_back(val);
                            while stack.len() > 1 {
                                let val = stack.pop().unwrap();
                                let left = stack.pop().unwrap();
                                match left {
                                    ASTNode::Comma => {
                                        children.push_back(val);
                                    }
                                    ASTNode::UnfinishedNode(UnfinishedNode::FunctionCall(a)) => {
                                        children.push_back(val);
                                        stack.push(ASTNode::FunctionCall(FunctionCall {
                                            inputs: children,
                                            operation: a,
                                        }));
                                        match combine_finished_val(&mut stack) {
                                            Ok(_) => {}
                                            Err(e) => {
                                                return Err(e)
                                            }
                                        }
                                        break;
                                    }
                                    _ => {

                                    }
                                }
                            }
                        }
                        (val, ASTNode::UnfinishedNode(UnfinishedNode::FunctionCall(a))) => {
                            let mut children = VecDeque::new();
                            children.push_back(val);
                            stack.push(ASTNode::FunctionCall(FunctionCall {
                                inputs: children,
                                operation: a,
                            }));
                        }
                        _ => {
                            panic!();
                        }
                    }
                    match combine_finished_val(&mut stack){
                        Ok(_) => {},
                        Err(e) => {return Err(e);},
                    }
                }
            }
            Token::Comma => {
                stack.push(ASTNode::Comma);
            }
            Token::FunctionCall(a) => {
                stack.push(ASTNode::UnfinishedNode(UnfinishedNode::FunctionCall(a)));
            }
            Token::Help => {
                return Err("Don't embed help in expressions".to_string());
            }
            Token::Variable(a) => {
                stack.push(ASTNode::Variable(a));
                let _ = combine_finished_val(&mut stack);
            }
            Token::Equal | Token::Graph => {
                return Err("I'll get io it".to_string());
            }
        }
    }
    Ok(stack.pop().unwrap())
}

fn combine_finished_val(stack: &mut Vec<ASTNode>) -> Result<(), String> {
    if stack.len() == 1 {
        return Ok(())
    }
    let right= stack.pop().unwrap();
    let op = stack.pop().unwrap();
    match op {
        ASTNode::UnfinishedNode(UnfinishedNode::LeftParen) | ASTNode::Comma => {
            stack.push(op);
            stack.push(right);
            Ok(())
        }
        ASTNode::UnfinishedNode(op) => {
            if stack.len() == 0 {
                match op {
                    UnfinishedNode::Negate => {
                        stack.push(ASTNode::UnaryNode(UnaryNode {
                            priority: 9,
                            child: Box::new(right),
                            operation: UnaryOperation::Negate,
                        }));
                        Ok(())
                    }
                    UnfinishedNode::FunctionCall(a) => {
                        stack.push(ASTNode::UnfinishedNode(UnfinishedNode::FunctionCall(a)));
                        stack.push(right);
                        Ok(())
                    }
                    UnfinishedNode::Plus => {
                        stack.push(right);
                        Ok(())
                    }
                    _ => {
                        Err(format!("Invalid sequence: {:?}", stack))
                    }
                }
            } else {
                let left = stack.pop().unwrap();
                match left {
                    ASTNode::UnfinishedNode(_) => {
                        match op {
                            UnfinishedNode::Minus => {
                                stack.push(left);
                                stack.push(ASTNode::UnaryNode(UnaryNode {
                                    priority: 9,
                                    child: Box::new(right),
                                    operation: UnaryOperation::Negate
                                }));
                            },
                            UnfinishedNode::Negate => {
                                stack.push(left);
                                stack.push(ASTNode::UnaryNode(UnaryNode {
                                    priority: 9,
                                    child: Box::new(right),
                                    operation: UnaryOperation::Negate
                                }));
                            },
                            _ => {
                                return Err(format!("Invalid sequence: {:?}", stack))
                            }
                        }
                        return combine_finished_val(stack);
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
                            UnfinishedNode::Exponent => {
                                ASTNode::BinaryNode(BinaryNode {
                                    priority: 3,
                                    left: Box::new(left),
                                    right: Box::new(right),
                                    operation: BinaryOperation::Exponent,
                                })
                            }
                            UnfinishedNode::Modulus => {
                                ASTNode::BinaryNode(BinaryNode {
                                    priority: 2,
                                    left: Box::new(left),
                                    right: Box::new(right),
                                    operation: BinaryOperation::Modulus,
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
                        return combine_finished_val(stack);
                    }
                }
            }
        }
        ASTNode::NumberNode(a) => {
            match right {
                ASTNode::Variable(_) => {
                    stack.push(ASTNode::BinaryNode(BinaryNode {
                        priority: 2,
                        left: Box::new(ASTNode::NumberNode(a)),
                        right: Box::new(right),
                        operation: BinaryOperation::Times,
                    }));
                    Ok(())
                }
                _ => {
                    panic!();
                }
            }
        }
        ASTNode::BinaryNode(a) => {
            stack.push(apply_priority(ASTNode::BinaryNode(BinaryNode {
                priority: 2,
                left: Box::new(ASTNode::BinaryNode(a)),
                right: Box::new(right),
                operation: BinaryOperation::Times,
            })));
            Ok(())
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
        ASTNode::UnaryNode(_) | ASTNode::NumberNode(_) | ASTNode::Variable(_) | ASTNode::UnfinishedNode(_) | ASTNode::Comma | ASTNode::FunctionCall(_) => {
            node
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use crate::ast::{ASTNode, BinaryNode, BinaryOperation, build_ast, FunctionCall, UnaryNode, UnaryOperation};
    use crate::tokens::tokenize;

    #[test]
    fn addition_ast() {
        let tokens = tokenize("1+2".to_string());
        let ast = build_ast(tokens.unwrap());
        assert_eq!(ast, Ok(ASTNode::BinaryNode(BinaryNode {
            priority: 1,
            left: Box::new(ASTNode::NumberNode(1.0)),
            right: Box::new(ASTNode::NumberNode(2.0)),
            operation: BinaryOperation::Plus,
        })));
    }

    #[test]
    fn subtraction_ast() {
        let tokens = tokenize("1-2".to_string());
        let ast = build_ast(tokens.unwrap());
        assert_eq!(ast, Ok(ASTNode::BinaryNode(BinaryNode {
            priority: 1,
            left: Box::new(ASTNode::NumberNode(1.0)),
            right: Box::new(ASTNode::NumberNode(2.0)),
            operation: BinaryOperation::Minus,
        })));
    }

    #[test]
    fn multiplication_ast() {
        let tokens = tokenize("1 * 2".to_string());
        let ast = build_ast(tokens.unwrap());
        assert_eq!(ast, Ok(ASTNode::BinaryNode(BinaryNode {
            priority: 2,
            left: Box::new(ASTNode::NumberNode(1.0)),
            right: Box::new(ASTNode::NumberNode(2.0)),
            operation: BinaryOperation::Times,
        })));
    }

    #[test]
    fn addition_and_multiplication_ast() {
        let tokens = tokenize("1 + 2 * 3 + 4".to_string());
        let ast = build_ast(tokens.unwrap());
        assert_eq!(ast, Ok(ASTNode::BinaryNode(BinaryNode {
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
        })));
    }

    #[test]
    fn parenthetical_test_ast() {
        let tokens = tokenize(" 1 + 2 * 3 + ( 4 - 5 ) * 6 ".to_string());
        let ast = build_ast(tokens.unwrap());
        assert_eq!(ast, Ok(ASTNode::BinaryNode(BinaryNode {
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
        })));
    }

    #[test]
    fn op_then_negate() {
        let input = build_ast(tokenize("-2".to_string()).unwrap()).unwrap();
        assert_eq!(input, ASTNode::UnaryNode(UnaryNode {
            priority: 9,
            child: Box::new(ASTNode::NumberNode(2.0)),
            operation: UnaryOperation::Negate,
        }));
        let input = build_ast(tokenize("--2".to_string()).unwrap()).unwrap();
        assert_eq!(input, ASTNode::UnaryNode(UnaryNode {
            priority: 9,
            child: Box::new(ASTNode::UnaryNode(UnaryNode {
                priority: 9,
                child: Box::new(ASTNode::NumberNode(2.0)),
                operation: UnaryOperation::Negate,
            })),
            operation: UnaryOperation::Negate,
        }));
        let input = build_ast(tokenize("3*-2".to_string()).unwrap()).unwrap();
        assert_eq!(input, ASTNode::BinaryNode(BinaryNode {
            priority: 2,
            left: Box::new(ASTNode::NumberNode(3.0)),
            right: Box::new(ASTNode::UnaryNode(UnaryNode {
                priority: 9,
                child: Box::new(ASTNode::NumberNode(2.0)),
                operation: UnaryOperation::Negate,
            })),
            operation: BinaryOperation::Times,
        }));
    }

    #[test]
    fn exponent_ast() {
        let tokens = tokenize("10^2".to_string());
        let ast = build_ast(tokens.unwrap());
        assert_eq!(ast, Ok(ASTNode::BinaryNode(BinaryNode {
            priority: 3,
            left: Box::new(ASTNode::NumberNode(10.0)),
            right: Box::new(ASTNode::NumberNode(2.0)),
            operation: BinaryOperation::Exponent,
        })));
    }

    #[test]
    fn modulus_ast() {
        let tokens = tokenize("10%5".to_string());
        let ast = build_ast(tokens.unwrap()).unwrap();
        assert_eq!(ast, ASTNode::BinaryNode(BinaryNode {
            priority: 2,
            left: Box::new(ASTNode::NumberNode(10.0)),
            right: Box::new(ASTNode::NumberNode(5.0)),
            operation: BinaryOperation::Modulus,
        }));
    }

    #[test]
    fn average_ast() {
        let tokens = tokenize("average(1)".to_string()).unwrap();
        let ast = build_ast(tokens).unwrap();
        let mut children = VecDeque::new();
        children.push_front(ASTNode::NumberNode(1.0));
        assert_eq!(ast, ASTNode::FunctionCall(FunctionCall {
            inputs: children,
            operation: "average".to_string(),
        }));
        let ast = build_ast(tokenize("average(1,2,3)".to_string()).unwrap()).unwrap();
        let mut children = VecDeque::new();
        children.push_front(ASTNode::NumberNode(1.0));
        children.push_front(ASTNode::NumberNode(2.0));
        children.push_front(ASTNode::NumberNode(3.0));
        assert_eq!(ast, ASTNode::FunctionCall(FunctionCall {
            inputs: children,
            operation: "average".to_string(),
        }));
    }

    #[test]
    fn expressions_with_variables_ast() {
        let tokens = tokenize("2x+5y-10".to_string()).unwrap();
        let ast = build_ast(tokens).unwrap();
        assert_eq!(ast, ASTNode::BinaryNode(BinaryNode {
            priority: 1,
            left: Box::new(ASTNode::BinaryNode(BinaryNode {
                priority: 1,
                left: Box::new(ASTNode::BinaryNode(BinaryNode {
                    priority: 2,
                    left: Box::new(ASTNode::NumberNode(2.0)),
                    right: Box::new(ASTNode::Variable("x".to_string())),
                    operation: BinaryOperation::Times,
                })),
                right: Box::new(ASTNode::BinaryNode(BinaryNode {
                    priority: 2,
                    left: Box::new(ASTNode::NumberNode(5.0)),
                    right: Box::new(ASTNode::Variable("y".to_string())),
                    operation: BinaryOperation::Times,
                })),
                operation: BinaryOperation::Plus,
            })),
            right: Box::new(ASTNode::NumberNode(10.0)),
            operation: BinaryOperation::Minus,
        }))
    }
}