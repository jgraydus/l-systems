use crate::l_system::*;
use crate::turtle::*;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Parser)]
#[grammar_inline = r##"
WHITESPACE = _{ " " | "\t" | NEWLINE }

number = { "-"? ~ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }

special_char = { "!" | "@" | "#" | "$" | "%" | "^" | "&" | "*" | "-" | "=" | "+" | "_" | "~"}
puncuation_char = { "." | ";" | ":" | "'" | "`" }
bracket_char = { "[" | "]" | "{" | "}" | "<" | ">" }
valid_char = { ASCII_ALPHANUMERIC | special_char | puncuation_char | bracket_char }

lsystem_start_value = { valid_char+ }

lsystem_rule_rhs = { valid_char+ }
lsystem_rule = { valid_char ~ "->" ~ lsystem_rule_rhs }
lsystem_rules = { "(" ~ lsystem_rule ~ ("," ~ lsystem_rule)* ~ ")" }

turtle_command_move = { "MOVE" ~ number }
turtle_command_turn = { "TURN" ~ number }
turtle_command_push = { "PUSH" }
turtle_command_pop = { "POP" }
turtle_command_pen_up = { "PEN" ~ "UP" }
turtle_command_pen_down = { "PEN" ~ "DOWN" }

turtle_command = { turtle_command_move
                 | turtle_command_turn
                 | turtle_command_push
                 | turtle_command_pop
                 | turtle_command_pen_up
                 | turtle_command_pen_down
                 }
turtle_commands = { turtle_command ~ ("," ~ turtle_command)* }
turtle_program = { "(" ~ turtle_commands? ~ ")" }

lsystem_interpreter_rule = { valid_char ~ "->" ~ turtle_program }
lsystem_interpreter_rules = { lsystem_interpreter_rule ~ ("," ~ lsystem_interpreter_rule)* }
lsystem_interpreter = { "(" ~ lsystem_interpreter_rules? ~ ")" }

lsystem = { SOI ~ "LSYSTEM" ~ "("
          ~ lsystem_start_value ~ ","
          ~ lsystem_rules ~ ","
          ~ lsystem_interpreter
          ~ ")" ~ EOI
          }
"##]
pub struct LSystemParser;

fn to_start_value(pair: Pair<Rule>) -> &str {
    pair.as_str()
}

fn to_rule(pair: Pair<Rule>) -> (char, String) {
    let mut items = pair.into_inner();
    let k = items.next().unwrap().as_str().chars().nth(0).unwrap();
    let v = items.next().unwrap().as_str().to_owned();
    (k, v)
}

fn to_rules(pair: Pair<Rule>) -> HashMap<char, String> {
    let mut result = HashMap::new();
    for item in pair.into_inner() {
        let (k, v) = to_rule(item);
        result.insert(k, v);
    }
    result
}

fn to_f64(pair: Pair<Rule>) -> f64 {
    f64::from_str(pair.as_str()).expect("failed to parse f64")
}

fn to_turtle_command(pair: Pair<Rule>) -> TurtleCommand {
    let item = pair.into_inner().next().unwrap();
    match item.as_rule() {
        Rule::turtle_command_move => {
            let v = to_f64(item.into_inner().next().unwrap());
            TurtleCommand::Move(v)
        }
        Rule::turtle_command_turn => {
            let v = to_f64(item.into_inner().next().unwrap());
            TurtleCommand::Turn(v)
        }
        Rule::turtle_command_push => TurtleCommand::Push,
        Rule::turtle_command_pop => TurtleCommand::Pop,
        Rule::turtle_command_pen_up => TurtleCommand::PenUp,
        Rule::turtle_command_pen_down => TurtleCommand::PenDown,
        _ => panic!("failed to match turtle command rule")
    }
}

fn to_turtle_program(pair: Pair<Rule>) -> Vec<TurtleCommand> {
    let mut result = Vec::new();
    for item in pair.into_inner().next().unwrap().into_inner() {
        result.push(to_turtle_command(item));
    }
    result
}

fn to_interpreter_rule(pair: Pair<Rule>) -> (char, Vec<TurtleCommand>) {
    let mut items = pair.into_inner();
    let k = items.next().unwrap().as_str().chars().nth(0).unwrap();
    let v = to_turtle_program(items.next().unwrap());
    (k, v)
}

fn to_interpreter(pair: Pair<Rule>) -> HashMap<char, Vec<TurtleCommand>> {
    let mut result = HashMap::new();
    for item in pair.into_inner().next().unwrap().into_inner() {
        let (k, v) = to_interpreter_rule(item);
        result.insert(k, v);
    }
    result
}

pub fn parse(input: &str) -> Result<LSystem, String> {
    match LSystemParser::parse(Rule::lsystem, input) {
        Ok(mut result) => {
            let mut pair = result.next().unwrap().into_inner();
            Ok(
                LSystem::new(
                    to_start_value(pair.next().unwrap()),
                    to_rules(pair.next().unwrap()),
                    to_interpreter(pair.next().unwrap())
                )
            )
        }
        Err(err) => {
            Err(format!("parse error at: {:?}", err.line_col))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::*;

    macro_rules! generate_test {
        ($function_name:ident, $source:expr, $lsystem: expr) => {
            #[test]
            fn $function_name() {
                let actual = parse($source).unwrap();
                let expected = $lsystem();
                assert_eq!(actual, expected);
            }
        }
    }

    // TODO proc macro for this?
    generate_test!(algae, ALGAE.1, ALGAE.2);
    generate_test!(koch, KOCH.1, KOCH.2);
    generate_test!(sierpinski, SIERPINSKI.1, SIERPINSKI.2);
    generate_test!(tree, TREE.1, TREE.2);
    generate_test!(dragon, DRAGON.1, DRAGON.2);
    generate_test!(levy, LEVY.1, LEVY.2);
}

