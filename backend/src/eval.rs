use crate::ast::{ASTNode, BinaryOperation, UnaryOperation};

pub fn evaluate_ast(ast: ASTNode) -> f64 {
    match ast {
        ASTNode::BinaryNode(a) => {
            match a.operation {
                BinaryOperation::Plus => {
                    return evaluate_ast(*a.left) + evaluate_ast(*a.right);
                }
                BinaryOperation::Minus => {
                    return evaluate_ast(*a.left) - evaluate_ast(*a.right);
                }
                BinaryOperation::Times => {
                    return evaluate_ast(*a.left) * evaluate_ast(*a.right);
                }
                BinaryOperation::Divide => {
                    return evaluate_ast(*a.left) / evaluate_ast(*a.right);
                }
            }
        }
        ASTNode::UnaryNode(a) => {
            match a.operation {
                UnaryOperation::Negate => {
                    return -1.0 * evaluate_ast(*a.child);
                }
                UnaryOperation::Parens => {
                    return evaluate_ast(*a.child)
                }
            }
        }
        ASTNode::NumberNode(a) => {
            return a;
        }
        ASTNode::UnfinishedNode(a) => {
            panic!()
        }
    }
}