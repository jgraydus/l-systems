mod examples;
mod l_system;
mod parser;
mod turtle;
mod util;

use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;
use web_sys::{
    KeyboardEvent,
    WheelEvent,
};

use parser::parse;
use turtle::*;
use util::*;

#[wasm_bindgen]
struct State {
    program: Option<String>,
    iterations: u32,
    viewport: Viewport,
}

#[wasm_bindgen]
pub struct Controller {
    state: Rc<RefCell<State>>,
}

#[wasm_bindgen]
pub fn init() -> Controller {
    let context = get_context2d();
    let canvas = context.canvas().expect("canvas missing!");
    canvas.set_width(canvas.client_width() as u32);
    canvas.set_height(canvas.client_height() as u32);
    let (width, height) = (canvas.width() as f64, canvas.height() as f64);
    let ratio = height / width;
    let base = 1000.0;
    let viewport = Viewport { x0: -base, x1: base, y0: -base * ratio, y1: base * ratio };

    let state = Rc::new(RefCell::new(State {
        program: None,
        iterations: 10,
        viewport: viewport,
    }));

    let handle_wheel = {
        let state = state.clone();
        Closure::<dyn FnMut(WheelEvent)>::new(move |evt: WheelEvent| {
            let canvas = get_context2d().canvas().expect("canvas missing!");
            let (w, h) = (canvas.client_width() as f64, canvas.client_height() as f64);
            let ratio = h / w;

            let Viewport { x0, x1, y0, y1 } = state.borrow().viewport.clone();

            let center_x = (x0 + x1) / 2.0; 
            let center_y = (y0 + y1) / 2.0; 

            let zoom = if evt.delta_y() > 0.0 { 1.1 } else { 0.9 };

            let half_width = (x1 - x0) / 2.0 * zoom;
            let half_height = half_width * ratio;

            let viewport = Viewport { x0: center_x - half_width,
                                      x1: center_x + half_width,
                                      y0: center_y - half_height,
                                      y1: center_y + half_height };

            state.borrow_mut().viewport = viewport; 
            state.borrow().draw();
        })
    };
    let window = web_sys::window().expect("no window?!");
    window.set_onwheel(Some(handle_wheel.as_ref().unchecked_ref()));
    handle_wheel.forget();

    let handle_keypress = {
        let state = state.clone();
        Closure::<dyn FnMut(KeyboardEvent)>::new(move |evt: KeyboardEvent| {
            if evt.shift_key() {
                match evt.key().as_ref() {
                    "ArrowLeft" => {
                        let iterations = state.borrow().iterations;
                        state.borrow_mut().iterations = if iterations > 1 { iterations - 1 } else { 1 };
                        state.borrow().draw();
                    },
                    "ArrowRight" => {
                        let iterations = state.borrow().iterations;
                        state.borrow_mut().iterations = iterations + 1;
                        state.borrow().draw();
                    },
                    _ => {}
                }
            }
        })
    };
    window.set_onkeydown(Some(handle_keypress.as_ref().unchecked_ref()));
    handle_keypress.forget();

    Controller {
        state: state.clone()
    }
}

impl State {
    pub fn draw(&self) {
        let program = self.program.clone();
        let iterations = self.iterations;
        let viewport = self.viewport.clone();

        if let Some(input) = program {
            wasm_bindgen_futures::spawn_local(async move {
                match parse(&input) {
                    Ok(lsystem) => {
                        let program = lsystem.compile(iterations);
                        let context = get_context2d();
                        clear_canvas(&context);
                        program.execute(&context, viewport);
                    }
                    Err(err) => {
                        web_sys::console::log_1(&err.into());
                    }
                }
            });
        }
    }
}

#[wasm_bindgen]
impl Controller {
    pub fn set_program(&self, program: String) {
        self.state.borrow_mut().program = Some(program);
    }

    pub fn set_iterations(&self, iterations: u32) {
        self.state.borrow_mut().iterations = iterations;
    }

    pub fn set_viewport(&self, viewport: Viewport) {
        self.state.borrow_mut().viewport = viewport;
    }

    pub fn draw(&self) {
        self.state.borrow().draw();
    }
}

#[cfg(test)]
mod tests {
    use super::turtle::*;

    #[test]
    fn test_01() {
        let mut program = TurtleProgram {
            turtle: Turtle {
                location: (0.0, 0.0),
                orientation: 0.0,
                pen: Pen {
                    color: (1.0, 1.0, 1.0),
                    width: 3.0,
                    state: PenState::Down,
                }
            },
            commands: vec![
                TurtleCommand::Turn(45.0_f64.to_radians()),
                TurtleCommand::Move(100.0),
                TurtleCommand::Turn(45.0_f64.to_radians()),
                TurtleCommand::Move(100.0),
            ],
        };

        let result = program.run();
        println!("{:?}", result);
    }
}

