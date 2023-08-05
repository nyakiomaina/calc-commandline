use std::io::{self, Write};
use std::str::FromStr;
use std::convert::TryFrom;
use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Debug, PartialEq, PartialOrd)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Expression {
    Number(f64),
    Operation(Box<Expression>, Operator, Box<Expression>),
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            _ => Err(()),
        }
    }
}

impl TryFrom<VecDeque<&str>> for Expression {
    type Error = ();

    fn try_from(mut expr: VecDeque<&str>) -> Result<Self, Self::Error> {
        let lhs:Expression = Expression::Number(expr.pop_front().ok_or(())?.parse::<f64>()
        .map_err(|_| ())?
        .into());

        if expr.is_empty() {
            return Ok(lhs);
        }

        let op = expr.pop_front().ok_or(())?.parse()?;
        let rhs = Self::try_from(expr)?;

        Ok(Expression::Operation(Box::new(lhs), op, Box::new(rhs)))
    }
}

impl Expression {
    fn eval(&self) -> Result<f64, ()> {
        match self {
            Expression::Number(n) => Ok(*n),
            Expression::Operation(lhs, op, rhs) => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;

                match op {
                    Operator::Add => Ok(lhs + rhs),
                    Operator::Subtract => Ok(lhs - rhs),
                    Operator::Multiply => Ok(lhs * rhs),
                    Operator::Divide => {
                        if rhs == 0.0 {
                            return Err(());
                        }
                        Ok(lhs / rhs)
                    }
                    
                }
            }
        }
    }
}

fn main () {
    loop {
        let mut buffer = String::new();

        print!("Enter and expression (or 'exit' to quit): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();

        let input = buffer.trim();

        if input == "exit" {
            break;
        }

        let parts: VecDeque<&str> = VecDeque::from_iter(input.split_whitespace());

        match Expression::try_from(parts) {
            Ok(expr) => {
                match expr.eval() {
                    Ok(result) => println!("Result: {:?}", result),
                    Err(_) => println!("Error: division by zero."),
                }
            }
            Err(_) => println!("Invalid expression")
        }
    }
}