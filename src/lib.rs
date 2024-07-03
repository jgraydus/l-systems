mod examples;
mod handlers;
mod l_system;
mod turtle;
mod util;

use gloo_timers::future::TimeoutFuture;
use std::collections::HashMap;
use web_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;

use handlers::*;
use l_system::*;
use turtle::*;
use util::*;

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async move {
        set_up_mouse_handlers();

        let context = get_context2d();

        let viewport = Viewport { x0: -1000.0, x1: 1000.0, y0: -1000.0, y1: 1000.0 };

        examples::levy().compile().execute(&context, viewport);
        TimeoutFuture::new(1000).await;
        clear_canvas(&context);
        TimeoutFuture::new(1000).await;
        examples::levy().compile().execute(&context, viewport);
    });

    Ok(())
}

#[wasm_bindgen]
pub fn draw() -> Result<(), JsValue> {
    wasm_bindgen_futures::spawn_local(async move {
        let context = get_context2d();
        let canvas = context.canvas().expect("canvas missing!");
        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);
        let (width, height) = (canvas.width() as f64, canvas.height() as f64);
        let ratio = height / width;
        let viewport = Viewport { x0: -2000.0, x1: 2000.0, y0: -2000.0 * ratio, y1: 2000.0 * ratio };
        clear_canvas(&context);
        //TimeoutFuture::new(1000).await;
        examples::levy().compile().execute(&context, viewport);
    });
    Ok(())
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

