#![allow(unused)]
use futures::{
    future::{self},
    stream::{self, Stream, StreamExt},
};
use std::collections::HashMap;
use std::pin::Pin;
use std::rc::{Rc};

use crate::turtle::*;

type CharStream = Pin<Box<dyn Stream<Item=char>>>;
type TurtleCommandStream = Pin<Box<dyn Stream<Item=TurtleCommand>>>;

fn string_to_stream(s: &String) -> CharStream {
    let v: Vec<char> = s.chars().collect();
    Box::pin(stream::iter(v))
}

#[derive(Clone, Debug, PartialEq)]
struct Rules {
    inner: HashMap<char, String>
}

impl Rules {
    fn from(inner: HashMap<char, String>) -> Self {
        Self { inner }
    }

    fn get_as_string(&self, c: char) -> String {
        self.inner.get(&c)
            .map(|s| s.clone())
            .unwrap_or(format!("{}", c))
    }

    fn get_as_stream(&self, c: char) -> CharStream { 
        self.inner.get(&c)
            .map(|s| string_to_stream(s))
            .unwrap_or(Box::pin(stream::once(future::ready(c))))
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Interpreter {
    inner: HashMap<char, Vec<TurtleCommand>>,
}

impl Interpreter {
    fn from(inner: HashMap<char, Vec<TurtleCommand>>) -> Self {
        Self { inner }
    }

    fn get(&self, c: &char) -> Option<&Vec<TurtleCommand>> {
        self.inner.get(c)
    }

    fn get_as_stream(&self, c: &char) -> TurtleCommandStream {
        if let Some(r) = self.inner.get(&c) {
            Box::pin(stream::iter(r.clone()))
        } else {
            Box::pin(stream::empty())
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LSystem {
    start: String,
    rules: Rc<Rules>, 
    interpreter: Rc<Interpreter>,
}

impl LSystem {
    pub fn new(start: &str,
               rules: HashMap<char, String>,
               interpreter: HashMap<char, Vec<TurtleCommand>>) -> Self {
        Self {
            start: start.to_owned(),
            rules: Rc::new(Rules::from(rules)),
            interpreter: Rc::new(Interpreter::from(interpreter)),
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
            result.push_str(&self.rules.get_as_string(c))
        }
        result
    }

    pub fn expand_stream(&self, iterations: u32) -> CharStream {
        fn aux(rules: Rc<Rules>,
               input: CharStream,
               iterations: u32) -> CharStream {
            if iterations <= 1 {
                input
            } else {
                Box::pin(input.flat_map(move |c| {
                    let s = rules.get_as_stream(c);
                    aux(rules.clone(), s, iterations - 1)
                }))
            }
        }
        aux(self.rules.clone(), string_to_stream(&self.start), iterations)
    }

    pub fn compile(&self, iterations: u32) -> TurtleProgram {
        let mut commands = Vec::new();
        let s = self.expand(iterations);
        for c in s.chars() {
            if let Some(r) = self.interpreter.get(&c) {
                commands.append(&mut r.clone());
            }
        }

        TurtleProgram::new(
            Turtle {
                location: (0.0, 0.0),
                orientation: 0.0,
                pen: Pen {
                    color: (1.0, 1.0, 1.0),
                    width: 2.0,
                    state: PenState::Down
                },
            },
            commands,
        )
    }


    pub fn compile_stream(&self, iterations: u32) -> TurtleProgram {
        let interpreter = self.interpreter.clone();

        let commands = Box::pin(self.expand_stream(iterations).flat_map(move |c| {
            interpreter.get_as_stream(&c)
        }));

        TurtleProgram::new_async(
            Turtle {
                location: (0.0, 0.0),
                orientation: 0.0,
                pen: Pen {
                    color: (1.0, 1.0, 1.0),
                    width: 2.0,
                    state: PenState::Down
                },
            },
            commands
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use wasm_bindgen_test::*;
    use wasm_bindgen_futures;

    #[wasm_bindgen_test]
    async fn expand_stream_1() {
        let system = LSystem::new("A", HashMap::from([('A', "AB".into())]), HashMap::new());
        let s = system.expand_stream(1);
        let v: Vec<char> = s.collect().await;
        assert_eq!(v, vec!['A'])
    }

    #[wasm_bindgen_test]
    async fn expand_stream_2() {
        let system = LSystem::new("A", HashMap::from([('A', "AB".into())]), HashMap::new());
        let s = system.expand_stream(2);
        let v: Vec<char> = s.collect().await;
        assert_eq!(v, vec!['A', 'B'])
    }

    #[wasm_bindgen_test]
    async fn expand_stream_3() {
        let system = LSystem::new(
            "AA",
            HashMap::from([('A', "BB".into()), ('B', "A".into())]),
            HashMap::new()
        );
        let s = system.expand_stream(3);
        let v: Vec<char> = s.collect().await;
        assert_eq!(v, vec!['A', 'A', 'A', 'A'])
    }
}
