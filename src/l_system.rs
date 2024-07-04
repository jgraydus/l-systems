use std::collections::HashMap;

use crate::turtle::*;

#[derive(Debug, PartialEq)]
pub struct LSystem {
    start: String,
    rules: HashMap<char, String>,
    interpreter: HashMap<char, Vec<TurtleCommand>>,
}

impl LSystem {
    pub fn new(start: &str,
               rules: HashMap<char, String>,
               interpreter: HashMap<char, Vec<TurtleCommand>>) -> Self {
        Self {
            start: start.to_owned(), rules, interpreter
        }
    }

    pub fn expand(&self, iterations: u32) -> String {
        let mut s = self.start.clone();
        for _ in 1..iterations {
            s = self.apply_rules(s);
        }
        s
    }

    fn apply_rules(&self, s: String) -> String {
        let mut result = String::new();

        for c in s.chars() {
            if let Some(rule) = self.rules.get(&c) {
                result.push_str(rule);
            } else {
                // if there's no rule for c, it's a constant
                result.push(c);
            }
        }

        result
    }

    pub fn compile(&self, iterations: u32) -> TurtleProgram {
        let mut result = Vec::new();
        let s = self.expand(iterations);
        for c in s.chars() {
            if let Some(r) = self.interpreter.get(&c) {
                result.append(&mut r.clone());
            }
        }

        TurtleProgram {
            turtle: Turtle {
                location: (0.0, 0.0),
                orientation: 0.0,
                pen: Pen {
                    color: (1.0, 1.0, 1.0),
                    width: 2.0,
                    state: PenState::Down
                },
            },
            commands: result,
        }
    }
}

