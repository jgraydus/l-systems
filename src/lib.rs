mod draw;
mod examples;
mod l_system;
mod parser;
mod turtle;
mod util;

use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{Event};

use draw::*;
use examples::all_examples;
use parser::parse;
use util::*;

#[wasm_bindgen]
pub fn examples() -> js_sys::Map {
    let result = js_sys::Map::new();
    for (name, source, _) in all_examples() {
        result.set(&name.into(), &source.into());
    }
    result
}

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

    let handle_resize = {
        let state = state.clone();
        Closure::<dyn FnMut(Event)>::new(move |_: Event| {
            let canvas = get_context2d().canvas().expect("canvas missing!");
            canvas.set_width(canvas.client_width() as u32);
            canvas.set_height(canvas.client_height() as u32);
            let (width, height) = (canvas.width() as f64, canvas.height() as f64);
            let ratio = height / width;
            let Viewport { x0, x1, .. } = state.borrow().viewport.clone();
            let viewport = Viewport { x0, x1, y0: x0 * ratio, y1: x1 * ratio };
            state.borrow_mut().viewport = viewport;
            let _ = state.borrow().draw();
        })
    };
    let window = web_sys::window().expect("no window?!");
    window.set_onresize(Some(handle_resize.as_ref().unchecked_ref()));
    handle_resize.forget();

    Controller {
        state: state.clone()
    }
}

impl State {
    fn draw(&self) -> Result<(), JsValue> {
        let program = self.program.clone();
        let iterations = self.iterations;
        let viewport = self.viewport.clone();

        if let Some(input) = program {
            match parse(&input) {
                Ok(lsystem) => {
                    let program =
                        if false { lsystem.compile_stream(iterations) }
                        else { lsystem.compile(iterations) };
                    let context = get_context2d();
                    clear_canvas(&context);
                    program.execute(context, viewport);
                    return Ok(());
                }
                Err(err) => {
                    return Err(err.into());
                }
            }
        }
        Err("program is not set".into())
    }

    fn zoom(&mut self, multiplier: f64) {
        let canvas = get_context2d().canvas().expect("canvas missing!");
        let (w, h) = (canvas.client_width() as f64, canvas.client_height() as f64);
        let ratio = h / w;

        let Viewport { x0, x1, y0, y1 } = self.viewport.clone();

        let center_x = (x0 + x1) / 2.0;
        let center_y = (y0 + y1) / 2.0;

        let half_width = (x1 - x0) / 2.0 * multiplier;
        let half_height = half_width * ratio;

        let viewport = Viewport { x0: center_x - half_width,
                                  x1: center_x + half_width,
                                  y0: center_y - half_height,
                                  y1: center_y + half_height };

        self.viewport = viewport;
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

    pub fn zoom(&self, multiplier: f64) {
        self.state.borrow_mut().zoom(multiplier);
    }

    pub fn draw(&self) -> Result<(), JsValue> {
        self.state.borrow().draw()
    }
}

