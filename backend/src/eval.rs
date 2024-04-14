use crate::ast::{ASTNode, BinaryOperation, UnaryOperation, VariableInputNode, VariableInputOperation};

pub fn evaluate_ast(ast: ASTNode) -> Result<f64, String> {
    match ast {
        ASTNode::BinaryNode(a) => {
            match (evaluate_ast(*a.left), evaluate_ast(*a.right)) {
                (Ok(left_result), Ok(right_result)) => {
                    match a.operation {
                        BinaryOperation::Plus => {
                            return Ok(left_result + right_result);
                        }
                        BinaryOperation::Minus => {
                            return Ok(left_result - right_result);
                        }
                        BinaryOperation::Times => {
                            return Ok(left_result * right_result);
                        }
                        BinaryOperation::Divide => {
                            return Ok(left_result / right_result);
                        }
                        BinaryOperation::Exponent => {
                            return Ok(left_result.powf(right_result));
                        }                 
                        BinaryOperation::Modulus => todo!(),
                    }
                }
                (Ok(_), Err(e)) => {
                    println!("Error: {}", e);
                    return Err(e);
                },
                (Err(e), Ok(_)) => {
                    println!("Error: {}", e);
                    return Err(e);
                },
                (Err(e), Err(e2)) => {
                    println!("Errors: {} and {}", e, e2);
                    return Err(e)
                },
            }
            
        }
        ASTNode::UnaryNode(a) => {
            match evaluate_ast(*a.child){
                Ok(child) => {
                    match a.operation {
                        UnaryOperation::Negate => {
                            return Ok(-1.0 * child);
                        } 
                        UnaryOperation::Parens => {
                            return Ok(child)
                        }
                    }
                },
                Err(e) => {
                    println!("Error: {}", e);
                    return Err(e)
                },
            }
        }
        ASTNode::NumberNode(a) => {
            return Ok(a);
        }
        ASTNode::UnfinishedNode(a) => {
            return Err(format!("Syntax Error: {:?}", a).to_owned())
        }
        ASTNode::VariableInputNode(a) => {
            return match a.operation {
                VariableInputOperation::Average => {
                    let mut total = 0.0;
                    for child in a.children {
                        match evaluate_ast(child) {
                            Ok(a) => {
                                total += a;
                            }
                            Err(e) => {
                                return Err(format!("Syntax Error: {:?}", e));
                            }
                        }
                    }
                    Ok(total)
                }
            }
        }
        ASTNode::Comma => {
            return Err("Syntax Error, stray comma?".to_string());
        }
    }
}