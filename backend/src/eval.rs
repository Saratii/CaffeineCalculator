use crate::{ast::{ASTNode, BinaryOperation, FunctionCall, UnaryOperation}, math::{average, factorial, max, median, min, standard_deviation, sum}};

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
            if a.operation == "average" || a.operation == "avg" || a.operation == "mean" {
                let mut vals = Vec::new();
                for child in a.inputs {
                    match evaluate_ast(child) {
                        Ok(a) => {
                            vals.push(a);
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
                Ok(average(&vals))
            } else if a.operation == "std" {
                let mut vals = Vec::new();
                for child in a.inputs {
                    match evaluate_ast(child) {
                        Ok(a) => {
                            vals.push(a);
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
                Ok(standard_deviation(&vals))
            } else if a.operation == "median" {
                let mut vals = Vec::new();
                for child in a.inputs {
                    match evaluate_ast(child) {
                        Ok(a) => {
                            vals.push(a);
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
                Ok(median(&vals))
            } else if a.operation == "min" {
                let mut vals = Vec::new();
                for child in a.inputs {
                    match evaluate_ast(child) {
                        Ok(a) => {
                            vals.push(a);
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
                Ok(min(&vals))
            } else if a.operation == "max" {
                let mut vals = Vec::new();
                for child in a.inputs {
                    match evaluate_ast(child) {
                        Ok(a) => {
                            vals.push(a);
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
                Ok(max(&vals))
            } else if a.operation == "sum" {
                let mut vals = Vec::new();
                for child in a.inputs {
                    match evaluate_ast(child) {
                        Ok(a) => {
                            vals.push(a);
                        }
                        Err(e) => {
                            return Err(format!("Syntax Error: {:?}", e));
                        }
                    }
                }
                Ok(sum(&vals))
            } else if a.operation == "sin" {
                evaluate_single_param(a, f64::sin)
            } else if a.operation == "cos" {
                evaluate_single_param(a, f64::cos)
            } else if a.operation == "tan" {
                evaluate_single_param(a, f64::tan)
            } else if a.operation == "asin" {
                evaluate_single_param(a, f64::asin)
            } else if a.operation == "acos" {
                evaluate_single_param(a, f64::acos)
            } else if a.operation == "atan" {
                evaluate_single_param(a, f64::atan)
            } else if a.operation == "sec" {
                evaluate_single_param(a, |x| 1. / f64::cos(x))
            } else if a.operation == "cot" {
                evaluate_single_param(a, |x| 1. / f64::tan(x))
            } else if a.operation == "csc" {
                evaluate_single_param(a, |x| 1. / f64::sin(x))
            } else if a.operation == "ln" {
                evaluate_single_param(a, |x| x.ln())
            } else if a.operation == "abs" {
                evaluate_single_param(a, |x| x.abs())
            } else if a.operation == "factorial" {
                evaluate_single_param(a, factorial)
            } else {
                return Err(format!("Unknown function: {:?}", a.operation));
            }
        }
        ASTNode::Comma => {
            return Err("Syntax Error, stray comma?".to_string());
        }
    }
}

fn evaluate_single_param(a: FunctionCall, func: fn(f64) -> f64) -> Result<f64, String> {
    if a.inputs.len() != 1 {
        return Err(format!("{} takes one argument moron", a.operation));
    }
    match evaluate_ast(a.inputs[0].clone()) {
        Ok(value) => Ok(func(value)),
        Err(e) => Err(format!("Syntax Error: {:?}", e)),
    }
}