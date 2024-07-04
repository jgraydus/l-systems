use web_sys;
use wasm_bindgen::prelude::*;

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
}

#[derive(Clone, Debug)]
pub struct Turtle {
    pub location: (f64, f64),
    pub orientation: f64,
    pub pen: Pen
}

#[derive(Clone, Debug)]
pub enum TurtleCommand {
    Move(f64),
    Turn(f64),
    PenDown,
    PenUp,
    Repeat(u32, Vec<TurtleCommand>),
    Push,
    Pop,
}

#[derive(Clone, Debug)]
pub enum DrawCommand {
    BeginPath,
    SetLineWidth(f64),
    SetStrokeStyle(String),
    MoveTo(f64, f64),
    LineTo(f64, f64),
    Stroke,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Viewport {
    pub x0: f64, pub x1: f64, pub y0: f64, pub y1: f64
}

impl DrawCommand {
    fn exec(&self,
            context: &web_sys::CanvasRenderingContext2d,
            delta_x: f64,
            delta_y: f64,
            alpha: f64,
            beta: f64) {
        match self {
            DrawCommand::BeginPath => {
                context.begin_path();
            }
            DrawCommand::SetLineWidth(lw) => {
                context.set_line_width(*lw);
            }
            DrawCommand::SetStrokeStyle(s) => {
                context.set_stroke_style(&JsValue::from_str(&s));
            }
            DrawCommand::MoveTo(x, y) => {
                let x = (*x - delta_x) * alpha;
                let y = (*y - delta_y) * beta;
                context.move_to(x, y);
            }
            DrawCommand::LineTo(x, y) => {
                let x = (*x - delta_x) * alpha;
                let y = (*y - delta_y) * beta;
                context.line_to(x, y);
            }
            DrawCommand::Stroke => {
                context.stroke();
            }
        }
    }

    fn exec_all(commands: &Vec<DrawCommand>,
                context: &web_sys::CanvasRenderingContext2d,
                viewport: Viewport) {
        let canvas = context.canvas().expect("canvas missing!");
        let (width, height) = (canvas.width() as f64, canvas.height() as f64);
        let Viewport { x0, x1, y0, y1 } = viewport;
        let alpha = width / (x1 - x0);
        let beta = height / (y0 - y1);

        for command in commands.iter() {
            command.exec(context, x0, y1, alpha, beta);
        }
    }
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
                    let t = stack.pop().expect("cannot pop empty stack");
                    self.location = t.location;
                    let (x, y) = self.location;
                    result.push(DrawCommand::MoveTo(x, y));
                    self.orientation = t.orientation;
                    self.pen = t.pen;
                    result.append(&mut self.pen.run());
                },
            }
        }

        result
    }
}

pub struct TurtleProgram {
    pub turtle: Turtle,
    pub commands: Vec<TurtleCommand>,
}

impl TurtleProgram {
    pub fn execute(mut self,
                   context: &web_sys::CanvasRenderingContext2d,
                   viewport: Viewport) {
        let commands = self.run();
        DrawCommand::exec_all(&commands, context, viewport);
    }

    pub fn run(&mut self) -> Vec<DrawCommand> {
        let mut result = Vec::new();
        let mut stack = Vec::new();

        // start stuff
        result.push(DrawCommand::BeginPath);
        result.append(&mut self.turtle.pen.run());
        let (x, y) = self.turtle.location;
        result.push(DrawCommand::MoveTo(x, y));

        // middle stuff
        result.append(&mut self.turtle.run(&self.commands, &mut stack));

        // end stuff
        result.push(DrawCommand::Stroke);

        result
    }
}
