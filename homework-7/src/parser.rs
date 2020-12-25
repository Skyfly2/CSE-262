// Here is where the various combinators are imported. You can find all the combinators here:
// https://docs.rs/nom/5.0.1/nom/
// If you want to use it in your parser, you need to import it here. I've already imported a couple.

use nom::{
  branch::alt,
  bytes::complete::{tag, take_until},
  character::complete::{alphanumeric1, digit1},
  combinator::opt,
  multi::{many0, many1, separated_list},
  IResult,
};

// Here are the different node types. You will use these to make your parser and your grammar.
// You may add other nodes as you see fit, but these are expected by the runtime.

#[derive(Debug, Clone)]
pub enum Node {
  Program { children: Vec<Node> },
  Statement { children: Vec<Node> },
  FunctionReturn { children: Vec<Node> },
  FunctionDefine { children: Vec<Node> },
  FunctionArguments { children: Vec<Node> },
  FunctionStatements { children: Vec<Node> },
  Expression { children: Vec<Node> },
  MathExpression { name: String, children: Vec<Node> },
  FunctionCall { name: String, children: Vec<Node> },
  VariableDefine { children: Vec<Node> },
  Number { value: i32 },
  Bool { value: bool },
  Identifier { value: String },
  String { value: String },
}

// Define production rules for an identifier
pub fn identifier(input: &str) -> IResult<&str, Node> {
  let (input, result) = alphanumeric1(input)?; // Consume at least 1 alphanumeric character. The ? automatically unwraps the result if it's okay and bails if it is an error.
  Ok((
    input,
    Node::Identifier {
      value: result.to_string(),
    },
  )) // Return the now partially consumed input, as well as a node with the string on it.
}

// Define an integer number
pub fn number(input: &str) -> IResult<&str, Node> {
  let (input, result) = digit1(input)?; // Consume at least 1 digit 0-9
  let number = result.parse::<i32>().unwrap(); // Parse the string result into a usize
  Ok((input, Node::Number { value: number })) // Return the now partially consumed input with a number as well
}

pub fn boolean(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, result) = alt((tag("true"), tag("false")))(input)?;
  let val: bool;
  if result == "true" {
    val = true;
  } else {
    val = false;
  }
  Ok((input, Node::Bool { value: val }))
}

pub fn string(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("\"")(input)?;
  let (input, string_val) = take_until("\"")(input)?;
  let (input, _) = tag("\"")(input)?;
  Ok((
    input,
    Node::String {
      value: String::from(string_val),
    },
  ))
}

pub fn function_call(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, string_node) = identifier(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("(")(input)?;
  let (input, args) = arguments(input)?;
  let (input, _) = tag(")")(input)?;
  let name: String = match string_node {
    Node::Identifier { value } => value,
    _ => String::from("Error"),
  };
  Ok((
    input,
    Node::FunctionCall {
      name: name,
      children: vec![args],
    },
  ))
}

// Math expressions with parens (1 * (2 + 3))
pub fn parenthetical_expression(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("(")(input)?;
  let (input, result) = math_expression(input)?;
  let (input, _) = tag(")")(input)?;
  Ok((
    input,
    Node::Statement {
      children: vec![result],
    },
  ))
}

// Pass math more terms
pub fn l4(input: &str) -> IResult<&str, Node> {
  alt((function_call, number, identifier, parenthetical_expression))(input)
}

// Pass math exponent
pub fn l3_infix(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = tag("^")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = l4(input)?;
  Ok((
    input,
    Node::MathExpression {
      name: op.to_string(),
      children: vec![args],
    },
  ))
}

pub fn l3(input: &str) -> IResult<&str, Node> {
  let (input, mut head) = l4(input)?;
  let (input, tail) = many0(l3_infix)(input)?;
  for n in tail {
    match n {
      Node::MathExpression { name, mut children } => {
        let mut new_children = vec![head.clone()];
        new_children.append(&mut children);
        head = Node::MathExpression {
          name,
          children: new_children,
        };
      }
      _ => (),
    };
  }
  Ok((input, head))
}

// Pass math divide
// Pass math multiply
pub fn l2_infix(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = alt((tag("*"), tag("/")))(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = l3(input)?;
  Ok((
    input,
    Node::MathExpression {
      name: op.to_string(),
      children: vec![args],
    },
  ))
}

pub fn l2(input: &str) -> IResult<&str, Node> {
  let (input, mut head) = l3(input)?;
  let (input, tail) = many0(l2_infix)(input)?;
  for n in tail {
    match n {
      Node::MathExpression { name, mut children } => {
        let mut new_children = vec![head.clone()];
        new_children.append(&mut children);
        head = Node::MathExpression {
          name,
          children: new_children,
        };
      }
      _ => (),
    };
  }
  Ok((input, head))
}

// L1 - L4 handle order of operations for math expressions
// Pass math test
// Pass math no space
// Pass math subtraction
pub fn l1_infix(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = alt((tag("+"), tag("-")))(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = l2(input)?;
  Ok((
    input,
    Node::MathExpression {
      name: op.to_string(),
      children: vec![args],
    },
  ))
}

pub fn l1(input: &str) -> IResult<&str, Node> {
  let (input, mut head) = l2(input)?;
  let (input, tail) = many0(l1_infix)(input)?;
  for n in tail {
    match n {
      Node::MathExpression { name, mut children } => {
        let mut new_children = vec![head.clone()];
        new_children.append(&mut children);
        head = Node::MathExpression {
          name,
          children: new_children,
        };
      }
      _ => (),
    };
  }
  Ok((input, head))
}

pub fn math_expression(input: &str) -> IResult<&str, Node> {
  l1(input)
}

pub fn expression(input: &str) -> IResult<&str, Node> {
  let (input, result) = alt((
    boolean,
    math_expression,
    number,
    function_call,
    identifier,
    string,
    empty_expression,
  ))(input)?;
  // let (input, _) = many0(tag(";"))(input)
  // let (input, _) = tag(";")(input)?;
  Ok((
    input,
    Node::Expression {
      children: vec![result],
    },
  ))
}

pub fn empty_expression(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  Ok((input, Node::Expression { children: vec![] }))
}

// Pass variable_bool test
// Pass variable_init test
// Pass variable_init_no_space
// Pass variable_string test
pub fn statement(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, result) = alt((variable_define, function_return))(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = many0(tag(";"))(input)?;
  Ok((
    input,
    Node::Statement {
      children: vec![result],
    },
  ))
}

pub fn function_return(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("return")(input)?;
  let (input, result) = expression(input)?;
  Ok((
    input,
    Node::FunctionReturn {
      children: vec![result],
    },
  ))
}

// Define a statement of the form
// let x = expression
pub fn variable_define(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("let ")(input)?;
  let (input, variable) = identifier(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("=")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, expression) = expression(input)?;
  Ok((
    input,
    Node::VariableDefine {
      children: vec![variable, expression],
    },
  ))
}

// Pass function call
// Pass function call one arg
pub fn arguments(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, children) = expression(input)?;
  let (input, other_children) = many0(other_arg)(input)?;
  Ok((
    input,
    Node::FunctionArguments {
      children: vec![children],
    },
  ))
}

// Like the first argument but with a comma in front
pub fn other_arg(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag(",")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, children) = expression(input)?;
  let (input, other_children) = other_arg(input)?;
  Ok((
    input,
    Node::FunctionArguments {
      children: vec![children, other_children],
    },
  ))
}

pub fn function_definition(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(alt((tag(" "), tag("\n"))))(input)?;
  let (input, _) = tag("fn")(input)?;
  let (input, _) = many0(alt((tag(" "), tag("\n"))))(input)?;
  let (input, name) = identifier(input)?;
  let (input, _) = many0(alt((tag(" "), tag("\n"))))(input)?;
  let (input, _) = tag("(")(input)?;
  let (input, _) = many0(alt((tag(" "), tag("\n"))))(input)?;
  let (input, arguments) = arguments(input)?;
  let (input, _) = many0(alt((tag(" "), tag("\n"))))(input)?;
  let (input, _) = tag(")")(input)?;
  let (input, _) = many0(alt((tag(" "), tag("\n"))))(input)?;
  let (input, _) = tag("{")(input)?;
  let (input, _) = many0(alt((tag(" "), tag("\n"))))(input)?;
  let (input, children) = many1(alt((statement, expression)))(input)?;
  let (input, _) = many0(alt((tag(" "), tag("\n"))))(input)?;
  let (input, _) = tag("}")(input)?;
  Ok((
    input,
    Node::FunctionDefine {
      children: vec![
        name,
        Node::FunctionArguments {
          children: vec![arguments],
        },
        Node::FunctionStatements { children: children },
      ],
    },
  ))
}

pub fn comment(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("//")(input)?;
  let (input, result) = string(input)?;
  Ok((input, result))
}

// Define a program. You will change this, this is just here for example.
// You'll probably want to modify this by changing it to be that a program
// is defined as at least one function definition, but maybe more. Start
// by looking up the many1() combinator and that should get you started.
pub fn program(input: &str) -> IResult<&str, Node> {
  let (input, result) = alt((function_definition, statement, expression))(input)?; // Now that we've defined a number and an identifier, we can compose them using more combinators. Here we use the "alt" combinator to propose a choice.
                                                                                   // This should be let (input, result) = many1(alt((function_call, statement, expression)))?; however, for some reason using the many1 function results in the entire input being skipped
  Ok((
    input,
    Node::Program {
      children: vec![result],
    },
  )) // Whether the result is an identifier or a number, we attach that to the program
}
