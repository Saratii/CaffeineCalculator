use crate::{ast::{ASTNode, BinaryOperation, UnaryOperation}, math::{factorial, is_positive_integer}};

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
        ASTNode::FunctionCall(a) => {
            if a.operation == "average" || a.operation == "avg" {
                let mut total = 0.0;
                for child in a.inputs {
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
            } else if a.operation == "sin" {
                if a.inputs.len() > 1{
                    return Err("sin takes one arguement moron".to_string())
                } else {
                    match evaluate_ast(a.inputs[0].clone()) {
                        Ok(a) => {
                            Ok(a.sin())
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
            } else if a.operation == "cos" {
                if a.inputs.len() > 1{
                    return Err("cos takes one arguement moron".to_string())
                } else {
                    match evaluate_ast(a.inputs[0].clone()) {
                        Ok(a) => {
                            Ok(a.cos())
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
            } else if a.operation == "tan" {
                if a.inputs.len() > 1{
                    return Err("tan takes one arguement moron".to_string())
                } else {
                    match evaluate_ast(a.inputs[0].clone()) {
                        Ok(a) => {
                            Ok(a.tan())
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
            } else if a.operation == "asin" {
                if a.inputs.len() > 1{
                    return Err("asin takes one arguement moron".to_string())
                } else {
                    match evaluate_ast(a.inputs[0].clone()) {
                        Ok(a) => {
                            Ok(a.asin())
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
            } else if a.operation == "acos" {
                if a.inputs.len() > 1{
                    return Err("acos takes one arguement moron".to_string())
                } else {
                    match evaluate_ast(a.inputs[0].clone()) {
                        Ok(a) => {
                            Ok(a.acos())
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
            } else if a.operation == "atan" {
                if a.inputs.len() > 1{
                    return Err("atan takes one arguement moron".to_string())
                } else {
                    match evaluate_ast(a.inputs[0].clone()) {
                        Ok(a) => {
                            Ok(a.atan())
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
            } else if a.operation == "sec" {
                if a.inputs.len() > 1{
                    return Err("sec takes one arguement moron".to_string())
                } else {
                    match evaluate_ast(a.inputs[0].clone()) {
                        Ok(a) => {
                            Ok(1./a.cos())
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
            } else if a.operation == "cot" {
                if a.inputs.len() > 1{
                    return Err("cot takes one arguement moron".to_string())
                } else {
                    match evaluate_ast(a.inputs[0].clone()) {
                        Ok(a) => {
                            Ok(1./a.tan())
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
            } else if a.operation == "csc" {
                if a.inputs.len() > 1{
                    return Err("csc takes one arguement moron".to_string())
                } else {
                    match evaluate_ast(a.inputs[0].clone()) {
                        Ok(a) => {
                            Ok(1./a.sin())
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
            } else if a.operation == "ln" {
                if a.inputs.len() > 1{
                    return Err("ln takes one arguement moron".to_string())
                } else {
                    match evaluate_ast(a.inputs[0].clone()) {
                        Ok(a) => {
                            Ok(a.ln())
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
            } else if a.operation == "abs" {
                if a.inputs.len() > 1{
                    return Err("abs takes one arguement moron".to_string())
                } else {
                    match evaluate_ast(a.inputs[0].clone()) {
                        Ok(a) => {
                            Ok(a.abs())
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
            } else if a.operation == "factorial" {
                if a.inputs.len() > 1{
                    return Err("factorial takes one arguement moron".to_string())
                } else {
                    match evaluate_ast(a.inputs[0].clone()) {
                        Ok(a) => {
                            if is_positive_integer(a){
                                return Ok(factorial(a));
                            } else {
                                return Err("factorial takes positive integers moron".to_string())
                            }
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
            } else {
                return Err(format!("Unknown function: {:?}", a.operation));
            }
        }
        ASTNode::Comma => {
            return Err("Syntax Error, stray comma?".to_string());
        }
    }
}