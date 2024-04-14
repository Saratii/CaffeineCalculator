use crate::ast::{ASTNode, BinaryOperation, UnaryOperation};

pub fn evaluate_ast(ast: ASTNode) -> String {
    match ast {
        ASTNode::BinaryNode(a) => {
            match (evaluate_ast(*a.left), evaluate_ast(*a.right)) {
                (left_result, right_result) => {
                    match a.operation {
                        BinaryOperation::Plus => {
                            return (left_result.parse::<i64>().unwrap() + right_result.parse::<i64>().unwrap()).to_string();
                        }
                        BinaryOperation::Minus => {
                            return (left_result.parse::<i64>().unwrap() - right_result.parse::<i64>().unwrap()).to_string();
                        }
                        BinaryOperation::Times => {
                            return (left_result.parse::<i64>().unwrap() * right_result.parse::<i64>().unwrap()).to_string();
                        }
                        BinaryOperation::Divide => {
                            return (left_result.parse::<i64>().unwrap() / right_result.parse::<i64>().unwrap()).to_string();
                        }
                        BinaryOperation::Modulus => {
                            return (left_result.parse::<i64>().unwrap() % right_result.parse::<i64>().unwrap()).to_string();
                        }
                        BinaryOperation::Exponent => {
                            return (left_result.parse::<u32>().unwrap().pow(right_result.parse::<u32>().unwrap())).to_string();
                        }
                        BinaryOperation::GreaterThan => {
                            return (left_result.parse::<i64>().unwrap() > right_result.parse::<i64>().unwrap()).to_string();
                        }
                        BinaryOperation::LessThan => {
                            return (left_result.parse::<i64>().unwrap() < right_result.parse::<i64>().unwrap()).to_string();
                        }
                    }
                }
            }
            
        }
        ASTNode::UnaryNode(a) => {
            match evaluate_ast(*a.child){
                child => {
                    match a.operation {
                        UnaryOperation::Negate => {
                            return (-1.0 * child.parse::<f64>().unwrap()).to_string();
                        } 
                        UnaryOperation::Parens => {
                            return child
                        }
                    }
                },
                _ => {
                    println!("moron programmer");
                    return "moron programmer".to_string()
                },
            }
        }
        ASTNode::NumberNode(a) => {
            return a.to_string();
        }
        ASTNode::UnfinishedNode(a) => {
            return format!("Syntax Error: {:?}", a).to_owned()
        }
    }
}