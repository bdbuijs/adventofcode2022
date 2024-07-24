use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, one_of, space1, u64 as nomu64},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (input, monkey_vec) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let monkey_names = monkey_vec.iter().map(|(name, _)| *name).collect::<Vec<_>>();
    let mut monkeys = monkey_vec.into_iter().collect::<HashMap<&str, Monkey>>();
    loop {
        monkey_names.iter().for_each(|&monkey| {
            if let Some(Monkey::Operation { left, right, op }) = monkeys.get(monkey) {
                if let (Some(Monkey::Value(l)), Some(Monkey::Value(r))) =
                    (monkeys.get(left), monkeys.get(right))
                {
                    {
                        let value = op.apply(*l, *r);
                        monkeys
                            .get_mut(monkey)
                            .expect("Monkey has already been found")
                            .set(value);
                    }
                }
            }
        });
        if let Some(Monkey::Value(value)) = monkeys.get("root") {
            return value.to_string();
        }
    }
}

pub fn process_part2(input: &str) -> String {
    let (input, monkey_vec) = parse_input(input).unwrap();
    assert!(input.is_empty());
    let monkeys = monkey_vec.into_iter().collect::<HashMap<&str, Monkey>>();
    let root = Node::from_monkeys("root", &monkeys);
    match root {
        Node::Equals {
            mut left,
            mut right,
        } => {
            let (mut value, mut arm) = match (left.resolve(), right.resolve()) {
                (true, false) => {
                    if let &Node::Value(value) = left.as_ref() {
                        (value, right)
                    } else {
                        unreachable!("Resolve only returns true when we end up with a value")
                    }
                }
                (false, true) => {
                    if let &Node::Value(value) = right.as_ref() {
                        (value, left)
                    } else {
                        unreachable!("Resolve only returns true when we end up with a value")
                    }
                }
                _ => unreachable!("Broken tree"),
            };
            loop {
                match arm.as_mut() {
                    Node::Operation {
                        operation,
                        left,
                        right,
                    } => match (left.as_mut(), right.as_mut()) {
                        (Node::Human, Node::Value(x)) => {
                            return match operation {
                                Operation::Plus => value - *x,
                                Operation::Minus => value + *x,
                                Operation::Multiply => value / *x,
                                Operation::Divide => value * *x,
                            }
                            .to_string()
                        }
                        (Node::Value(x), Node::Human) => {
                            return match operation {
                                Operation::Plus => value - *x,
                                Operation::Minus => *x - value,
                                Operation::Multiply => value / *x,
                                Operation::Divide => *x / value,
                            }
                            .to_string()
                        }
                        (Node::Human, other) | (other, Node::Human) => loop {
                            if other.resolve() {
                                break;
                            }
                        },
                        (other, Node::Value(x)) => {
                            value = match operation {
                                Operation::Plus => value - *x,
                                Operation::Minus => value + *x,
                                Operation::Multiply => value / *x,
                                Operation::Divide => value * *x,
                            };
                            arm = Box::new(other.clone());
                        }
                        (Node::Value(x), other) => {
                            value = match operation {
                                Operation::Plus => value - *x,
                                Operation::Minus => *x - value,
                                Operation::Multiply => value / *x,
                                Operation::Divide => *x / value,
                            };
                            arm = Box::new(other.clone());
                        }
                        (Node::Operation { .. }, Node::Operation { .. }) => {
                            left.resolve();
                            right.resolve();
                        }
                        problem => unreachable!("This is unaccounted for: {problem:?}"),
                    },
                    Node::Value(_) => unreachable!(
                        "The unresolved arm of the equation cannot have a value as root"
                    ),
                    Node::Equals { .. } => unreachable!(
                        "The unresolved arm of the equation doesn't have an equals as root anymore"
                    ),
                    Node::Human => {
                        unreachable!("The unresolved arm does not have a singular human as root")
                    }
                }
            }
        }
        _ => unreachable!("root is an Equals"),
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Operation {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, c) = one_of("+-*/")(input)?;
        let op = match c {
            '+' => Self::Plus,
            '-' => Self::Minus,
            '*' => Self::Multiply,
            '/' => Self::Divide,
            x => unreachable!("Unexpected character for operation: {x}"),
        };
        Ok((input, op))
    }

    fn apply(&self, left: isize, right: isize) -> isize {
        match self {
            Operation::Plus => left + right,
            Operation::Minus => left - right,
            Operation::Multiply => left * right,
            Operation::Divide => left / right,
        }
    }
}

#[derive(Debug)]
enum Monkey<'a> {
    Value(isize),
    Operation {
        left: &'a str,
        right: &'a str,
        op: Operation,
    },
}

impl<'a> Monkey<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        if let Ok((input, value)) = nomu64::<_, ()>(input) {
            return Ok((input, Self::Value(value as isize)));
        }
        let (input, left) = terminated(alpha1, space1)(input)?;
        let (input, op) = terminated(Operation::parse, space1)(input)?;
        let (input, right) = alpha1(input)?;
        Ok((input, Self::Operation { left, right, op }))
    }

    fn set(&mut self, value: isize) {
        *self = Self::Value(value);
    }
}

#[derive(Debug, Clone)]
enum Node {
    Value(isize),
    Operation {
        operation: Operation,
        left: Box<Node>,
        right: Box<Node>,
    },
    Equals {
        left: Box<Node>,
        right: Box<Node>,
    },
    Human,
}

impl Node {
    fn from_monkeys(name: &str, monkeys: &HashMap<&str, Monkey>) -> Self {
        if name == "humn" {
            return Self::Human;
        }
        match (name, monkeys.get(name)) {
            ("humn", Some(Monkey::Value(_))) => Self::Human,
            ("root", Some(Monkey::Operation { left, right, op: _ })) => Self::Equals {
                left: Box::new(Self::from_monkeys(left, monkeys)),
                right: Box::new(Self::from_monkeys(right, monkeys)),
            },
            (_, Some(Monkey::Value(x))) => Self::Value(*x),
            (_, Some(Monkey::Operation { left, right, op })) => Self::Operation {
                operation: *op,
                left: Box::new(Self::from_monkeys(left, monkeys)),
                right: Box::new(Self::from_monkeys(right, monkeys)),
            },
            (name, None) => unreachable!("Invalid monkey: {name}"),
        }
    }

    fn resolve(&mut self) -> bool {
        match self {
            Node::Value(_) => true,
            Node::Operation {
                operation,
                left,
                right,
            } => match (left.as_mut(), right.as_mut()) {
                (&mut Node::Value(a), &mut Node::Value(b)) => {
                    *self = Self::Value(operation.apply(a, b));
                    true
                }
                (a, b) => {
                    if a.resolve() && b.resolve() {
                        if let (&mut Self::Value(one), &mut Self::Value(two)) = (a, b) {
                            *self = Self::Value(operation.apply(one, two));
                            true
                        } else {
                            unreachable!("Resolve only returns true when it's a value!")
                        }
                    } else {
                        false
                    }
                }
            },
            Node::Equals { .. } => false,
            Node::Human => false,
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<(&str, Monkey)>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, (&str, Monkey)> {
    let (input, name) = terminated(alpha1, tag(": "))(input)?;
    let (input, monkey) = Monkey::parse(input)?;
    Ok((input, (name, monkey)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part1(&input);
        assert_eq!(result, "152");
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./example.txt").unwrap();
        let result = process_part2(&input);
        assert_eq!(result, "301");
    }
}
