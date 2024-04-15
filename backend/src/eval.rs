use crate::{ast::{ASTNode, BinaryOperation, FunctionCall, UnaryOperation}, math::{average, factorial, max, median, min, standard_deviation, sum, validate}};

pub const FUNCTIONS: &'static [&'static str] = &["sum", "average", "sin", "cos", "tan", "asin", "acos", "atan", "sec", "csc", "cot", "ln", "factorial", "mean", "median", "mode", "average", "avg", "abs", "max", "min", "std"];

enum Function{
    OneToOne(fn(f64) -> f64),
    MultiToOne(fn(Vec<f64>) -> f64),
}

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
                        BinaryOperation::Modulus => {
                            return Ok(left_result % right_result)
                        },
                    }
                }
                //reduncant eror handeling and logging
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
            return evaluate_function(a)
        }
        ASTNode::Comma => {
            return Err("Syntax Error, stray comma?".to_string());
        }
        ASTNode::Variable(_) => {
            return Err("Syntax Error".to_owned());
        }
    }
}

pub fn evaluate_function(function_call: FunctionCall) -> Result<f64, String> {
    let func;
    if function_call.operation == "average" || function_call.operation == "avg" || function_call.operation == "mean" {
        func = Function::MultiToOne(average);
    } else if function_call.operation == "std" {
        func = Function::MultiToOne(standard_deviation);
    } else if function_call.operation == "median" {
        func = Function::MultiToOne(median);
    } else if function_call.operation == "min" {
        func = Function::MultiToOne(min);
    } else if function_call.operation == "max" {
        func = Function::MultiToOne(max);
    } else if function_call.operation == "sum" {
        func = Function::MultiToOne(sum);
    } else if function_call.operation == "sin" {
        func = Function::OneToOne(f64::sin);
    } else if function_call.operation == "cos" {
        func = Function::OneToOne(f64::cos);
    } else if function_call.operation == "tan" {
        func = Function::OneToOne(f64::tan);
    } else if function_call.operation == "asin" {
        func = Function::OneToOne(f64::asin);
    } else if function_call.operation == "acos" {
        func = Function::OneToOne(f64::acos);
    } else if function_call.operation == "atan" {
        func = Function::OneToOne(f64::atan);
    } else if function_call.operation == "sec" {
        func = Function::OneToOne(|x| 1. / f64::cos(x));
    } else if function_call.operation == "cot" {
        func = Function::OneToOne(|x| 1. / f64::tan(x));
    } else if function_call.operation == "csc" {
        func = Function::OneToOne(|x| 1. / f64::sin(x));
    } else if function_call.operation == "ln" {
        func = Function::OneToOne(|x| x.ln());
    } else if function_call.operation == "abs" {
        func = Function::OneToOne(|x| x.abs());
    } else if function_call.operation == "factorial" {
        func = Function::OneToOne(factorial);
    } else {
        return Err(format!("Unknown function: {:?}", function_call.operation));
    }
    let mut paramaters = Vec::new();
    for child in function_call.inputs {
        match evaluate_ast(child) {
            Ok(a) => {
                paramaters.push(a);
            }
            Err(e) => {
                return Err(format!("Syntax Error: {:?}", e));
            }
        }
    }
    if validate(&paramaters, &function_call.operation){
        match func{
            Function::OneToOne(f) => Ok(f(paramaters[0])),
            Function::MultiToOne(f) => Ok(f(paramaters)),
        }
    } else {
        Err(format!("Invalid input for {}", function_call.operation))
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::build_ast, eval::evaluate_ast, tokens::tokenize};

    #[test]
    fn factorial_parse(){
        let tokens = tokenize("factorial(8)".to_string()).unwrap();
        let ast = build_ast(tokens).unwrap();
        let result = evaluate_ast(ast).unwrap();
        assert_eq!(result, 40320.);
    }   
}