use ::std::env;
use anyhow::{bail, Ok, Result};
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Operator(String),
    Function(String),
    Paren(char),
}

#[derive(Debug, Clone)]
enum Node {
    Number(f64),
    Operation {
        operator: String,
        left: Box<Node>,
        right: Box<Node>,
    },
    Function {
        name: String,
        argument: Box<Node>,
    },
}
fn get_tokens(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut s = input.chars().peekable();
    while let Some(&ch) = s.peek() {
        if ch.is_whitespace() {
            s.next();
        } else if ch.is_ascii_digit() || ch == '.' {
            let mut num = String::new();
            while let Some(&c) = s.peek() {
                if c.is_ascii_digit() || c == '.' {
                    num.push(c);
                    s.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Number(num.parse().unwrap()));
        } else if ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '^' {
            let mut operation = String::new();
            operation.push(ch);
            s.next();
            if ch == '^' && s.peek() == Some(&'^') {
                operation.push('^');
                s.next();
            }
            tokens.push(Token::Operator(operation));
        } else if ch == '(' || ch == ')' {
            tokens.push(Token::Paren(ch));
            s.next();
        } else if ch.is_alphabetic() {
            let mut function = String::new();
            while let Some(&c) = s.peek() {
                if c.is_alphabetic() {
                    function.push(c);
                    s.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Function(function));
        }
    }
    tokens
}

fn get_precedence(operator: &str) -> Result<usize, anyhow::Error> {
    match operator {
        "+" | "-" => Ok(1),
        "*" | "/" => Ok(2),
        "^^" => Ok(3),
        _ => bail!("Unknown operator: {}", operator),
    }
}

fn is_left_associative(operator: &str) -> Result<bool, anyhow::Error> {
    match operator {
        "+" | "-" | "*" | "/" => Ok(true),
        "^^" => Ok(false),
        _ => bail!("Unknown operator: {}", operator),
    }
}
fn shunting_yard(tokens: Vec<Token>) -> Result<Vec<Token>, anyhow::Error> {
    let mut output_queue: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => {
                output_queue.push(token);
            }
            Token::Function(_) => {
                operator_stack.push(token);
            }
            Token::Operator(ref op) => {
                while let Some(top) = operator_stack.last() {
                    match top {
                        Token::Operator(ref top_op) => {
                            let top_precedence = get_precedence(top_op)?;
                            let current_precedence = get_precedence(op)?;
                            let left_associative = is_left_associative(op)?;

                            if (top_precedence > current_precedence)
                                || (top_precedence == current_precedence && left_associative)
                            {
                                output_queue.push(operator_stack.pop().unwrap());
                            } else {
                                break;
                            }
                        }
                        _ => break,
                    }
                }
                operator_stack.push(token);
            }
            Token::Paren('(') => {
                operator_stack.push(token);
            }
            Token::Paren(')') => {
                while let Some(top) = operator_stack.last() {
                    if let Token::Paren('(') = top {
                        break;
                    } else {
                        output_queue.push(operator_stack.pop().unwrap());
                    }
                }

                if operator_stack.last() == Some(&Token::Paren('(')) {
                    operator_stack.pop();
                } else {
                    bail!("Incorrect or mismatched parentheses");
                }

                if let Some(&Token::Function(_)) = operator_stack.last() {
                    output_queue.push(operator_stack.pop().unwrap());
                }
            }
            _ => bail!("Unknown token: {:?}", token),
        }
    }

    while let Some(top) = operator_stack.pop() {
        if let Token::Paren(_) = top {
            bail!("Incorrect or mismatched parentheses");
        }
        output_queue.push(top);
    }

    Ok(output_queue)
}
fn create_ast(tokens: Vec<Token>) -> Result<Node, anyhow::Error> {
    let mut stack: Vec<Node> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(value) => stack.push(Node::Number(value)),
            Token::Operator(op) => {
                let right = stack.pop().expect("Operator withouth right operand");
                let left = stack.pop().expect("Operator withouth left operand");
                stack.push(Node::Operation {
                    operator: op,
                    left: Box::new(left),
                    right: Box::new(right),
                });
            }
            Token::Function(name) => {
                let argument = stack.pop().expect("Function withouth arguments");
                stack.push(Node::Function {
                    name,
                    argument: Box::new(argument),
                });
            }
            _ => bail!("Unknown token in RPN: {:?}", token),
        }
    }

    if stack.len() != 1 {
        bail!("AST tree is nod valid")
    }
    Ok(stack.pop().unwrap())
}
fn print_expr(expression: &mut str) {
    let fmt_expression = expression
        .replace("+", " + ")
        .replace("-", " - ")
        .replace("*", " * ")
        .replace("/", " / ")
        .replace("^^", " ^^ ");

    println!("= {}", fmt_expression);
}
fn print_input(expression: &str) {
    let fmt_expression = expression
        .replace("+", " + ")
        .replace("-", " - ")
        .replace("*", " * ")
        .replace("/", " / ")
        .replace("^^", " ^^ ");
    println!("{}", fmt_expression);
}
fn evaluate_ast(
    ast: &mut Node,
    expr: &mut String,
    last_step: String,
) -> Result<f64, anyhow::Error> {
    match ast {
        Node::Number(val) => Ok(*val),
        Node::Operation {
            operator,
            left,
            right,
        } => {
            let left_val = evaluate_ast(left, expr, expr.clone())?;
            let right_val = evaluate_ast(right, expr, expr.clone())?;

            let result = match operator.as_str() {
                "+" => left_val + right_val,
                "-" => left_val - right_val,
                "*" => left_val * right_val,
                "/" => left_val / right_val,
                "^^" => left_val.powf(right_val),
                _ => bail!("Unknown operator"),
            };

            let sub_expr = format!(
                "({}{}{})",
                format_value(left_val, false),
                operator,
                format_value(right_val, false)
            );

            if expr.contains(&sub_expr) {
                let pos = expr.find(&sub_expr).unwrap();
                let last_char = expr.chars().nth(pos - 1).unwrap();
                if last_char.is_alphabetic() {
                    *expr = expr.replace(&sub_expr, &format_value(result, true));
                } else {
                    *expr = expr.replace(&sub_expr, &format_value(result, false));
                }
            } else {
                let alt_sub_expr = sub_expr.replace("(", "").replace(")", "");
                *expr = expr.replace(&alt_sub_expr, &format_value(result, false));
            }
            if *expr != last_step {
                print_expr(expr);
            }

            Ok(result)
        }
        Node::Function { name, argument } => {
            let arg_val = evaluate_ast(argument, expr, expr.clone())?;

            let result = match name.as_str() {
                "sin" => arg_val.sin(),
                "cos" => arg_val.cos(),
                "sqrt" => arg_val.sqrt(),
                "log" => arg_val.ln(),
                _ => bail!("Unkonwn function: {}", name),
            };

            let sub_expr = format!("{}({})", name, format_value(arg_val, false));

            *expr = expr.replace(&sub_expr, &format_value(result, false));

            if *expr != last_step {
                print_expr(expr);
            }

            Ok(result)
        }
    }
}

fn format_value(val: f64, ok: bool) -> String {
    if val.fract() == 0.0 {
        if ok {
            format!("({})", val as i64)
        } else {
            format!("{}", val as i64)
        }
    } else if ok {
        format!("({})", val)
    } else {
        format!("{}", val)
    }
}
fn main() -> Result<(), anyhow::Error> {
    let vec: Vec<String> = env::args().collect();
    let input = vec[1].as_str();
    let val = input.replace(" ", "");
    let mod_input = val.as_str();

    print_input(mod_input);
    let tokens = get_tokens(mod_input);
    let rpn = shunting_yard(tokens)?;
    let mut tree = create_ast(rpn)?;
    let mut expr = mod_input.to_string();
    
    evaluate_ast(&mut tree, &mut expr, String::new())?;
    Ok(())
}
