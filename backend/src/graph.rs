use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use crate::ast::{ASTNode, BinaryNode, BinaryOperation, build_ast, FunctionCall, UnaryNode, UnaryOperation};
use crate::eval::{evaluate_ast, evaluate_function_call};
use crate::tokens::Token;

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return f.write_str(&format!("{{\"x\":{},\"y\":{}}}", self.x, self.y));
    }
}

pub fn graph(mut tokens: VecDeque<Token>) -> Result<Vec<Point>, String> {
    if *tokens.front().unwrap() == Token::Graph {
        tokens.pop_front();
    }
    if *tokens.back().unwrap() == Token::RightParen {
        tokens.pop_back();
    }
    let mut left = VecDeque::new();
    let mut right = VecDeque::new();
    let mut found_equal = false;
    for token in tokens {
        if token == Token::Equal {
            found_equal = true;
        } else if found_equal {
            right.push_back(token);
        } else {
            left.push_back(token);
        }
    }
    let left = build_ast(left);
    let right = build_ast(right);
    match (left, right) {
        (Ok(left), Ok(right)) => {
            let left_variables = get_variables(&left);
            let right_variables = get_variables(&right);
            if left_variables.len() == 0 {
                match evaluate_ast(left) {
                    Ok(_left_val) => {
                        let _right = reduce_ast(&right);
                        panic!();
                    }
                    Err(_) => {
                        panic!();
                    }
                }
            } else if right_variables.len() == 0 {
                match evaluate_ast(right) {
                    Ok(_right_val) => {
                        let _left = reduce_ast(&left);
                        panic!();
                    }
                    Err(_) => {
                        panic!();
                    }
                }
            } else {
                let mut points = vec![];
                for i in -50..50 {
                    let variable = right_variables.get(0).unwrap();
                    let subbed_left = replace_variables(&left, variable, i as f64/5.0);
                    let subbed_right = replace_variables(&right, variable, i as f64/5.0);
                    let val = solve_for_variable(subbed_left, subbed_right);
                    points.push(Point {
                        x: i as f64 / 5.0,
                        y: val,
                    });
                }
                return Ok(points);
            }
        }
        (_,_) => {
            return Err("Syntax Error".to_owned())
        }
    }
}

fn solve_for_variable(mut left: ASTNode, mut right: ASTNode) -> f64 {
    if get_variables(&left).len() > 0 && get_variables(&right).len() == 0 {
        let mut right = evaluate_ast(right).unwrap();
        loop {
            match left.clone() {
                ASTNode::BinaryNode(a) => {
                    let left_node = reduce_ast(&a.left).unwrap();
                    let right_node = reduce_ast(&a.right).unwrap();
                    match (a.operation, left_node, right_node) {
                        (BinaryOperation::Plus, ASTNode::NumberNode(a), node) => {
                            left = node;
                            right -= a;
                        }
                        (BinaryOperation::Plus, node, ASTNode::NumberNode(a)) => {
                            left = node;
                            right -= a;
                        }
                        (BinaryOperation::Minus, ASTNode::NumberNode(a), node) => {
                            left = node;
                            right = a - right;
                        }
                        (BinaryOperation::Minus, node, ASTNode::NumberNode(a)) => {
                            left = node;
                            right = right + a;
                        }
                        (BinaryOperation::Times, ASTNode::NumberNode(a), node) => {
                            left = node;
                            right = right/a;
                        }
                        (BinaryOperation::Times, node, ASTNode::NumberNode(a)) => {
                            left = node;
                            right = right/a;
                        }
                        (BinaryOperation::Divide, ASTNode::NumberNode(a), node) => {
                            left = node;
                            right = a/right;
                        }
                        (BinaryOperation::Divide, node, ASTNode::NumberNode(a)) => {
                            left = node;
                            right = a*right;
                        }
                        (BinaryOperation::Exponent, ASTNode::NumberNode(a), node) => {
                            left = node;
                            right = right.ln()/a.ln();
                        }
                        (BinaryOperation::Exponent, node, ASTNode::NumberNode(a)) => {
                            left = node;
                            right = right.powf(1.0/a);
                        }
                        _ => {
                            panic!();
                        }
                    }
                }
                ASTNode::UnaryNode(a) => {
                    match a.operation {
                        UnaryOperation::Negate => {
                            right = -1.0 * right;
                            left = *a.child;
                        }
                        UnaryOperation::Parens => {
                            left = *a.child;
                        }
                    }
                }
                ASTNode::NumberNode(_) => {
                    panic!("am tired");
                }
                ASTNode::UnfinishedNode(_) => {
                    panic!("this shouldn't happen");
                }
                ASTNode::FunctionCall(_) => {
                    panic!("No functions allowed in graphing");
                }
                ASTNode::Comma => {
                    panic!("bad");
                }
                ASTNode::Variable(_) => {
                    return right;
                }
            }
        }
    } else {
        let mut left = evaluate_ast(left).unwrap();
        loop {
            match right.clone() {
                ASTNode::BinaryNode(a) => {
                    let left_node = reduce_ast(&a.left).unwrap();
                    let right_node = reduce_ast(&a.right).unwrap();
                    match (a.operation, left_node, right_node) {
                        (BinaryOperation::Plus, ASTNode::NumberNode(a), node) => {
                            right = node;
                            left -= a;
                        }
                        (BinaryOperation::Plus, node, ASTNode::NumberNode(a)) => {
                            right = node;
                            left -= a;
                        }
                        (BinaryOperation::Minus, ASTNode::NumberNode(a), node) => {
                            right = node;
                            left = a - left;
                        }
                        (BinaryOperation::Minus, node, ASTNode::NumberNode(a)) => {
                            right = node;
                            left = left + a;
                        }
                        (BinaryOperation::Times, ASTNode::NumberNode(a), node) => {
                            right = node;
                            left = left/a;
                        }
                        (BinaryOperation::Times, node, ASTNode::NumberNode(a)) => {
                            right = node;
                            left = left/a;
                        }
                        (BinaryOperation::Divide, ASTNode::NumberNode(a), node) => {
                            right = node;
                            left = a/left;
                        }
                        (BinaryOperation::Divide, node, ASTNode::NumberNode(a)) => {
                            right = node;
                            left = a*left;
                        }
                        (BinaryOperation::Exponent, ASTNode::NumberNode(a), node) => {
                            right = node;
                            left = left.ln()/a.ln();
                        }
                        (BinaryOperation::Exponent, node, ASTNode::NumberNode(a)) => {
                            right = node;
                            left = left.powf(1.0/a);
                        }
                        _ => {
                            panic!();
                        }
                    }
                }
                ASTNode::UnaryNode(a) => {
                    match a.operation {
                        UnaryOperation::Negate => {
                            left = -1.0 * left;
                            right = *a.child;
                        }
                        UnaryOperation::Parens => {
                            right = *a.child;
                        }
                    }
                }
                ASTNode::NumberNode(_) => {
                    panic!("am tired");
                }
                ASTNode::UnfinishedNode(_) => {
                    panic!("this shouldn't happen");
                }
                ASTNode::FunctionCall(_) => {
                    panic!("No functions allowed in graphing");
                }
                ASTNode::Comma => {
                    panic!("bad");
                }
                ASTNode::Variable(_) => {
                    return left;
                }
            }
        }
    }
}

fn get_variables(node: &ASTNode) -> Vec<String> {
    let mut variables = vec![];
    match node {
        ASTNode::BinaryNode(a) => {
            let left_vars = get_variables(&*a.left);
            let right_vars = get_variables(&*a.right);
            for var in left_vars {
                variables.push(var);
            }
            for var in right_vars {
                variables.push(var);
            }
        }
        ASTNode::UnaryNode(a) => {
            for var in get_variables(&*a.child) {
                variables.push(var);
            }
        }
        ASTNode::NumberNode(_) => {}
        ASTNode::UnfinishedNode(_) => {}
        ASTNode::FunctionCall(_) => {}
        ASTNode::Comma => {}
        ASTNode::Variable(a) => {
            variables.push(a.clone());
        }
    }
    return variables;
}

fn reduce_ast(node: &ASTNode) -> Result<ASTNode, String> {
    match node {
        ASTNode::BinaryNode(a) => {
            let left = reduce_ast(&*a.left).unwrap();
            let right = reduce_ast(&*a.right).unwrap();
            match (left, right, &a.operation) {
                (ASTNode::NumberNode(left), ASTNode::NumberNode(right), BinaryOperation::Plus) => {
                    return Ok(ASTNode::NumberNode(left + right));
                }
                (ASTNode::NumberNode(left), ASTNode::NumberNode(right), BinaryOperation::Times) => {
                    return Ok(ASTNode::NumberNode(left * right));
                }
                (ASTNode::NumberNode(left), ASTNode::NumberNode(right), BinaryOperation::Minus) => {
                    return Ok(ASTNode::NumberNode(left - right));
                }
                (ASTNode::NumberNode(left), ASTNode::NumberNode(right), BinaryOperation::Divide) => {
                    return Ok(ASTNode::NumberNode(left / right));
                }
                (ASTNode::NumberNode(left), ASTNode::NumberNode(right), BinaryOperation::Exponent) => {
                    return Ok(ASTNode::NumberNode(left.powf(right)));
                }
                (ASTNode::NumberNode(left), ASTNode::NumberNode(right), BinaryOperation::Modulus) => {
                    return Ok(ASTNode::NumberNode(left % right));
                }
                (left, right, b) => {
                    return Ok(ASTNode::BinaryNode(BinaryNode {
                        priority: a.priority,
                        left: Box::new(left),
                        right: Box::new(right),
                        operation: b.clone(),
                    }));
                }
            }
        }
        ASTNode::UnaryNode(a) => {
            let child = reduce_ast(&*a.child).unwrap();
            match (child, &a.operation) {
                (ASTNode::NumberNode(a),UnaryOperation::Parens) => {
                    return Ok(ASTNode::NumberNode(a));
                }
                (ASTNode::NumberNode(a),UnaryOperation::Negate) => {
                    return Ok(ASTNode::NumberNode(-a));
                }
                (b, c) => {
                    return Ok(ASTNode::UnaryNode(UnaryNode {
                        priority: a.priority,
                        child: Box::new(b),
                        operation: c.clone(),
                    }));
                }
            }
        }
        ASTNode::NumberNode(_) => {
            return Ok(node.clone());
        }
        ASTNode::UnfinishedNode(_) | ASTNode::Comma => {
            panic!();
        }
        ASTNode::FunctionCall(a) => {
            let mut inputs = VecDeque::new();
            let mut non_reducable = false;
            for input in a.inputs.clone() {
                let input = reduce_ast(&input).unwrap();
                match input {
                    ASTNode::NumberNode(_) => {}
                    _ => {
                        non_reducable = true;
                    }
                }
                inputs.push_back(input);
            }
            if non_reducable {
                return Ok(ASTNode::FunctionCall(FunctionCall {
                    inputs,
                    operation: a.operation.clone(),
                }));
            } else {
                match evaluate_function_call(FunctionCall {
                    inputs,
                    operation: a.operation.clone(),
                }) {
                    Ok(a) => {
                        return Ok(ASTNode::NumberNode(a));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        ASTNode::Variable(_) => {
            return Ok(node.clone());
        }
    }
}

fn replace_variables(node: &ASTNode, variable: &String, value: f64) -> ASTNode {
    match node {
        ASTNode::BinaryNode(a) => {
            return ASTNode::BinaryNode(BinaryNode {
                priority: a.priority,
                left: Box::new(replace_variables(&*a.left, variable, value)),
                right: Box::new(replace_variables(&*a.right, variable, value)),
                operation: a.operation.clone(),
            });
        }
        ASTNode::UnaryNode(a) => {
            return ASTNode::UnaryNode(UnaryNode {
                priority: a.priority,
                child: Box::new(replace_variables(&*a.child, variable, value)),
                operation: a.operation.clone(),
            });
        }
        ASTNode::NumberNode(_a) => {
            return node.clone();
        }
        ASTNode::UnfinishedNode(_a) => {
            panic!();
        }
        ASTNode::FunctionCall(a) => {
            let mut inputs = VecDeque::new();
            for input in a.inputs.clone() {
                inputs.push_back(replace_variables(&input, &variable, value));
            }
            return ASTNode::FunctionCall(FunctionCall {
                inputs,
                operation: a.operation.clone(),
            })
        }
        ASTNode::Comma => {
            return node.clone();
        }
        ASTNode::Variable(a) => {
            if *a == *variable {
                return ASTNode::NumberNode(value);
            } else {
                return node.clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::{graph, Point};
    use crate::tokens::tokenize;

    #[test]
    fn graph_of_linear_equation_works() {
        let input = tokenize("graph(y=5x)".to_string()).unwrap();
        let output = graph(input);
        let mut expected_results = vec![];
        for i in -50..50 {
            expected_results.push(Point {
                x: i as f64 / 5.0,
                y: i as f64,
            });
        }
        assert_eq!(output, Ok(expected_results));
    }
}