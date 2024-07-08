use wasm_bindgen_futures;
use web_sys;
use futures::{
    future::{self},
    stream::{self, Stream, StreamExt},
};
use std::{
    cell::RefCell,
    fmt,
    pin::Pin,
    rc::Rc,
};

use crate::draw::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PenState { Down, Up }

#[derive(Clone, Debug)]
pub struct Pen {
    pub color: (f64, f64, f64),
    pub width: f64,
    pub state: PenState
}

impl Pen {
    fn run(&self) -> Vec<DrawCommand> {
        let mut result = Vec::new();
        result.push(DrawCommand::SetLineWidth(self.width));
        result.push(DrawCommand::SetStrokeStyle("red".to_string()));//TODO
        result
    }

    fn run_as_stream(&self) -> Pin<Box<dyn Stream<Item=DrawCommand>>> {
        stream::iter(self.run()).boxed_local()
    }
}

#[derive(Clone, Debug)]
pub struct Turtle {
    pub location: (f64, f64),
    pub orientation: f64,
    pub pen: Pen
}

#[derive(Clone, Debug, PartialEq)]
pub enum TurtleCommand {
    Move(f64),
    Turn(f64),
    PenDown,
    PenUp,
    Repeat(u32, Vec<TurtleCommand>),
    Push,
    Pop,
}

impl Turtle {
    pub fn run(&mut self, commands: &Vec<TurtleCommand>, stack: &mut Vec<Turtle>) -> Vec<DrawCommand> {
        let mut result = Vec::new();

        for command in commands.iter() {
            match command {
                TurtleCommand::Move(distance) => {
                    let angle = self.orientation;
                    let (x, y) = self.location;
                    let x = x + distance * angle.to_radians().cos();
                    let y = y + distance * angle.to_radians().sin();
                    self.location = (x, y);
                    if self.pen.state == PenState::Down {
                        result.push(DrawCommand::LineTo(x, y));
                    } else {
                        result.push(DrawCommand::MoveTo(x, y));
                    }
                },
                TurtleCommand::Turn(angle) => {
                    self.orientation += angle;
                },
                TurtleCommand::PenDown => {
                    self.pen.state = PenState::Down;
                }
                TurtleCommand::PenUp => {
                    self.pen.state = PenState::Up;
                }
                TurtleCommand::Repeat(n, cs) => {
                    for _ in 0..*n {
                        result.append(&mut self.run(cs, stack));
                    }
                }
                TurtleCommand::Push => {
                    stack.push(self.clone());
                },
                TurtleCommand::Pop => {
                    if let Some(t) = stack.pop() {
                        self.location = t.location;
                        let (x, y) = self.location;
                        result.push(DrawCommand::MoveTo(x, y));
                        self.orientation = t.orientation;
                        self.pen = t.pen;
                        result.append(&mut self.pen.run());
                    } else {
                        web_sys::console::log_1(&"cannot pop an empty stack".into());
                    }
                },
            }
        }

        result
    }
}

enum Commands {
    Vec(Vec<TurtleCommand>),
    Stream(Pin<Box<dyn Stream<Item=TurtleCommand>>>),
}

impl fmt::Debug for Commands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Commands::Vec(v) => write!(f, "{:?}", v),
            Commands::Stream(_) => write!(f, "<stream>"),
        }
    }
}

#[derive(Debug)]
pub struct TurtleProgram {
    turtle: Turtle,
    commands: Commands,
}

impl TurtleProgram {
    pub fn new(turtle: Turtle, commands: Vec<TurtleCommand>) -> Self {
        Self { turtle, commands: Commands::Vec(commands) }
    }

    pub fn new_async(turtle: Turtle, commands: Pin<Box<dyn Stream<Item=TurtleCommand>>>) -> Self {
        Self { turtle, commands: Commands::Stream(commands) }
    }

    pub fn execute(mut self,
                   context: web_sys::CanvasRenderingContext2d,
                   viewport: Viewport) {
        match self.commands {
            Commands::Vec(commands) => {
                let mut draw_commands = Vec::new();
                let mut stack = Vec::new();
                // start stuff
                draw_commands.push(DrawCommand::BeginPath);
                draw_commands.append(&mut self.turtle.pen.run());
                let (x, y) = self.turtle.location;
                draw_commands.push(DrawCommand::MoveTo(x, y));
                // middle stuff
                draw_commands.append(&mut self.turtle.run(&commands, &mut stack));
                // end stuff
                draw_commands.push(DrawCommand::Stroke);
                // do it!
                DrawCommand::exec_all(&draw_commands, &context, viewport);
            }
            Commands::Stream(commands) => {
                wasm_bindgen_futures::spawn_local(async move {
                    let turtle = Rc::new(RefCell::new(self.turtle.clone()));
                    let stack = Rc::new(RefCell::new(Vec::<Turtle>::new()));
                    let mut draw_commands: Pin<Box<dyn Stream<Item=DrawCommand>>> =
                        stream::iter(vec![
                            //start stuff
                            stream::once(future::ready(DrawCommand::BeginPath)).boxed_local(),
                            turtle.clone().borrow().pen.run_as_stream(),
                            stream::once(future::ready(DrawCommand::BeginPath)).boxed_local(),
                            // middle stuff
                            commands.flat_map(move |command|
                                run_command(turtle.clone(), &command, stack.clone())
                            ).boxed_local(),
                            // end stuff
                            stream::once(future::ready(DrawCommand::Stroke)).boxed_local(),
                        ]).flatten().boxed_local();
                    // do it!
                    DrawCommand::exec_stream(&mut draw_commands, &context, viewport).await;

                });
            }
        }
    }
}

pub fn run_command(turtle: Rc<RefCell<Turtle>>,
                   command: &TurtleCommand,
                   stack: Rc<RefCell<Vec<Turtle>>>) -> Pin<Box<dyn Stream<Item=DrawCommand>>> {
    use TurtleCommand::*;

    match command {
        Move(distance) => {
            let (angle, (x, y)) = (turtle.borrow().orientation, turtle.borrow().location);
            let x = x + distance * angle.to_radians().cos();
            let y = y + distance * angle.to_radians().sin();
            turtle.borrow_mut().location = (x, y);
            if turtle.borrow().pen.state == PenState::Down {
                return stream::once(future::ready(DrawCommand::LineTo(x,y))).boxed_local();
            } else {
                return stream::once(future::ready(DrawCommand::MoveTo(x,y))).boxed_local();
            }
        }
        Turn(angle) => {
            turtle.borrow_mut().orientation += angle;
            return stream::empty().boxed_local();
        }
        PenDown => {
            turtle.borrow_mut().pen.state = PenState::Down;
            return stream::empty().boxed_local();
        }
        PenUp => {
            turtle.borrow_mut().pen.state = PenState::Up;
            return stream::empty().boxed_local();
        }
        Repeat(n, cs) => {
            let n = *n;
            let cs = cs.clone();
            return stream::iter(0..n).flat_map(move |_| {
                let turtle = turtle.clone();
                let stack = stack.clone();
                stream::iter(cs.clone()).flat_map(move |c| {
                    run_command(turtle.clone(), &c, stack.clone())
                }).boxed_local()
            }).boxed_local();
        }
        Push => {
            stack.borrow_mut().push(turtle.borrow().clone());
            return stream::empty().boxed_local();
        },
        Pop => {
            if let Some(t) = stack.borrow_mut().pop() {
                let (x, y) = t.location;
                *turtle.borrow_mut() = t;
                let mut v = turtle.borrow().pen.run();
                v.push(DrawCommand::MoveTo(x, y));
                return stream::iter(v).boxed_local();
            } else {
                web_sys::console::log_1(&"cannot pop an empty stack".into());
                return stream::empty().boxed_local();
            }
        },
    }
}

